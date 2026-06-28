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

    /// Projects a world position to screen space with proper elevation handling.
    /// The camera maintains consistent vertical centering across resolutions and zoom levels.
    pub fn project(&self, position: Vec2, elevation: f32) -> Vec2 {
        let projected = raw_project(position);
        let focus_screen = raw_project(self.focus);

        // Calculate screen dimensions for proper aspect-ratio-aware positioning
        let _screen_w = screen_width() as f32;
        let screen_h = screen_height() as f32;

        // Use a normalized focal point that maintains consistent vertical centering.
        // The Y-center ratio (0.47) is applied relative to the scaled screen height,
        // ensuring stable camera behavior when zoom changes or window resizes occur.
        let target_center_y = screen_h * 0.47;

        // Compute offset from focus point in screen space
        let delta_x = projected.x - focus_screen.x;
        let delta_y = (projected.y - focus_screen.y) / self.zoom;

        // Apply zoom scaling to the horizontal component only for proper isometric projection
        vec2(
            target_center_y + delta_x * self.zoom,
            target_center_y + delta_y * self.zoom,
        ) - vec2(0.0, elevation * ISO_HEIGHT)
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
