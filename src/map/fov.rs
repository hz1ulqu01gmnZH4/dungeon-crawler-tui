use crate::map::Map;

pub fn compute_fov(map: &Map, x: i32, y: i32, range: i32) -> Vec<(i32, i32)> {
    let mut visible = Vec::new();

    // Add center point
    visible.push((x, y));

    // Cast rays in a circle
    for angle in 0..360 {
        let rad = (angle as f32).to_radians();
        let cos = rad.cos();
        let sin = rad.sin();

        let mut blocked = false;
        for distance in 1..=range {
            if blocked {
                break;
            }

            let target_x = x + (cos * distance as f32).round() as i32;
            let target_y = y + (sin * distance as f32).round() as i32;

            if !map.in_bounds(target_x, target_y) {
                break;
            }

            visible.push((target_x, target_y));

            if map.is_opaque(target_x, target_y) {
                blocked = true;
            }
        }
    }

    // Remove duplicates
    visible.sort_unstable();
    visible.dedup();
    visible
}
