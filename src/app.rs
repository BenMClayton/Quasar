use crate::{game::Game, render::Renderer};

pub struct App {
    game: Game,
    renderer: Renderer,
}

impl App {
    pub fn new() -> Self {
        Self {
            game: Game::new(),
            renderer: Renderer::new(),
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.game.update(dt);
    }

    pub fn draw(&mut self) {
        self.renderer.draw(&self.game);
    }
}
