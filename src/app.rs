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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn app_new_creates_valid_app() {
        let app = App::new();

        // Verify that new() creates a valid App instance with properly initialized components
        assert!(app.game.current_world().name() == "Glass Orchard");
        assert_eq!(app.game.worlds.len(), 5);
        assert!(app.game.player.position.x != 0.0);
        assert_eq!(app.game.inventory.wood, 2);
        
        // Verify renderer was initialized (it should not panic)
        let _ = app.renderer;

        // Basic sanity check - if we got here, the struct was constructed successfully
    }
}
