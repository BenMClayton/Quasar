use macroquad::prelude::*;

pub const LOGICAL_WIDTH: f32 = 1280.0;
pub const LOGICAL_HEIGHT: f32 = 720.0;
pub const MIN_CAMERA_ZOOM: f32 = 0.72;
pub const MAX_CAMERA_ZOOM: f32 = 1.85;

pub const ISO_TILE_W: f32 = 54.0;
pub const ISO_TILE_H: f32 = 28.0;
pub const ISO_HEIGHT: f32 = 18.0;

pub const PLAYER_SPEED: f32 = 5.2;
pub const SHIP_SPEED: f32 = 160.0;
pub const BEACON_PLATE_COST: u32 = 3;

pub fn window_conf() -> Conf {
    Conf {
        window_title: "Quasar".to_owned(),
        window_width: LOGICAL_WIDTH as i32,
        window_height: LOGICAL_HEIGHT as i32,
        window_resizable: true,
        high_dpi: true,
        sample_count: 1,
        ..Default::default()
    }
}
