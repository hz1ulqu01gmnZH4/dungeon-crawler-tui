use crate::ecs::{Position, Viewshed};
use crate::ecs::resources::Resources;
use hecs::World;

pub fn update_fov(world: &mut World, resources: &mut Resources) {
    let active_layer = resources.world.maps.active;
    let player_entity = resources.player.player_entity;

    // Calculate visibility modifier based on time of day and weather
    let time_modifier = resources.sim.world_time.time_of_day().visibility_modifier();
    let weather_modifier = resources.sim.weather.current_weather.visibility_modifier();
    let total_modifier = time_modifier + weather_modifier;

    // Update viewsheds and collect player visibility
    let mut player_visible = None;
    let map = resources.world.maps.active_map();

    for (entity, (pos, viewshed)) in world.query_mut::<(&Position, &mut Viewshed)>() {
        if !viewshed.dirty || pos.layer != active_layer {
            continue;
        }

        viewshed.dirty = false;
        viewshed.visible.clear();

        // Apply lighting modifiers to FOV range (minimum 3 tiles visibility)
        let effective_range = (viewshed.range + total_modifier).max(3);
        viewshed.visible = crate::map::compute_fov(map, pos.x, pos.y, effective_range);

        // Check if this is the player using the stored player entity
        if player_entity == Some(entity) {
            player_visible = Some(viewshed.visible.clone());
        }
    }

    // Update map visibility if player moved
    if let Some(visible) = player_visible {
        let map = resources.world.maps.active_map_mut();
        map.clear_visible();
        for (vx, vy) in &visible {
            map.set_visible(*vx, *vy);
        }
    }
}
