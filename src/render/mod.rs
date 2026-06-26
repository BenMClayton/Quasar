mod entities;
mod iso;
mod space;
mod ui;
mod world;

use crate::{
    config::{MAX_CAMERA_ZOOM, MIN_CAMERA_ZOOM},
    game::{Game, MenuPanel, Mode},
    input,
};

pub struct Renderer {
    zoom: f32,
}

impl Renderer {
    pub fn new() -> Self {
        Self { zoom: 1.0 }
    }

    pub fn draw(&mut self, game: &Game) {
        if game.active_menu == MenuPanel::None {
            let scroll = input::scroll_delta();
            if scroll.abs() > f32::EPSILON {
                self.zoom = (self.zoom + scroll * 0.08).clamp(MIN_CAMERA_ZOOM, MAX_CAMERA_ZOOM);
            }
        }

        match game.mode {
            Mode::World => world::draw_world_scene(game, self.zoom),
            Mode::Space => space::draw_space_scene(game),
        }

        ui::draw(game);
    }
}
