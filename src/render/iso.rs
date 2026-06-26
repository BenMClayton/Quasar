use crate::config::{ISO_HEIGHT, ISO_TILE_H, ISO_TILE_W};
use macroquad::prelude::*;

pub struct IsoCamera {
    focus: Vec2,
    zoom: f32,
}

impl IsoCamera {
    pub fn new(focus: Vec2, zoom: f32) -> Self {
        Self { focus, zoom }
    }

    pub fn project(&self, position: Vec2, elevation: f32) -> Vec2 {
        let projected = raw_project(position);
        let focus = raw_project(self.focus);
        vec2(screen_width() * 0.5, screen_height() * 0.47) + (projected - focus) * self.zoom
            - vec2(0.0, elevation * ISO_HEIGHT * self.zoom)
    }

    pub fn zoom(&self) -> f32 {
        self.zoom
    }
}

pub fn diamond(center: Vec2, zoom: f32) -> [Vec2; 4] {
    let half_w = ISO_TILE_W * 0.5 * zoom;
    let half_h = ISO_TILE_H * 0.5 * zoom;

    [
        vec2(center.x, center.y - half_h),
        vec2(center.x + half_w, center.y),
        vec2(center.x, center.y + half_h),
        vec2(center.x - half_w, center.y),
    ]
}

fn raw_project(position: Vec2) -> Vec2 {
    vec2(
        (position.x - position.y) * ISO_TILE_W * 0.5,
        (position.x + position.y) * ISO_TILE_H * 0.5,
    )
}
