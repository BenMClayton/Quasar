use crate::world::{World, WorldId};
use macroquad::prelude::*;

pub fn world_star_position(world_id: WorldId) -> Vec2 {
    let i = world_id.0 as f32;
    let angle = i * 1.973 + 0.45;
    let radius = 64.0 + i * 62.0;
    vec2(angle.cos() * radius, angle.sin() * radius)
}

pub fn nearest_world(worlds: &[World], ship_position: Vec2) -> Option<(WorldId, f32)> {
    worlds
        .iter()
        .map(|world| {
            let distance = ship_position.distance(world_star_position(world.id()));
            (world.id(), distance)
        })
        .min_by(|a, b| a.1.total_cmp(&b.1))
}
