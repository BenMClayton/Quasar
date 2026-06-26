use crate::{
    render::iso::IsoCamera,
    world::{Palette, World},
};
use macroquad::prelude::*;

pub fn draw_player(position: Vec2, world: &World, camera: &IsoCamera, tick: f32) {
    let tile = ivec2(position.x.floor() as i32, position.y.floor() as i32);
    let base = camera.project(position, world.tile(tile).elevation);
    let bob = (tick * 7.0).sin() * 2.0;
    let s = camera.zoom();

    draw_ellipse(
        base.x,
        base.y + 10.0 * s,
        14.0 * s,
        5.0 * s,
        0.0,
        Color::new(0.0, 0.0, 0.0, 0.24),
    );
    draw_rectangle(
        base.x - 10.0 * s,
        base.y - 29.0 * s + bob,
        20.0 * s,
        22.0 * s,
        Color::from_rgba(37, 54, 72, 255),
    );
    draw_rectangle(
        base.x - 8.0 * s,
        base.y - 35.0 * s + bob,
        16.0 * s,
        20.0 * s,
        Color::from_rgba(237, 231, 204, 255),
    );
    draw_rectangle(
        base.x - 9.0 * s,
        base.y - 46.0 * s + bob,
        18.0 * s,
        13.0 * s,
        Color::from_rgba(30, 44, 60, 255),
    );
    draw_rectangle(
        base.x - 6.0 * s,
        base.y - 43.0 * s + bob,
        12.0 * s,
        7.0 * s,
        Color::from_rgba(58, 226, 209, 255),
    );
    draw_rectangle(
        base.x - 12.0 * s,
        base.y - 31.0 * s + bob,
        5.0 * s,
        17.0 * s,
        Color::from_rgba(221, 215, 190, 255),
    );
    draw_rectangle(
        base.x + 7.0 * s,
        base.y - 31.0 * s + bob,
        5.0 * s,
        17.0 * s,
        Color::from_rgba(221, 215, 190, 255),
    );
    draw_rectangle(
        base.x - 10.0 * s,
        base.y - 18.0 * s + bob,
        20.0 * s,
        4.0 * s,
        Color::from_rgba(90, 201, 185, 255),
    );
    draw_rectangle(
        base.x - 5.0 * s,
        base.y - 6.0 * s + bob,
        4.0 * s,
        8.0 * s,
        Color::from_rgba(18, 20, 29, 255),
    );
    draw_rectangle(
        base.x + 2.0 * s,
        base.y - 6.0 * s + bob,
        4.0 * s,
        8.0 * s,
        Color::from_rgba(18, 20, 29, 255),
    );
}

pub fn draw_beacon(base: Vec2, palette: Palette, tick: f32) {
    let pulse = 1.0 + (tick * 3.0).sin() * 0.12;
    draw_circle(
        base.x,
        base.y - 40.0,
        54.0 * pulse,
        Color::new(palette.glow.r, palette.glow.g, palette.glow.b, 0.12),
    );
    draw_rectangle(
        base.x - 10.0,
        base.y - 64.0,
        20.0,
        58.0,
        Color::from_rgba(11, 16, 26, 255),
    );
    draw_rectangle(base.x - 5.0, base.y - 57.0, 10.0, 45.0, palette.accent);
    draw_triangle(
        vec2(base.x, base.y - 82.0),
        vec2(base.x - 18.0, base.y - 52.0),
        vec2(base.x + 18.0, base.y - 52.0),
        Color::new(palette.glow.r, palette.glow.g, palette.glow.b, 0.65),
    );
}

pub fn draw_miner(base: Vec2, progress: f32) {
    let bob = (progress * 5.0).sin() * 2.0;
    draw_machine_shadow(base);
    draw_rectangle(
        base.x - 17.0,
        base.y - 22.0,
        34.0,
        22.0,
        Color::from_rgba(36, 48, 57, 255),
    );
    draw_rectangle(
        base.x - 12.0,
        base.y - 33.0 + bob,
        24.0,
        10.0,
        Color::from_rgba(219, 203, 142, 255),
    );
    draw_rectangle(
        base.x - 6.0,
        base.y - 17.0,
        12.0,
        12.0,
        Color::from_rgba(94, 222, 202, 255),
    );
}

pub fn draw_deep_miner(base: Vec2, progress: f32) {
    let bob = (progress * 7.0).sin() * 3.0;
    draw_machine_shadow(base);
    draw_rectangle(
        base.x - 21.0,
        base.y - 30.0,
        42.0,
        30.0,
        Color::from_rgba(24, 40, 50, 255),
    );
    draw_rectangle(
        base.x - 14.0,
        base.y - 45.0 + bob,
        28.0,
        14.0,
        Color::from_rgba(98, 232, 211, 255),
    );
    draw_rectangle(
        base.x - 7.0,
        base.y - 24.0,
        14.0,
        18.0,
        Color::from_rgba(235, 209, 123, 255),
    );
    draw_circle(
        base.x,
        base.y - 15.0,
        17.0,
        Color::from_rgba(98, 232, 211, 42),
    );
}

pub fn draw_furnace(base: Vec2, progress: f32) {
    let heat = (progress / 5.0).clamp(0.0, 1.0);
    draw_machine_shadow(base);
    draw_rectangle(
        base.x - 18.0,
        base.y - 28.0,
        36.0,
        28.0,
        Color::from_rgba(47, 43, 45, 255),
    );
    draw_rectangle_lines(
        base.x - 18.0,
        base.y - 28.0,
        36.0,
        28.0,
        2.0,
        Color::from_rgba(134, 123, 104, 255),
    );
    draw_rectangle(
        base.x - 9.0,
        base.y - 13.0,
        18.0,
        8.0,
        Color::new(1.0, 0.26 + heat * 0.48, 0.1, 1.0),
    );
}

pub fn draw_assembler(base: Vec2, progress: f32) {
    let spin = progress / 6.0;
    draw_machine_shadow(base);
    draw_rectangle(
        base.x - 24.0,
        base.y - 28.0,
        48.0,
        28.0,
        Color::from_rgba(45, 57, 78, 255),
    );
    draw_rectangle_lines(
        base.x - 24.0,
        base.y - 28.0,
        48.0,
        28.0,
        2.0,
        Color::from_rgba(136, 160, 205, 255),
    );
    draw_circle(
        base.x,
        base.y - 14.0,
        11.0,
        Color::from_rgba(224, 178, 104, 255),
    );
    draw_line(
        base.x,
        base.y - 14.0,
        base.x + (spin * std::f32::consts::TAU).cos() * 13.0,
        base.y - 14.0 + (spin * std::f32::consts::TAU).sin() * 13.0,
        3.0,
        Color::from_rgba(244, 238, 206, 255),
    );
}

pub fn draw_belt(base: Vec2, progress: f32) {
    draw_machine_shadow(base);
    draw_rectangle(
        base.x - 20.0,
        base.y - 10.0,
        40.0,
        14.0,
        Color::from_rgba(34, 40, 50, 255),
    );
    draw_line(
        base.x - 16.0,
        base.y - 6.0,
        base.x + 16.0,
        base.y - 6.0,
        3.0,
        Color::from_rgba(207, 166, 82, 255),
    );
    let notch = -14.0 + progress * 28.0;
    draw_rectangle(
        base.x + notch,
        base.y - 1.0,
        7.0,
        4.0,
        Color::from_rgba(229, 220, 182, 255),
    );
}

pub fn draw_stone_block(base: Vec2, palette: Palette) {
    draw_machine_shadow(base);
    draw_rectangle(base.x - 21.0, base.y - 29.0, 42.0, 29.0, palette.stone);
    draw_rectangle(
        base.x - 21.0,
        base.y - 6.0,
        42.0,
        7.0,
        shade(palette.stone, 0.58),
    );
    draw_rectangle_lines(
        base.x - 21.0,
        base.y - 29.0,
        42.0,
        30.0,
        2.0,
        shade(palette.stone, 0.72),
    );
    draw_line(
        base.x - 6.0,
        base.y - 28.0,
        base.x - 6.0,
        base.y - 1.0,
        1.0,
        shade(palette.stone, 0.78),
    );
    draw_line(
        base.x + 9.0,
        base.y - 28.0,
        base.x + 9.0,
        base.y - 1.0,
        1.0,
        shade(palette.stone, 0.78),
    );
}

pub fn draw_timber_frame(base: Vec2, palette: Palette) {
    let wood = shade(palette.forest, 0.62);
    draw_machine_shadow(base);
    draw_rectangle(base.x - 20.0, base.y - 35.0, 7.0, 35.0, wood);
    draw_rectangle(base.x + 13.0, base.y - 35.0, 7.0, 35.0, wood);
    draw_rectangle(base.x - 22.0, base.y - 36.0, 44.0, 7.0, shade(wood, 1.2));
    draw_line(
        base.x - 16.0,
        base.y - 2.0,
        base.x + 17.0,
        base.y - 32.0,
        5.0,
        shade(wood, 0.9),
    );
}

pub fn draw_glow_lamp(base: Vec2, palette: Palette, tick: f32) {
    let pulse = 0.78 + (tick * 3.7).sin() * 0.14;
    draw_circle(
        base.x,
        base.y - 34.0,
        42.0,
        Color::new(palette.glow.r, palette.glow.g, palette.glow.b, 0.15 * pulse),
    );
    draw_rectangle(
        base.x - 4.0,
        base.y - 40.0,
        8.0,
        40.0,
        shade(palette.stone, 0.68),
    );
    draw_circle(base.x, base.y - 44.0, 12.0, palette.glow);
    draw_circle(base.x, base.y - 44.0, 6.0, palette.highlight);
}

pub fn draw_power_relay(base: Vec2, palette: Palette, tick: f32) {
    let pulse = 0.8 + (tick * 4.1).sin() * 0.16;
    draw_machine_shadow(base);
    draw_circle(
        base.x,
        base.y - 36.0,
        54.0,
        Color::new(palette.glow.r, palette.glow.g, palette.glow.b, 0.11 * pulse),
    );
    draw_rectangle(
        base.x - 6.0,
        base.y - 52.0,
        12.0,
        52.0,
        shade(palette.stone, 0.62),
    );
    draw_triangle(
        vec2(base.x, base.y - 72.0),
        vec2(base.x - 22.0, base.y - 42.0),
        vec2(base.x + 22.0, base.y - 42.0),
        palette.glow,
    );
    draw_circle(base.x, base.y - 48.0, 10.0, palette.highlight);
}

fn draw_machine_shadow(base: Vec2) {
    draw_ellipse(
        base.x,
        base.y + 4.0,
        22.0,
        7.0,
        0.0,
        Color::new(0.0, 0.0, 0.0, 0.22),
    );
}

fn shade(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r * amount).clamp(0.0, 1.0),
        (color.g * amount).clamp(0.0, 1.0),
        (color.b * amount).clamp(0.0, 1.0),
        color.a,
    )
}
