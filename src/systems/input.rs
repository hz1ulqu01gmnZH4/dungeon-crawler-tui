use crate::ecs::{
    Position, Player, WantsToMove, RealityLayer, CombatStats, WantsToPickupItem,
    WantsToDropItem, WantsToEquipItem, WantsToUnequipItem, WantsToUseItem,
    Inventory, Equipable, Consumable, EquipSlot, ItemData,
};
use crate::ecs::resources::{Resources, RunMode};
use crate::world::{TimeCosts, generate_terrain, place_settlements, generate_settlement_map, place_pois, TravelEventChance, TravelEventGenerator};
use crate::save::{quick_save, quick_load};
use crate::systems::inventory::{get_items_at_position, find_equipable_for_slot};
use crossterm::event::{self, Event, KeyCode};
use hecs::World;
use std::time::Duration;

pub fn handle_input(world: &mut World, resources: &mut Resources) -> anyhow::Result<bool> {
    if resources.mode != RunMode::AwaitingInput {
        return Ok(true);
    }

    if !event::poll(Duration::from_millis(100))? {
        return Ok(true);
    }

    if let Event::Key(key) = event::read()? {
        return Ok(handle_key(key.code, world, resources));
    }

    Ok(true)
}

/// Handle a specific key press (useful for testing and direct key injection)
pub fn handle_key(key_code: KeyCode, world: &mut World, resources: &mut Resources) -> bool {
    // Handle main menu input separately
    if resources.in_main_menu {
        return handle_main_menu_input(key_code, world, resources);
    }

    // Handle character screen input separately
    if resources.in_character_screen {
        return handle_character_screen_input(key_code, resources);
    }

    // Handle examine mode input separately
    if resources.in_examine_mode {
        return handle_examine_input(key_code, world, resources);
    }

    // Handle inventory UI input separately
    if resources.in_inventory_mode {
        return handle_inventory_input(key_code, world, resources);
    }

    match key_code {
            KeyCode::Char('q') => return false,
            KeyCode::Char('S') => {
                // Save game (capital S to avoid conflict with future 's' for smash)
                match quick_save(world, resources, resources.seed) {
                    Ok(_) => resources.log.add("Game saved to savegame.json"),
                    Err(e) => resources.log.add(format!("Failed to save: {}", e)),
                }
            }
            KeyCode::Char('L') => {
                // Load game (capital L to avoid conflict with 'l' for movement)
                match quick_load(world, resources) {
                    Ok(seed) => {
                        resources.seed = seed;
                        resources.log.add("Game loaded from savegame.json");
                        // Reset camera to player position
                        if let Some(player_entity) = resources.player_entity {
                            if let Ok(pos) = world.get::<&Position>(player_entity) {
                                resources.camera.center_on(pos.x, pos.y);
                            }
                        }
                    }
                    Err(e) => resources.log.add(format!("Failed to load: {}", e)),
                }
            }
            KeyCode::Tab => {
                // Toggle between local and overmap mode
                resources.in_overmap_mode = !resources.in_overmap_mode;

                if resources.in_overmap_mode {
                    // Entering overmap mode - clear current location
                    if let Some(loc_id) = resources.current_location {
                        if let Some(settlement) = resources.settlements.iter().find(|s| s.id == loc_id) {
                            resources.log.add(format!("You leave {}.", settlement.name));
                        }
                        resources.current_location = None;
                    }

                    // Generate terrain and settlements if not done yet
                    if !resources.overmap.tiles.iter().any(|t| t.discovered) {
                        generate_terrain(&mut resources.overmap);
                        resources.settlements = place_settlements(&mut resources.overmap);

                        // Generate POIs after settlements
                        let settlement_positions: Vec<(i32, i32)> = resources.settlements
                            .iter()
                            .map(|s| s.position)
                            .collect();
                        resources.pois = place_pois(&mut resources.overmap, &settlement_positions);

                        // Log settlement and POI counts
                        let city_count = resources.settlements.iter().filter(|s| s.settlement_type == crate::world::SettlementType::City).count();
                        let town_count = resources.settlements.iter().filter(|s| s.settlement_type == crate::world::SettlementType::Town).count();
                        let village_count = resources.settlements.iter().filter(|s| s.settlement_type == crate::world::SettlementType::Village).count();
                        resources.log.add(format!(
                            "Generated world with {} cities, {} towns, and {} villages.",
                            city_count, town_count, village_count
                        ));

                        // Log POI counts by type
                        let dungeon_count = resources.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Dungeon).count();
                        let cave_count = resources.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Cave).count();
                        let ruin_count = resources.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Ruin).count();
                        let shrine_count = resources.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Shrine).count();
                        let tomb_count = resources.pois.iter().filter(|p| p.poi_type == crate::world::POIType::TombCrypt).count();
                        let tower_count = resources.pois.iter().filter(|p| p.poi_type == crate::world::POIType::Tower).count();
                        resources.log.add(format!(
                            "Found {} dungeons, {} caves, {} ruins, {} shrines, {} tombs, and {} towers.",
                            dungeon_count, cave_count, ruin_count, shrine_count, tomb_count, tower_count
                        ));

                        // Generate road network connecting settlements
                        resources.roads = crate::world::generate_roads(&resources.overmap, &resources.settlements);
                        let king_road_count = resources.roads.iter().filter(|r| r.road_type == crate::world::RoadType::KingRoad).count();
                        let trade_route_count = resources.roads.iter().filter(|r| r.road_type == crate::world::RoadType::TradeRoute).count();
                        let path_count = resources.roads.iter().filter(|r| r.road_type == crate::world::RoadType::Path).count();
                        resources.log.add(format!(
                            "Built {} king roads, {} trade routes, and {} paths.",
                            king_road_count, trade_route_count, path_count
                        ));

                        // Discover starting area
                        let (px, py) = resources.player_overmap_pos;
                        for dy in -3..=3 {
                            for dx in -3..=3 {
                                resources.overmap.discover_tile(px + dx, py + dy);
                            }
                        }
                    }
                    resources.log.add("You survey the surrounding area...");
                } else {
                    resources.log.add("You return your focus to the immediate area.");
                }
            }
            KeyCode::Char('h') | KeyCode::Left => handle_movement(world, resources, -1, 0),
            KeyCode::Char('l') | KeyCode::Right => handle_movement(world, resources, 1, 0),
            KeyCode::Char('k') | KeyCode::Up => handle_movement(world, resources, 0, -1),
            KeyCode::Char('j') | KeyCode::Down => handle_movement(world, resources, 0, 1),
            KeyCode::Char('y') => handle_movement(world, resources, -1, -1),
            KeyCode::Char('u') => handle_movement(world, resources, 1, -1),
            KeyCode::Char('b') => handle_movement(world, resources, -1, 1),
            KeyCode::Char('n') => handle_movement(world, resources, 1, 1),
            KeyCode::Char('.') => {
                // Wait/skip turn
                if !resources.in_overmap_mode {
                    resources.mode = RunMode::PlayerTurn;
                }
            }
            KeyCode::Enter => {
                // Enter location (settlement, dungeon, etc.)
                if resources.in_overmap_mode {
                    try_enter_location(world, resources);
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                // Rest/camp
                if !resources.in_overmap_mode {
                    try_rest(world, resources);
                }
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                // Pickup items
                if !resources.in_overmap_mode {
                    try_pickup_items(world, resources);
                }
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                // Toggle inventory
                if !resources.in_overmap_mode {
                    resources.in_inventory_mode = !resources.in_inventory_mode;
                    resources.inventory_selection = 0;
                }
            }
            KeyCode::Char('@') => {
                // Toggle character screen
                if !resources.in_overmap_mode {
                    resources.in_character_screen = !resources.in_character_screen;
                }
            }
            KeyCode::Char('x') => {
                // Toggle examine mode
                if !resources.in_overmap_mode {
                    if !resources.in_examine_mode {
                        // Entering examine mode - set cursor to player position
                        if let Some(player_entity) = resources.player_entity {
                            if let Ok(pos) = world.get::<&Position>(player_entity) {
                                resources.examine_cursor = (pos.x, pos.y);
                                resources.in_examine_mode = true;
                                resources.log.add("Examine mode: Move cursor with hjkl/arrows. Press 'x' or ESC to exit.");
                            }
                        }
                    } else {
                        // Exiting examine mode
                        resources.in_examine_mode = false;
                    }
                }
            }
            KeyCode::Char('w') => {
                // Wield weapon (quick equip to main hand)
                if !resources.in_overmap_mode {
                    quick_wield_weapon(world, resources);
                }
            }
            KeyCode::Char('W') => {
                // Wear armor (quick equip to body)
                if !resources.in_overmap_mode {
                    quick_wear_armor(world, resources);
                }
            }
            KeyCode::Char('T') => {
                // Take off (unequip)
                if !resources.in_overmap_mode {
                    quick_take_off(world, resources);
                }
            }
            KeyCode::Char('o') => {
                // Open door
                if !resources.in_overmap_mode {
                    try_open_door(world, resources);
                }
            }
            KeyCode::Char('c') => {
                // Close door
                if !resources.in_overmap_mode {
                    try_close_door(world, resources);
                }
            }
            KeyCode::Char('<') => {
                // Ascend stairs
                if !resources.in_overmap_mode {
                    try_use_stairs(world, resources, true);
                }
            }
            KeyCode::Char('>') => {
                // Descend stairs
                if !resources.in_overmap_mode {
                    try_use_stairs(world, resources, false);
                }
            }
            _ => {}
        }

    true
}

fn handle_movement(world: &mut World, resources: &mut Resources, dx: i32, dy: i32) {
    if resources.in_overmap_mode {
        try_move_overmap(world, resources, dx, dy);
    } else {
        try_move_player(world, resources, dx, dy);
    }
}

fn try_move_player(world: &mut World, resources: &mut Resources, dx: i32, dy: i32) {
    let player_move = {
        let mut result = None;
        for (entity, (pos, _)) in world.query::<(&Position, &Player)>().iter() {
            result = Some((entity, pos.x + dx, pos.y + dy));
            break;
        }
        result
    };

    if let Some((entity, dest_x, dest_y)) = player_move {
        let _ = world.insert_one(entity, WantsToMove::new(dest_x, dest_y));
        resources.mode = RunMode::PlayerTurn;
    }
}

fn try_move_overmap(world: &mut World, resources: &mut Resources, dx: i32, dy: i32) {
    let (current_x, current_y) = resources.player_overmap_pos;
    let new_x = current_x + dx;
    let new_y = current_y + dy;

    // Check if destination is walkable
    if !resources.overmap.is_walkable(new_x, new_y) {
        if let Some(tile) = resources.overmap.get_tile(new_x, new_y) {
            let terrain_name = tile.terrain.name();
            resources
                .log
                .add(format!("You cannot travel through {}.", terrain_name));
        }
        return;
    }

    // Move to new tile
    resources.player_overmap_pos = (new_x, new_y);

    // Mark tile as visited and discover nearby tiles
    resources.overmap.visit_tile(new_x, new_y);
    for dy in -2..=2 {
        for dx in -2..=2 {
            resources.overmap.discover_tile(new_x + dx, new_y + dy);
        }
    }

    // Calculate time cost based on terrain and weather
    if let Some(tile) = resources.overmap.get_tile(new_x, new_y) {
        // Update weather based on current time
        let current_time = resources.world_time.total_minutes();
        let season = resources.world_time.season();
        resources.weather.update(current_time, season, tile.terrain, &mut resources.rng);

        // Calculate travel time with terrain and weather modifiers
        let base_time = TimeCosts::MOVE_OVERMAP; // 60 minutes
        let terrain_speed = tile.terrain.travel_speed();
        let weather_speed = resources.weather.current_weather.travel_speed_modifier();
        let combined_speed = terrain_speed * weather_speed;
        let actual_time = (base_time as f32 / combined_speed) as i32;

        resources.world_time.advance_minutes(actual_time);

        // Apply weather HP effects
        let weather_hp_damage = resources.weather.current_weather.hp_per_hour();
        if weather_hp_damage != 0 {
            let hours_traveled = actual_time as f32 / 60.0;
            let total_hp_change = (weather_hp_damage as f32 * hours_traveled) as i32;

            if let Some(player_entity) = resources.player_entity {
                if let Ok(mut stats) = world.get::<&mut CombatStats>(player_entity) {
                    stats.hp = (stats.hp + total_hp_change).clamp(0, stats.max_hp);

                    if stats.hp == 0 {
                        resources.log.add("You have perished from exposure to the elements!");
                        resources.mode = RunMode::GameOver;
                        return;
                    }

                    if total_hp_change < 0 {
                        resources.log.add(format!(
                            "The harsh weather damages you. ({} HP)",
                            total_hp_change
                        ));
                    }
                }
            }
        }

        // Log the movement with weather
        let terrain_name = tile.terrain.name();
        let time_str = resources.world_time.time_string();
        let weather_name = resources.weather.current_weather.name();
        resources.log.add(format!(
            "You travel through {}. {} ({})",
            terrain_name, weather_name, time_str
        ));

        // Roll for travel event
        let mut event_chance = TravelEventChance::new();

        // Calculate danger based on context
        let time_of_day = resources.world_time.time_of_day();
        let distance_to_settlement = resources.settlements
            .iter()
            .map(|s| {
                let (sx, sy) = s.position;
                (new_x - sx).abs() + (new_y - sy).abs()
            })
            .min()
            .unwrap_or(999);

        event_chance.calculate_danger(tile.terrain, time_of_day, distance_to_settlement);

        // Check if event triggers
        if event_chance.should_trigger_event(&mut resources.rng) {
            let generator = TravelEventGenerator::new(resources.seed);
            let event = generator.generate_event(
                &mut resources.rng,
                tile.terrain,
                time_of_day,
                event_chance.danger_multiplier,
            );

            // Log the event
            resources.log.add(format!("Event: {}", event.message));

            // Apply HP changes
            if event.hp_change != 0 {
                if let Some(player_entity) = resources.player_entity {
                    if let Ok(mut stats) = world.get::<&mut CombatStats>(player_entity) {
                        stats.hp = (stats.hp + event.hp_change).clamp(0, stats.max_hp);

                        if stats.hp == 0 {
                            resources.log.add("You have died from your wounds!");
                            resources.mode = RunMode::GameOver;
                        }
                    }
                }
            }

            // Apply additional time cost
            if event.time_cost > 0 {
                resources.world_time.advance_minutes(event.time_cost);
            }

            // Gold changes would go here if we had a gold system
            if event.gold_change != 0 {
                resources.log.add(format!("(Gold: {:+})", event.gold_change));
            }
        }
    }
}

fn try_enter_location(world: &mut World, resources: &mut Resources) {
    let (x, y) = resources.player_overmap_pos;

    // Check if player is on a settlement
    if let Some(settlement) = resources.settlements.iter().find(|s| s.position == (x, y)) {
        let settlement_id = settlement.id;
        let settlement_name = settlement.name.clone();
        let settlement_type = settlement.settlement_type.name();

        resources.log.add(format!(
            "You enter {} ({}).",
            settlement_name,
            settlement_type
        ));
        resources.log.add("Press Tab to return to the world map.");

        // Generate settlement map if not already generated
        if !resources.settlement_maps.contains_key(&settlement_id) {
            let map = generate_settlement_map(settlement, resources.seed);
            resources.settlement_maps.insert(settlement_id, map);
        }

        // Get the settlement map and find entrance position
        if let Some(settlement_map) = resources.settlement_maps.get(&settlement_id) {
            // Find entrance (gap in bottom wall)
            let entrance_x = settlement_map.width / 2;
            let entrance_y = settlement_map.height - 2; // Just inside the entrance

            // Move player to entrance
            if let Some(player_entity) = resources.player_entity {
                if let Ok(mut pos) = world.get::<&mut Position>(player_entity) {
                    pos.x = entrance_x;
                    pos.y = entrance_y;
                    pos.layer = RealityLayer::Normal;
                }
            }

            // Copy settlement map to active map
            resources.maps.normal = settlement_map.clone();
            resources.camera.center_on(entrance_x, entrance_y);
        }

        // Set current location and exit overmap mode
        resources.current_location = Some(settlement_id);
        resources.in_overmap_mode = false;
        return;
    }

    // Check if on other location types (dungeon, special location)
    if let Some(tile) = resources.overmap.get_tile(x, y) {
        match tile.terrain {
            crate::world::TerrainType::Dungeon => {
                resources.log.add("You enter the dungeon entrance...");
                resources.log.add("Press Tab to return to the world map.");
                resources.in_overmap_mode = false;
            }
            crate::world::TerrainType::SpecialLocation => {
                resources.log.add("You enter the mysterious location...");
                resources.log.add("Press Tab to return to the world map.");
                resources.in_overmap_mode = false;
            }
            _ => {
                resources.log.add("There is nothing to enter here.");
            }
        }
    }
}

fn try_rest(world: &mut World, resources: &mut Resources) {
    // Get player entity
    if let Some(player_entity) = resources.player_entity {
        // Extract HP values in a separate scope to drop the borrow
        let (current_hp, max_hp) = {
            if let Ok(stats) = world.get::<&CombatStats>(player_entity) {
                (stats.hp, stats.max_hp)
            } else {
                return; // No combat stats component
            }
        };

        // Check if already at full HP
        if current_hp >= max_hp {
            resources.log.add("You are already at full health.");
            return;
        }

        // Check if in safe location (settlement)
        let is_safe = resources.current_location.is_some();

        if !is_safe {
            resources.log.add("You cannot rest here - it's not safe! Find a settlement.");
            return;
        }

        // Perform rest
        let rest_hours = 8; // Long rest
        let hp_restored = (max_hp as f32 * 0.5).ceil() as i32; // Restore 50% HP

        // Now safely do mutable borrow (immutable borrow has been dropped)
        if let Ok(mut stats) = world.get::<&mut CombatStats>(player_entity) {
            stats.hp = (stats.hp + hp_restored).min(stats.max_hp);

            // Advance time
            resources.world_time.advance_hours(rest_hours);

            let time_str = resources.world_time.time_string();
            let season = resources.world_time.season().name();
            let tod = resources.world_time.time_of_day().name();

            resources.log.add(format!(
                "You rest for {} hours and recover {} HP.",
                rest_hours, hp_restored
            ));
            resources.log.add(format!(
                "You wake up feeling refreshed. {} - {} ({}, Day {})",
                time_str, tod, season, resources.world_time.day_of_year
            ));
        }
    }
}

fn try_pickup_items(world: &mut World, resources: &mut Resources) {
    // Get player position and entity
    if let Some(player_entity) = resources.player_entity {
        let player_pos = if let Ok(pos) = world.get::<&Position>(player_entity) {
            (pos.x, pos.y)
        } else {
            return;
        };

        // Find items at player position
        let items = get_items_at_position(world, player_pos.0, player_pos.1);

        if items.is_empty() {
            resources.log.add("There is nothing here to pick up.");
            return;
        }

        // Pick up all items at this location
        for item in items {
            let _ = world.insert_one(player_entity, WantsToPickupItem { item });
        }

        resources.mode = RunMode::PlayerTurn;
    }
}

/// Quick wield weapon - equip first unequipped weapon in inventory to main hand
fn quick_wield_weapon(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player_entity {
        let weapon_to_equip = {
            if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                find_equipable_for_slot(world, &*inventory, EquipSlot::MainHand)
            } else {
                None
            }
        };

        if let Some(weapon) = weapon_to_equip {
            let _ = world.insert_one(player_entity, WantsToEquipItem { item: weapon });
            resources.mode = RunMode::PlayerTurn;
        } else {
            // Check if already have something equipped
            let already_equipped = {
                if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                    inventory.get_equipped(EquipSlot::MainHand).is_some()
                } else {
                    false
                }
            };

            if already_equipped {
                resources.log.add("You already have a weapon equipped.");
            } else {
                resources.log.add("You have no weapon to wield.");
            }
        }
    }
}

/// Quick wear armor - equip first unequipped armor in inventory to body
fn quick_wear_armor(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player_entity {
        let armor_to_equip = {
            if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                find_equipable_for_slot(world, &*inventory, EquipSlot::Body)
            } else {
                None
            }
        };

        if let Some(armor) = armor_to_equip {
            let _ = world.insert_one(player_entity, WantsToEquipItem { item: armor });
            resources.mode = RunMode::PlayerTurn;
        } else {
            // Check if already have something equipped
            let already_equipped = {
                if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                    inventory.get_equipped(EquipSlot::Body).is_some()
                } else {
                    false
                }
            };

            if already_equipped {
                resources.log.add("You already have armor equipped.");
            } else {
                resources.log.add("You have no armor to wear.");
            }
        }
    }
}

/// Quick take off - unequip currently equipped item (prioritize: weapon > armor > other)
fn quick_take_off(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player_entity {
        // Find which slot to unequip (priority: MainHand > Body > OffHand > others)
        let slot_to_unequip = {
            if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                if inventory.get_equipped(EquipSlot::MainHand).is_some() {
                    Some(EquipSlot::MainHand)
                } else if inventory.get_equipped(EquipSlot::Body).is_some() {
                    Some(EquipSlot::Body)
                } else if inventory.get_equipped(EquipSlot::OffHand).is_some() {
                    Some(EquipSlot::OffHand)
                } else if inventory.get_equipped(EquipSlot::Head).is_some() {
                    Some(EquipSlot::Head)
                } else if inventory.get_equipped(EquipSlot::Legs).is_some() {
                    Some(EquipSlot::Legs)
                } else if inventory.get_equipped(EquipSlot::Feet).is_some() {
                    Some(EquipSlot::Feet)
                } else {
                    None
                }
            } else {
                None
            }
        };

        if let Some(slot) = slot_to_unequip {
            let _ = world.insert_one(player_entity, WantsToUnequipItem { slot });
            resources.mode = RunMode::PlayerTurn;
        } else {
            resources.log.add("You have nothing equipped to take off.");
        }
    }
}

fn handle_character_screen_input(key_code: KeyCode, resources: &mut Resources) -> bool {
    match key_code {
        // Close character screen
        KeyCode::Char('@') | KeyCode::Esc => {
            resources.in_character_screen = false;
        }
        // Future: navigation between tabs, etc.
        _ => {}
    }
    true
}

fn handle_examine_input(key_code: KeyCode, world: &mut World, resources: &mut Resources) -> bool {
    match key_code {
        // Close examine mode
        KeyCode::Char('x') | KeyCode::Esc => {
            resources.in_examine_mode = false;
        }
        // Move examine cursor
        KeyCode::Char('h') | KeyCode::Left => {
            resources.examine_cursor.0 -= 1;
        }
        KeyCode::Char('l') | KeyCode::Right => {
            resources.examine_cursor.0 += 1;
        }
        KeyCode::Char('k') | KeyCode::Up => {
            resources.examine_cursor.1 -= 1;
        }
        KeyCode::Char('j') | KeyCode::Down => {
            resources.examine_cursor.1 += 1;
        }
        KeyCode::Char('y') => {
            resources.examine_cursor.0 -= 1;
            resources.examine_cursor.1 -= 1;
        }
        KeyCode::Char('u') => {
            resources.examine_cursor.0 += 1;
            resources.examine_cursor.1 -= 1;
        }
        KeyCode::Char('b') => {
            resources.examine_cursor.0 -= 1;
            resources.examine_cursor.1 += 1;
        }
        KeyCode::Char('n') => {
            resources.examine_cursor.0 += 1;
            resources.examine_cursor.1 += 1;
        }
        _ => {}
    }
    true
}

fn handle_inventory_input(key_code: KeyCode, world: &mut World, resources: &mut Resources) -> bool {
    match key_code {
        // Close inventory
        KeyCode::Char('i') | KeyCode::Char('I') | KeyCode::Esc => {
            resources.in_inventory_mode = false;
        }
        // Navigate up
        KeyCode::Up | KeyCode::Char('k') => {
            if resources.inventory_selection > 0 {
                resources.inventory_selection -= 1;
            }
        }
        // Navigate down
        KeyCode::Down | KeyCode::Char('j') => {
            if let Some(player_entity) = resources.player_entity {
                if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                    if resources.inventory_selection < inventory.items.len().saturating_sub(1) {
                        resources.inventory_selection += 1;
                    }
                }
            }
        }
        // Equip/Unequip item
        KeyCode::Char('e') | KeyCode::Char('E') => {
            if let Some(player_entity) = resources.player_entity {
                // Extract item and equipable data first
                let equip_action = {
                    if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                        if resources.inventory_selection < inventory.items.len() {
                            let item = inventory.items[resources.inventory_selection];

                            // Check if item is equipable
                            if let Ok(equipable) = world.get::<&Equipable>(item) {
                                let slot = equipable.slot;
                                let is_equipped = inventory.get_equipped(slot) == Some(item);
                                Some((item, slot, is_equipped))
                            } else {
                                Some((item, crate::ecs::EquipSlot::MainHand, false)) // Dummy - will be rejected
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                // Now perform action
                if let Some((item, slot, is_equipped)) = equip_action {
                    // Verify item is equipable
                    if world.get::<&Equipable>(item).is_ok() {
                        if is_equipped {
                            // Unequip
                            let _ = world.insert_one(player_entity, WantsToUnequipItem { slot });
                        } else {
                            // Equip
                            let _ = world.insert_one(player_entity, WantsToEquipItem { item });
                        }

                        resources.mode = RunMode::PlayerTurn;
                        resources.in_inventory_mode = false;
                    } else {
                        resources.log.add("This item cannot be equipped.");
                    }
                }
            }
        }
        // Drop item
        KeyCode::Char('d') | KeyCode::Char('D') => {
            if let Some(player_entity) = resources.player_entity {
                // Extract item first
                let item_to_drop = {
                    if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                        if resources.inventory_selection < inventory.items.len() {
                            Some(inventory.items[resources.inventory_selection])
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                if let Some(item) = item_to_drop {
                    let _ = world.insert_one(player_entity, WantsToDropItem { item });

                    resources.mode = RunMode::PlayerTurn;
                    resources.in_inventory_mode = false;
                }
            }
        }
        // Use item
        KeyCode::Char('u') | KeyCode::Char('U') => {
            if let Some(player_entity) = resources.player_entity {
                // Extract item first
                let item_to_use = {
                    if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                        if resources.inventory_selection < inventory.items.len() {
                            let item = inventory.items[resources.inventory_selection];
                            // Check if item is consumable
                            if world.get::<&Consumable>(item).is_ok() {
                                Some(item)
                            } else {
                                None
                            }
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                };

                if let Some(item) = item_to_use {
                    let _ = world.insert_one(player_entity, WantsToUseItem { item });

                    resources.mode = RunMode::PlayerTurn;
                    resources.in_inventory_mode = false;
                } else if let Some(player_entity) = resources.player_entity {
                    if let Ok(inventory) = world.get::<&Inventory>(player_entity) {
                        if resources.inventory_selection < inventory.items.len() {
                            resources.log.add("This item cannot be used.");
                        }
                    }
                }
            }
        }
        KeyCode::Char('w') => {
            // Wield weapon (quick equip to main hand) in inventory mode
            quick_wield_weapon(world, resources);
            resources.in_inventory_mode = false;
        }
        KeyCode::Char('W') => {
            // Wear armor (quick equip to body) in inventory mode
            quick_wear_armor(world, resources);
            resources.in_inventory_mode = false;
        }
        KeyCode::Char('T') => {
            // Take off (unequip) in inventory mode
            quick_take_off(world, resources);
            resources.in_inventory_mode = false;
        }
        _ => {}
    }

    true
}

fn try_open_door(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player_entity {
        // Get player position
        let player_pos = if let Ok(pos) = world.get::<&Position>(player_entity) {
            (pos.x, pos.y, pos.layer)
        } else {
            return;
        };

        // Get the active map
        let map = if let Some(location_id) = resources.current_location {
            resources.settlement_maps.get_mut(&location_id)
        } else {
            Some(resources.maps.active_map_mut())
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
                resources.log.add("There is no door to open nearby.");
            } else if doors_found.len() == 1 {
                // Open the single door
                let (x, y, idx) = doors_found[0];
                map.tiles[idx] = crate::map::Tile::OpenDoor;
                resources.log.add(format!("You open the door at ({}, {}).", x, y));
                resources.mode = RunMode::PlayerTurn; // Opening a door takes a turn
            } else {
                // Multiple doors - open the first one for now
                // TODO: Let player choose which door to open
                let (x, y, idx) = doors_found[0];
                map.tiles[idx] = crate::map::Tile::OpenDoor;
                resources.log.add(format!("You open the door at ({}, {}).", x, y));
                resources.mode = RunMode::PlayerTurn;
            }
        }
    }
}

fn try_close_door(world: &mut World, resources: &mut Resources) {
    if let Some(player_entity) = resources.player_entity {
        // Get player position
        let player_pos = if let Ok(pos) = world.get::<&Position>(player_entity) {
            (pos.x, pos.y, pos.layer)
        } else {
            return;
        };

        // Get the active map
        let map = if let Some(location_id) = resources.current_location {
            resources.settlement_maps.get_mut(&location_id)
        } else {
            Some(resources.maps.active_map_mut())
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
                resources.log.add("There is no door to close nearby.");
            } else if doors_found.len() == 1 {
                // Close the single door
                let (x, y, idx) = doors_found[0];
                map.tiles[idx] = crate::map::Tile::ClosedDoor;
                resources.log.add(format!("You close the door at ({}, {}).", x, y));
                resources.mode = RunMode::PlayerTurn; // Closing a door takes a turn
            } else {
                // Multiple doors - close the first one for now
                // TODO: Let player choose which door to close
                let (x, y, idx) = doors_found[0];
                map.tiles[idx] = crate::map::Tile::ClosedDoor;
                resources.log.add(format!("You close the door at ({}, {}).", x, y));
                resources.mode = RunMode::PlayerTurn;
            }
        }
    }
}

fn try_use_stairs(world: &mut World, resources: &mut Resources, ascending: bool) {
    if let Some(player_entity) = resources.player_entity {
        // Get player position
        let player_pos = if let Ok(pos) = world.get::<&Position>(player_entity) {
            (pos.x, pos.y, pos.layer)
        } else {
            return;
        };

        // Get the map at current depth
        let map = if resources.current_depth > 0 {
            // In dungeon
            resources.dungeon_levels.get(&resources.current_depth)
        } else if let Some(location_id) = resources.current_location {
            // In settlement
            resources.settlement_maps.get(&location_id)
        } else {
            // On wilderness map
            Some(resources.maps.active_map())
        };

        if let Some(map) = map {
            // Check if standing on appropriate stairs
            let idx = map.xy_idx(player_pos.0, player_pos.1);
            let tile = map.tiles[idx];

            if ascending {
                // Trying to ascend
                if tile == crate::map::Tile::StairsUp {
                    if resources.current_depth > 0 {
                        // Ascending back up
                        resources.current_depth -= 1;
                        resources.log.add(format!("You climb the stairs upward to depth {}.", resources.current_depth));

                        // Find stairs down on the level above to place player
                        // For now, just place player at a safe starting position
                        // TODO: Remember stair positions between levels

                        resources.mode = RunMode::PlayerTurn;
                    } else {
                        resources.log.add("You are already on the surface level.");
                    }
                } else {
                    resources.log.add("You need to stand on stairs going up (<) to ascend.");
                }
            } else {
                // Trying to descend
                if tile == crate::map::Tile::StairsDown {
                    // Going deeper
                    resources.current_depth += 1;

                    // Generate new level if it doesn't exist
                    if !resources.dungeon_levels.contains_key(&resources.current_depth) {
                        let new_level = crate::map::generate_dungeon_level(
                            resources.maps.active_map().width,
                            resources.maps.active_map().height,
                            &mut resources.rng
                        );
                        resources.dungeon_levels.insert(resources.current_depth, new_level);
                        resources.log.add(format!("You descend into the depths... (Depth {})", resources.current_depth));
                    } else {
                        resources.log.add(format!("You climb down the stairs to depth {}.", resources.current_depth));
                    }

                    // Find stairs up on new level to place player
                    // For now, just search for the first StairsUp
                    if let Some(new_map) = resources.dungeon_levels.get(&resources.current_depth) {
                        for y in 0..new_map.height {
                            for x in 0..new_map.width {
                                let idx = new_map.xy_idx(x, y);
                                if new_map.tiles[idx] == crate::map::Tile::StairsUp {
                                    // Place player at stairs up
                                    if let Ok(mut pos) = world.get::<&mut Position>(player_entity) {
                                        pos.x = x;
                                        pos.y = y;
                                    }
                                    resources.mode = RunMode::PlayerTurn;
                                    return;
                                }
                            }
                        }
                    }

                    resources.mode = RunMode::PlayerTurn;
                } else {
                    resources.log.add("You need to stand on stairs going down (>) to descend.");
                }
            }
        }
    }
}

/// Handle main menu input
fn handle_main_menu_input(key_code: KeyCode, world: &mut World, resources: &mut Resources) -> bool {
    use crate::ui::MenuOption;

    match key_code {
        KeyCode::Char('q') | KeyCode::Esc => {
            // Quit from main menu
            return false;
        }
        KeyCode::Up | KeyCode::Char('k') => {
            // Move selection up
            resources.menu_selection = resources.menu_selection.prev();
        }
        KeyCode::Down | KeyCode::Char('j') => {
            // Move selection down
            resources.menu_selection = resources.menu_selection.next();
        }
        KeyCode::Enter => {
            let save_exists = std::path::Path::new("savegame.json").exists();

            match resources.menu_selection {
                MenuOption::Continue => {
                    if save_exists {
                        // Load game
                        match quick_load(world, resources) {
                            Ok(seed) => {
                                resources.seed = seed;
                                resources.in_main_menu = false;
                                resources.log.add("Game loaded successfully!");

                                // Reset camera to player position
                                if let Some(player_entity) = resources.player_entity {
                                    if let Ok(pos) = world.get::<&Position>(player_entity) {
                                        resources.camera.center_on(pos.x, pos.y);
                                    }
                                }
                            }
                            Err(e) => {
                                // If load fails, stay in menu and show error
                                resources.log.add(format!("Failed to load: {}", e));
                            }
                        }
                    } else {
                        // No save file, can't continue
                        resources.log.add("No save file found.");
                    }
                }
                MenuOption::NewGame => {
                    // Exit main menu and start new game
                    resources.in_main_menu = false;
                    resources.log.add("Starting new game...");
                }
                MenuOption::Quit => {
                    return false;
                }
            }
        }
        _ => {}
    }

    true
}
