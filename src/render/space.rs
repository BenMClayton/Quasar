use crate::game::{travel::world_star_position, Game};
use macroquad::prelude::*;

pub fn draw_space_scene(game: &Game) {
    clear_background(Color::from_rgba(5, 6, 18, 255));
    draw_nebula(game.tick);
    draw_stars(game.ship.position, game.tick);
    draw_star_map(game);
    draw_ship(screen_center(), game.tick);
}

fn draw_nebula(tick: f32) {
    for i in 0..9 {
        let x = 70.0 + i as f32 * screen_width() * 0.12 + (tick * 0.16 + i as f32).sin() * 18.0;
        let y = 70.0 + hash_to_float(i, 9) * (screen_height() - 140.0).max(120.0);
        draw_circle(
            x,
            y,
            90.0 + hash_to_float(i, 3) * 65.0,
            Color::from_rgba(34, 59, 115, 24),
        );
    }
}

fn draw_stars(camera: Vec2, tick: f32) {
    for i in 0..520 {
        let x = wrap(
            hash_to_float(i, 0) * 2800.0 - camera.x * 0.22,
            screen_width(),
        );
        let y = wrap(
            hash_to_float(i, 1) * 1800.0 - camera.y * 0.22,
            screen_height(),
        );
        let blink = if (tick * 2.0 + i as f32).sin() > 0.78 {
            1.0
        } else {
            0.58
        };
        draw_circle(
            x,
            y,
            if i % 11 == 0 { 1.6 } else { 1.0 },
            Color::new(blink, blink, blink, 1.0),
        );
    }
}

fn draw_star_map(game: &Game) {
    let camera = game.ship.position;
    for window in game.worlds.windows(2) {
        let a = project_star(world_star_position(window[0].id()), camera);
        let b = project_star(world_star_position(window[1].id()), camera);
        draw_line(a.x, a.y, b.x, b.y, 2.0, Color::from_rgba(45, 70, 113, 255));
    }

    for world in &game.worlds {
        let pos = project_star(world_star_position(world.id()), camera);
        let color = if world.id() == game.current_world {
            Color::from_rgba(255, 232, 121, 255)
        } else {
            world.palette().accent
        };
        draw_circle(
            pos.x,
            pos.y,
            22.0,
            Color::new(color.r, color.g, color.b, 0.14),
        );
        draw_circle(pos.x, pos.y, 8.0, color);
        draw_circle_lines(
            pos.x,
            pos.y,
            23.0,
            2.0,
            Color::from_rgba(126, 160, 202, 255),
        );
    }
}

fn draw_ship(position: Vec2, tick: f32) {
    let flame = 8.0 + (tick * 20.0).sin().abs() * 9.0;
    draw_circle(
        position.x,
        position.y + 9.0,
        32.0,
        Color::from_rgba(64, 146, 176, 35),
    );
    draw_triangle(
        vec2(position.x - 16.0, position.y + 18.0),
        vec2(position.x, position.y + 32.0 + flame),
        vec2(position.x + 16.0, position.y + 18.0),
        Color::from_rgba(255, 107, 74, 255),
    );
    draw_triangle(
        vec2(position.x, position.y - 28.0),
        vec2(position.x - 22.0, position.y + 22.0),
        vec2(position.x + 22.0, position.y + 22.0),
        Color::from_rgba(229, 232, 218, 255),
    );
    draw_rectangle(
        position.x - 8.0,
        position.y - 6.0,
        16.0,
        13.0,
        Color::from_rgba(42, 224, 235, 255),
    );
}

fn project_star(position: Vec2, camera: Vec2) -> Vec2 {
    screen_center() + position - camera
}

fn screen_center() -> Vec2 {
    vec2(screen_width() * 0.5, screen_height() * 0.5)
}

fn hash_to_float(a: u32, b: u32) -> f32 {
    let mut x = a.wrapping_mul(0x9e37_79b9) ^ b.wrapping_mul(0x85eb_ca6b);
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    (x as f32) / (u32::MAX as f32)
}

fn wrap(value: f32, max: f32) -> f32 {
    value.rem_euclid(max)
}
