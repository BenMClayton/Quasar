mod app;
mod config;
mod game;
mod input;
mod render;
mod world;

use macroquad::prelude::*;

use config::window_conf;

#[macroquad::main(window_conf)]
async fn main() {
    let mut app = app::App::new();

    loop {
        let dt = get_frame_time().min(1.0 / 20.0);
        app.update(dt);
        app.draw();
        next_frame().await;
    }
}
