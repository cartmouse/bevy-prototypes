use bevy::prelude::*;

pub fn is_pos_over(test_pos: &Vec2, target_pos: &Vec3, target_size: f32) -> bool {
    let half_size = target_size / 2.0;
    return test_pos.x <= target_pos.x + half_size
        && test_pos.x >= target_pos.x - half_size
        && test_pos.y <= target_pos.y + half_size
        && test_pos.y >= target_pos.y - half_size;
}

pub fn is_item_over(target_pos: &Vec3, target_size: f32, item_pos: &Vec3, item_size: f32) -> bool {
    let half_size = item_size / 2.0;
    return [
        (item_pos.x - half_size, item_pos.y - half_size),
        (item_pos.x + half_size, item_pos.y - half_size),
        (item_pos.x - half_size, item_pos.y + half_size),
        (item_pos.x + half_size, item_pos.y + half_size),
    ]
    .iter()
    .any(|pos| is_pos_over(&Vec2::new(pos.0, pos.1), target_pos, target_size));
}
