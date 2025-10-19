use crate::ecs::{
    Position, Player, RealityLayer, CombatStats, GameEvent,
};
use crate::ecs::resources::{Resources, RunMode, UiMode};
use crate::domain_types::HitPoints;
use crate::world::{TimeCosts, generate_terrain, place_settlements, generate_settlement_map, place_pois, TravelEventChance, TravelEventGenerator};
use crate::systems::inventory::get_items_at_position;
use hecs::World;

/// Handle movement in both local and overmap mode
pub fn handle_movement(world: &mut World, resources: &mut Resources, dx: i32, dy: i32) {
    if matches!(resources.ui.ui_mode, UiMode::Overmap) {
        try_move_overmap(world, resources, dx, dy);
    } else {
        try_move_player(world, resources, dx, dy);
    }
}

/// Attempt to move player in local mode
pub fn try_move_player(world: &mut World, resources: &mut Resources, dx: i32, dy: i32) {
    let player_move = {
        let mut result = None;
        for (entity, (pos, _)) in world.query::<(&Position, &Player)>().iter() {
            result = Some((entity, pos.x + dx, pos.y + dy));
            break;
        }
        result
    };

    if let Some((entity, dest_x, dest_y)) = player_move {
        // Send Move event instead of adding component
        resources.events.send(GameEvent::Move { entity, dest_x, dest_y });
        resources.sim.mode = RunMode::PlayerTurn;
    }
}

/// Attempt to move player in overmap mode
pub fn try_move_overmap(world: &mut World, resources: &mut Resources, dx: i32, dy: i32) {
    let (current_x, current_y) = resources.world.player_overmap_pos;
    let new_x = current_x + dx;
    let new_y = current_y + dy;

    // Check if destination is walkable
    if !resources.world.overmap.is_walkable(new_x, new_y) {
        if let Some(tile) = resources.world.overmap.get_tile(new_x, new_y) {
            let terrain_name = tile.terrain.name();
            resources
                .ui.log
                .add(format!("You cannot travel through {}.", terrain_name));
        }
        return;
    }

    // Move to new tile
    resources.world.player_overmap_pos = (new_x, new_y);

    // Mark tile as visited and discover nearby tiles
    resources.world.overmap.visit_tile(new_x, new_y);
    for dy in -2..=2 {
        for dx in -2..=2 {
            resources.world.overmap.discover_tile(new_x + dx, new_y + dy);
        }
    }

    // Calculate time cost based on terrain and weather
    if let Some(tile) = resources.world.overmap.get_tile(new_x, new_y) {
        // Update weather based on current time
        let current_time = resources.sim.world_time.total_minutes();
        let season = resources.sim.world_time.season();
        resources.sim.weather.update(current_time, season, tile.terrain, &mut resources.sim.rng);

        // Calculate travel time with terrain and weather modifiers
        let base_time = TimeCosts::MOVE_OVERMAP; // 60 minutes
        let terrain_speed = tile.terrain.travel_speed();
        let weather_speed = resources.sim.weather.current_weather.travel_speed_modifier();
        let combined_speed = terrain_speed * weather_speed;
        let actual_time = (base_time as f32 / combined_speed) as i32;

        resources.sim.world_time.advance_minutes(actual_time);

        // Apply weather HP effects
        let weather_hp_damage = resources.sim.weather.current_weather.hp_per_hour();
        if weather_hp_damage != 0 {
            let hours_traveled = actual_time as f32 / 60.0;
            let total_hp_change = (weather_hp_damage as f32 * hours_traveled) as i32;

            if let Some(player_entity) = resources.player.player_entity {
                if let Ok(mut stats) = world.get::<&mut CombatStats>(player_entity) {
                    // Apply HP change (can be positive or negative)
                    if total_hp_change >= 0 {
                        let new_hp = stats.hp.saturating_add(total_hp_change);
                        stats.hp = HitPoints::new(new_hp.value().min(stats.max_hp.value()));
                    } else {
                        stats.hp = stats.hp.saturating_sub(-total_hp_change);
                    }

                    if stats.hp.is_dead() {
                        resources.ui.log.add("You have perished from exposure to the elements!");
                        resources.sim.mode = RunMode::GameOver;
                        return;
                    }

                    if total_hp_change < 0 {
                        resources.ui.log.add(format!(
                            "The harsh weather damages you. ({} HP)",
                            total_hp_change
                        ));
                    }
                }
            }
        }

        // Log the movement with weather
        let terrain_name = tile.terrain.name();
        let time_str = resources.sim.world_time.time_string();
        let weather_name = resources.sim.weather.current_weather.name();
        resources.ui.log.add(format!(
            "You travel through {}. {} ({})",
            terrain_name, weather_name, time_str
        ));

        // Roll for travel event
        let mut event_chance = TravelEventChance::new();

        // Calculate danger based on context
        let time_of_day = resources.sim.world_time.time_of_day();
        let distance_to_settlement = resources.world.settlements
            .iter()
            .map(|s| {
                let (sx, sy) = s.position;
                (new_x - sx).abs() + (new_y - sy).abs()
            })
            .min()
            .unwrap_or(999);

        event_chance.calculate_danger(tile.terrain, time_of_day, distance_to_settlement);

        // Check if event triggers
        if event_chance.should_trigger_event(&mut resources.sim.rng) {
            let generator = TravelEventGenerator::new(resources.sim.seed);
            let event = generator.generate_event(
                &mut resources.sim.rng,
                tile.terrain,
                time_of_day,
                event_chance.danger_multiplier,
            );

            // Log the event
            resources.ui.log.add(format!("Event: {}", event.message));

            // Apply HP changes
            if event.hp_change != 0 {
                if let Some(player_entity) = resources.player.player_entity {
                    if let Ok(mut stats) = world.get::<&mut CombatStats>(player_entity) {
                        // Apply HP change (can be positive or negative)
                        if event.hp_change >= 0 {
                            let new_hp = stats.hp.saturating_add(event.hp_change);
                            stats.hp = HitPoints::new(new_hp.value().min(stats.max_hp.value()));
                        } else {
                            stats.hp = stats.hp.saturating_sub(-event.hp_change);
                        }

                        if stats.hp.is_dead() {
                            resources.ui.log.add("You have died from your wounds!");
                            resources.sim.mode = RunMode::GameOver;
                        }
                    }
                }
            }

            // Apply additional time cost
            if event.time_cost > 0 {
                resources.sim.world_time.advance_minutes(event.time_cost);
            }

            // Gold changes would go here if we had a gold system
            if event.gold_change != 0 {
                resources.ui.log.add(format!("(Gold: {:+})", event.gold_change));
            }
        }
    }
}

/// Handle Tab key - toggle between local and overmap mode
pub(super) fn handle_tab_key(world: &mut World, resources: &mut Resources) {
    let entering_overmap = matches!(resources.ui.ui_mode, UiMode::InGame);

    if entering_overmap {
        resources.ui.ui_mode = UiMode::Overmap;
        // Entering overmap mode - clear current location
        if let Some(loc_id) = resources.world.current_location {
            if let Some(settlement) = resources.world.settlements.iter().find(|s| s.id == loc_id) {
                resources.ui.log.add(format!("You leave {}.", settlement.name));
            }
            resources.world.current_location = None;
        }

        // Generate terrain and settlements if not done yet
        if !resources.world.overmap.tiles.iter().any(|t| t.discovered) {
            generate_terrain(&mut resources.world.overmap);
            resources.world.settlements = place_settlements(&mut resources.world.overmap);

            // Generate POIs after settlements
            let settlement_positions: Vec<(i32, i32)> = resources.world.settlements
                .iter()
                .map(|s| s.position)
                .collect();
            resources.world.pois = place_pois(&mut resources.world.overmap, &settlement_positions);

            // Log settlement and POI counts
            let city_count = resources.world.settlements.iter().filter(|s| s.settlement_type == crate::world::SettlementType::City).count();
            let town_count = resources.world.settlements.iter().filter(|s| s.settlement_type == crate::world::SettlementType::Town).count();
            let village_count = resources.world.settlements.iter().filter(|s| s.settlement_type == crate::world::SettlementType::Village).count();
            resources.ui.log.add(format!(
                "Generated world with {} cities, {} towns, and {} villages.",
                city_count, town_count, village_count
            ));

            // Log POI counts by type
            let dungeon_count = resources.world.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Dungeon).count();
            let cave_count = resources.world.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Cave).count();
            let ruin_count = resources.world.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Ruin).count();
            let shrine_count = resources.world.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Shrine).count();
            let tomb_count = resources.world.pois.iter().filter(|p| p.poi_type == crate::world::POIType::TombCrypt).count();
            let tower_count = resources.world.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Tower).count();
            resources.ui.log.add(format!(
                "Found {} dungeons, {} caves, {} ruins, {} shrines, {} tombs, and {} towers.",
                dungeon_count, cave_count, ruin_count, shrine_count, tomb_count, tower_count
            ));

            // Generate road network connecting settlements
            resources.world.roads = crate::world::generate_roads(&resources.world.overmap, &resources.world.settlements);
            let king_road_count = resources.world.roads.iter().filter(|r| r.road_type == crate::world::RoadType::KingRoad).count();
            let trade_route_count = resources.world.roads.iter().filter(|r| r.road_type == crate::world::RoadType::TradeRoute).count();
            let path_count = resources.world.roads.iter().filter(|r| r.road_type == crate::world::RoadType::Path).count();
            resources.ui.log.add(format!(
                "Built {} king roads, {} trade routes, and {} paths.",
                king_road_count, trade_route_count, path_count
            ));

            // Discover starting area
            let (px, py) = resources.world.player_overmap_pos;
            for dy in -3..=3 {
                for dx in -3..=3 {
                    resources.world.overmap.discover_tile(px + dx, py + dy);
                }
            }
        }
        resources.ui.log.add("You survey the surrounding area...");
    } else {
        resources.ui.ui_mode = UiMode::InGame;
        resources.ui.log.add("You return your focus to the immediate area.");
    }
}

/// Try to enter a location (settlement, dungeon, etc.) from overmap
pub(super) fn try_enter_location(world: &mut World, resources: &mut Resources) {
    let (x, y) = resources.world.player_overmap_pos;

    // Check if player is on a settlement
    if let Some(settlement) = resources.world.settlements.iter().find(|s| s.position == (x, y)) {
        let settlement_id = settlement.id;
        let settlement_name = settlement.name.clone();
        let settlement_type = settlement.settlement_type.name();

        resources.ui.log.add(format!(
            "You enter {} ({}).",
            settlement_name,
            settlement_type
        ));
        resources.ui.log.add("Press Tab to return to the world map.");

        // Generate settlement map if not already generated
        if !resources.world.settlement_maps.contains_key(&settlement_id) {
            let map = generate_settlement_map(settlement, resources.sim.seed);
            resources.world.settlement_maps.insert(settlement_id, map);
        }

        // Get the settlement map and find entrance position
        if let Some(settlement_map) = resources.world.settlement_maps.get(&settlement_id) {
            // Find entrance (gap in bottom wall)
            let entrance_x = settlement_map.width / 2;
            let entrance_y = settlement_map.height - 2; // Just inside the entrance

            // Move player to entrance
            if let Some(player_entity) = resources.player.player_entity {
                if let Ok(mut pos) = world.get::<&mut Position>(player_entity) {
                    pos.x = entrance_x;
                    pos.y = entrance_y;
                    pos.layer = RealityLayer::Normal;
                }
            }

            // Copy settlement map to active map
            resources.world.maps.normal = settlement_map.clone();
            resources.ui.camera.center_on(entrance_x, entrance_y);
        }

        // Set current location and exit overmap mode
        resources.world.current_location = Some(settlement_id);
        resources.ui.ui_mode = UiMode::InGame;
        return;
    }

    // Check if on other location types (dungeon, special location)
    if let Some(tile) = resources.world.overmap.get_tile(x, y) {
        match tile.terrain {
            crate::world::TerrainType::Dungeon => {
                resources.ui.log.add("You enter the dungeon entrance...");
                resources.ui.log.add("Press Tab to return to the world map.");
                resources.ui.ui_mode = UiMode::InGame;
            }
            crate::world::TerrainType::SpecialLocation => {
                resources.ui.log.add("You enter the mysterious location...");
                resources.ui.log.add("Press Tab to return to the world map.");
                resources.ui.ui_mode = UiMode::InGame;
            }
            _ => {
                resources.ui.log.add("There is nothing to enter here.");
            }
        }
    }
}

/// Try to rest and recover HP
pub(super) fn try_rest(world: &mut World, resources: &mut Resources) {
    // Get player entity
    if let Some(player_entity) = resources.player.player_entity {
        // Extract HP values in a separate scope to drop the borrow
        let (current_hp, max_hp) = {
            if let Ok(stats) = world.get::<&CombatStats>(player_entity) {
                (stats.hp, stats.max_hp)
            } else {
                return; // No combat stats component
            }
        };

        // Check if already at full HP
        if current_hp.value() >= max_hp.value() {
            resources.ui.log.add("You are already at full health.");
            return;
        }

        // Check if in safe location (settlement)
        let is_safe = resources.world.current_location.is_some();

        if !is_safe {
            resources.ui.log.add("You cannot rest here - it's not safe! Find a settlement.");
            return;
        }

        // Perform rest
        let rest_hours = 8; // Long rest
        let hp_restored = (max_hp.value() as f32 * 0.5).ceil() as i32; // Restore 50% HP

        // Now safely do mutable borrow (immutable borrow has been dropped)
        if let Ok(mut stats) = world.get::<&mut CombatStats>(player_entity) {
            let new_hp = stats.hp.saturating_add(hp_restored);
            stats.hp = HitPoints::new(new_hp.value().min(stats.max_hp.value()));

            // Advance time
            resources.sim.world_time.advance_hours(rest_hours);

            let time_str = resources.sim.world_time.time_string();
            let season = resources.sim.world_time.season().name();
            let tod = resources.sim.world_time.time_of_day().name();

            resources.ui.log.add(format!(
                "You rest for {} hours and recover {} HP.",
                rest_hours, hp_restored
            ));
            resources.ui.log.add(format!(
                "You wake up feeling refreshed. {} - {} ({}, Day {})",
                time_str, tod, season, resources.sim.world_time.day_of_year
            ));
        }
    }
}

/// Try to pick up items at player's location
pub(super) fn try_pickup_items(world: &mut World, resources: &mut Resources) {
    // Get player position and entity
    if let Some(player_entity) = resources.player.player_entity {
        let player_pos = if let Ok(pos) = world.get::<&Position>(player_entity) {
            (pos.x, pos.y)
        } else {
            return;
        };

        // Find items at player position
        let items = get_items_at_position(world, player_pos.0, player_pos.1);

        if items.is_empty() {
            resources.ui.log.add("There is nothing here to pick up.");
            return;
        }

        // Pick up all items at this location
        for item in items {
            resources.events.send(GameEvent::PickupItem { entity: player_entity, item });
        }

        resources.sim.mode = RunMode::PlayerTurn;
    }
}

/// Try to open a nearby door
pub(super) fn try_open_door(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player.player_entity {
        // Get player position
        let player_pos = if let Ok(pos) = world.get::<&Position>(player_entity) {
            (pos.x, pos.y, pos.layer)
        } else {
            return;
        };

        // Get the active map
        let map = if let Some(location_id) = resources.world.current_location {
            resources.world.settlement_maps.get_mut(&location_id)
        } else {
            Some(resources.world.maps.active_map_mut())
        };

        if let Some(map) = map {
            // Check all adjacent tiles for closed doors
            let directions = [
                (-1, 0), (1, 0), (0, -1), (0, 1), // Cardinal
                (-1, -1), (1, -1), (-1, 1), (1, 1) // Diagonal
            ];

            let mut doors_found = Vec::new();
            for (dx, dy) in directions.iter() {
                let check_x = player_pos.0 + dx;
                let check_y = player_pos.1 + dy;

                if map.in_bounds(check_x, check_y) {
                    let idx = map.xy_idx(check_x, check_y);
                    if map.tiles[idx] == crate::map::Tile::ClosedDoor {
                        doors_found.push((check_x, check_y, idx));
                    }
                }
            }

            if doors_found.is_empty() {
                resources.ui.log.add("There is no door to open nearby.");
            } else if doors_found.len() == 1 {
                // Open the single door
                let (x, y, idx) = doors_found[0];
                map.tiles[idx] = crate::map::Tile::OpenDoor;
                resources.ui.log.add(format!("You open the door at ({}, {}).", x, y));
                resources.sim.mode = RunMode::PlayerTurn; // Opening a door takes a turn
            } else {
                // Multiple doors - open the first one for now
                // TODO: Let player choose which door to open
                let (x, y, idx) = doors_found[0];
                map.tiles[idx] = crate::map::Tile::OpenDoor;
                resources.ui.log.add(format!("You open the door at ({}, {}).", x, y));
                resources.sim.mode = RunMode::PlayerTurn;
            }
        }
    }
}

/// Try to close a nearby door
pub(super) fn try_close_door(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player.player_entity {
        // Get player position
        let player_pos = if let Ok(pos) = world.get::<&Position>(player_entity) {
            (pos.x, pos.y, pos.layer)
        } else {
            return;
        };

        // Get the active map
        let map = if let Some(location_id) = resources.world.current_location {
            resources.world.settlement_maps.get_mut(&location_id)
        } else {
            Some(resources.world.maps.active_map_mut())
        };

        if let Some(map) = map {
            // Check all adjacent tiles for open doors
            let directions = [
                (-1, 0), (1, 0), (0, -1), (0, 1), // Cardinal
                (-1, -1), (1, -1), (-1, 1), (1, 1) // Diagonal
            ];

            let mut doors_found = Vec::new();
            for (dx, dy) in directions.iter() {
                let check_x = player_pos.0 + dx;
                let check_y = player_pos.1 + dy;

                if map.in_bounds(check_x, check_y) {
                    let idx = map.xy_idx(check_x, check_y);
                    if map.tiles[idx] == crate::map::Tile::OpenDoor {
                        // Check if any entity is blocking the door
                        let mut blocked = false;
                        for (_, pos) in world.query::<&Position>().iter() {
                            if pos.x == check_x && pos.y == check_y && pos.layer == player_pos.2 {
                                blocked = true;
                                break;
                            }
                        }

                        if !blocked {
                            doors_found.push((check_x, check_y, idx));
                        }
                    }
                }
            }

            if doors_found.is_empty() {
                resources.ui.log.add("There is no door to close nearby.");
            } else if doors_found.len() == 1 {
                // Close the single door
                let (x, y, idx) = doors_found[0];
                map.tiles[idx] = crate::map::Tile::ClosedDoor;
                resources.ui.log.add(format!("You close the door at ({}, {}).", x, y));
                resources.sim.mode = RunMode::PlayerTurn; // Closing a door takes a turn
            } else {
                // Multiple doors - close the first one for now
                // TODO: Let player choose which door to close
                let (x, y, idx) = doors_found[0];
                map.tiles[idx] = crate::map::Tile::ClosedDoor;
                resources.ui.log.add(format!("You close the door at ({}, {}).", x, y));
                resources.sim.mode = RunMode::PlayerTurn;
            }
        }
    }
}

/// Try to use stairs (ascend or descend)
pub(super) fn try_use_stairs(world: &mut World, resources: &mut Resources, ascending: bool) {
    if let Some(player_entity) = resources.player.player_entity {
        // Get player position
        let player_pos = if let Ok(pos) = world.get::<&Position>(player_entity) {
            (pos.x, pos.y, pos.layer)
        } else {
            return;
        };

        // Get the map at current depth
        let map = if resources.world.current_depth.is_dungeon() {
            // In dungeon
            resources.world.dungeon_levels.get(&resources.world.current_depth)
        } else if let Some(location_id) = resources.world.current_location {
            // In settlement
            resources.world.settlement_maps.get(&location_id)
        } else {
            // On wilderness map
            Some(resources.world.maps.active_map())
        };

        if let Some(map) = map {
            // Check if standing on appropriate stairs
            let idx = map.xy_idx(player_pos.0, player_pos.1);
            let tile = map.tiles[idx];

            if ascending {
                // Trying to ascend
                if tile == crate::map::Tile::StairsUp {
                    if let Some(new_depth) = resources.world.current_depth.ascend() {
                        // Ascending back up
                        resources.world.current_depth = new_depth;
                        resources.ui.log.add(format!("You climb the stairs upward to depth {}.", resources.world.current_depth));

                        // Find stairs down on the level above to place player
                        // For now, just place player at a safe starting position
                        // TODO: Remember stair positions between levels

                        resources.sim.mode = RunMode::PlayerTurn;
                    } else {
                        resources.ui.log.add("You are already on the surface level.");
                    }
                } else {
                    resources.ui.log.add("You need to stand on stairs going up (<) to ascend.");
                }
            } else {
                // Trying to descend
                if tile == crate::map::Tile::StairsDown {
                    // Going deeper
                    resources.world.current_depth = resources.world.current_depth.descend();

                    // Generate new level if it doesn't exist
                    if !resources.world.dungeon_levels.contains_key(&resources.world.current_depth) {
                        let new_level = crate::map::generate_dungeon_level(
                            resources.world.maps.active_map().width,
                            resources.world.maps.active_map().height,
                            &mut resources.sim.rng
                        );
                        resources.world.dungeon_levels.insert(resources.world.current_depth, new_level);
                        resources.ui.log.add(format!("You descend into the depths... (Depth {})", resources.world.current_depth));
                    } else {
                        resources.ui.log.add(format!("You climb down the stairs to depth {}.", resources.world.current_depth));
                    }

                    // Find stairs up on new level to place player
                    // For now, just search for the first StairsUp
                    if let Some(new_map) = resources.world.dungeon_levels.get(&resources.world.current_depth) {
                        for y in 0..new_map.height {
                            for x in 0..new_map.width {
                                let idx = new_map.xy_idx(x, y);
                                if new_map.tiles[idx] == crate::map::Tile::StairsUp {
                                    // Place player at stairs up
                                    if let Ok(mut pos) = world.get::<&mut Position>(player_entity) {
                                        pos.x = x;
                                        pos.y = y;
                                    }
                                    resources.sim.mode = RunMode::PlayerTurn;
                                    return;
                                }
                            }
                        }
                    }

                    resources.sim.mode = RunMode::PlayerTurn;
                } else {
                    resources.ui.log.add("You need to stand on stairs going down (>) to descend.");
                }
            }
        }
    }
}
