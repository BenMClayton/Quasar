use crate::{
    config::ISO_HEIGHT,
    game::{Game, MachineKind, StructureKind},
    render::{
        entities,
        iso::{diamond, IsoCamera},
    },
    world::{noise::hash_float, tile_center, Palette, Tile, TileKind, World},
};
use macroquad::prelude::*;

pub fn draw_world_scene(game: &Game, zoom: f32) {
    let world = game.current_world();
    let camera = IsoCamera::new(game.player.position, zoom);

    let local_day_time = game.local_day_time();
    draw_backdrop(world, local_day_time);
    draw_tiles(game, world, &camera);
    draw_beacon(world, &camera, game.tick);
    draw_structures(game, &camera);
    draw_machines(game, &camera);
    entities::draw_player(game.player.position, world, &camera, game.tick);
    draw_night_overlay(local_day_time);
}

fn draw_backdrop(world: &World, day_time: f32) {
    let palette = world.palette();
    let sun = sun_strength(day_time);
    let tint = 0.36 + sun * 0.64;

    let sw = screen_width();
    let sh = screen_height();

    let bands = 96;
    let band_h = (sh / bands as f32).ceil();
    for band in 0..bands {
        let y = band as f32 * band_h;
        let t = y / sh;
        let color = lerp_color(palette.sky_top, palette.sky_bottom, t);
        draw_rectangle(
            0.0,
            y,
            sw,
            band_h + 1.0,
            Color::new(color.r * tint, color.g * tint, color.b * tint, 1.0),
        );
    }

    let sun_x = sw * (0.12 + day_time * 0.76);
    let sun_y = sh * (0.62 - sun * 0.48);
    draw_circle(
        sun_x,
        sun_y,
        34.0,
        Color::new(
            palette.highlight.r,
            palette.highlight.g,
            palette.highlight.b,
            0.18 + sun * 0.28,
        ),
    );
    draw_circle(
        sun_x,
        sun_y,
        13.0,
        Color::new(
            palette.highlight.r,
            palette.highlight.g,
            palette.highlight.b,
            0.7,
        ),
    );

    let horizon = sh * 0.42;
    for layer in 0..4 {
        let base_y = horizon + layer as f32 * 28.0;
        let color = Color::new(
            palette.shadow.r + layer as f32 * 0.025,
            palette.shadow.g + layer as f32 * 0.025,
            palette.shadow.b + layer as f32 * 0.035,
            0.18 + layer as f32 * 0.06,
        );
        for i in 0..12 {
            let x = i as f32 * 130.0 - layer as f32 * 32.0;
            draw_triangle(
                vec2(x, base_y + 120.0),
                vec2(x + 74.0, base_y),
                vec2(x + 176.0, base_y + 120.0),
                color,
            );
        }
    }
}

fn draw_tiles(game: &Game, world: &World, camera: &IsoCamera) {
    let player_tile = ivec2(
        game.player.position.x.floor() as i32,
        game.player.position.y.floor() as i32,
    );
    let radius_x = ((screen_width() / (54.0 * camera.zoom())).ceil() as i32 + 12).clamp(18, 54);
    let radius_y = ((screen_height() / (28.0 * camera.zoom())).ceil() as i32 + 12).clamp(18, 54);
    let min_x = (player_tile.x - radius_x).max(0);
    let max_x = (player_tile.x + radius_x).min(world.width() - 1);
    let min_y = (player_tile.y - radius_y).max(0);
    let max_y = (player_tile.y + radius_y).min(world.height() - 1);

    for diagonal in min_x + min_y..=max_x + max_y {
        for x in min_x..=max_x {
            let y = diagonal - x;
            if y < min_y || y > max_y {
                continue;
            }

            let tile_pos = ivec2(x, y);
            let mut tile = world.tile(tile_pos);
            tile.kind = game.effective_kind_at(world, tile_pos);
            draw_tile(tile_pos, tile, world.palette(), camera);
        }
    }
}

fn draw_tile(tile_pos: IVec2, tile: Tile, palette: Palette, camera: &IsoCamera) {
    let center = camera.project(tile_pos.as_vec2() + vec2(0.5, 0.5), tile.elevation);
    let sw = screen_width();
    let sh = screen_height();
    if center.x < -80.0 || center.x > sw + 80.0 || center.y < -80.0 || center.y > sh + 90.0 {
        return;
    }

    let points = diamond(center, camera.zoom());
    let base = palette.color_for(tile.kind, tile.biome);
    draw_tile_sides(points, tile.elevation, base, camera.zoom());
    draw_diamond(points, base);
    draw_diamond_outline(points, shade(base, 0.78));
    if camera.zoom() > 0.82 {
        draw_surface_texture(tile_pos, tile, palette, center, points);
    }
    draw_tile_detail(tile_pos, tile, palette, center, points);
}

fn draw_tile_sides(points: [Vec2; 4], elevation: f32, color: Color, zoom: f32) {
    let depth = (elevation * ISO_HEIGHT * zoom).clamp(4.0, 24.0);
    let bottom = vec2(0.0, depth);

    draw_triangle(points[1], points[2], points[2] + bottom, shade(color, 0.58));
    draw_triangle(
        points[1],
        points[2] + bottom,
        points[1] + bottom,
        shade(color, 0.58),
    );
    draw_triangle(points[2], points[3], points[3] + bottom, shade(color, 0.46));
    draw_triangle(
        points[2],
        points[3] + bottom,
        points[2] + bottom,
        shade(color, 0.46),
    );
}

fn draw_surface_texture(
    tile_pos: IVec2,
    tile: Tile,
    palette: Palette,
    center: Vec2,
    points: [Vec2; 4],
) {
    let h1 = hash_float(tile_pos.x, tile_pos.y, 0x1101);
    let h2 = hash_float(tile_pos.x, tile_pos.y, 0x2202);
    let h3 = hash_float(tile_pos.x, tile_pos.y, 0x3303);
    let detail = match tile.kind {
        TileKind::Meadow | TileKind::Forest => palette.highlight,
        TileKind::Sand => shade(palette.sand, 1.14),
        TileKind::Snow => shade(palette.snow, 0.92),
        TileKind::Water => shade(palette.water, 1.16),
        TileKind::Cliff | TileKind::Stone => shade(palette.stone, 0.78),
        TileKind::CopperOre => shade(palette.copper, 1.08),
        TileKind::CoalOre => shade(palette.coal, 1.42),
        TileKind::Crystal => palette.glow,
    };

    for i in 0..3 {
        let t = [h1, h2, h3][i];
        let offset = vec2(
            (t - 0.5) * 28.0,
            (hash_float(tile_pos.y, tile_pos.x, i as u32) - 0.5) * 12.0,
        );
        draw_line(
            center.x + offset.x - 7.0,
            center.y + offset.y,
            center.x + offset.x + 7.0,
            center.y + offset.y - 2.0,
            1.0,
            Color::new(detail.r, detail.g, detail.b, 0.13),
        );
    }

    if tile.kind == TileKind::Water {
        draw_line(
            points[3].x + 12.0,
            points[3].y,
            points[1].x - 12.0,
            points[1].y,
            2.0,
            Color::new(1.0, 1.0, 1.0, 0.24),
        );
    }
}

fn draw_tile_detail(
    tile_pos: IVec2,
    tile: Tile,
    palette: Palette,
    center: Vec2,
    points: [Vec2; 4],
) {
    match tile.kind {
        TileKind::Forest => draw_tree(center, palette, hash_float(tile_pos.x, tile_pos.y, 4)),
        TileKind::Stone => draw_rocks(center, palette.stone, palette.highlight),
        TileKind::CopperOre => draw_ore(center, palette.copper, palette.highlight),
        TileKind::CoalOre => draw_ore(center, palette.coal, palette.highlight),
        TileKind::Crystal => draw_crystal(center, palette.crystal, palette.glow),
        TileKind::Water => {
            draw_line(
                points[3].x + 8.0,
                points[3].y,
                points[1].x - 8.0,
                points[1].y,
                2.0,
                Color::new(1.0, 1.0, 1.0, 0.22),
            );
        }
        TileKind::Sand | TileKind::Snow | TileKind::Meadow | TileKind::Cliff => {}
    }

    if tile.moisture > 0.62 && matches!(tile.kind, TileKind::Meadow | TileKind::Sand) {
        draw_circle(
            center.x - 9.0,
            center.y - 1.0,
            2.0,
            Color::new(1.0, 1.0, 1.0, 0.12),
        );
    }
}

fn draw_tree(center: Vec2, palette: Palette, variant: f32) {
    let height = 58.0 + variant * 30.0;
    let width = 34.0 + variant * 16.0;
    let trunk = shade(palette.forest, 0.48);
    draw_rectangle(
        center.x - 5.0,
        center.y - height * 0.34,
        10.0,
        height * 0.4,
        trunk,
    );
    draw_rectangle(
        center.x - 2.0,
        center.y - height * 0.34,
        3.0,
        height * 0.38,
        shade(palette.highlight, 0.42),
    );
    draw_triangle(
        vec2(center.x, center.y - height),
        vec2(center.x - width * 0.62, center.y - height * 0.38),
        vec2(center.x + width * 0.62, center.y - height * 0.38),
        palette.forest,
    );
    draw_triangle(
        vec2(center.x, center.y - height * 0.78),
        vec2(center.x - width * 0.55, center.y - height * 0.18),
        vec2(center.x + width * 0.55, center.y - height * 0.18),
        shade(palette.forest, 1.15),
    );
    draw_triangle(
        vec2(center.x, center.y - height * 0.56),
        vec2(center.x - width * 0.48, center.y - height * 0.02),
        vec2(center.x + width * 0.48, center.y - height * 0.02),
        shade(palette.forest, 0.9),
    );
}

fn draw_rocks(center: Vec2, color: Color, highlight: Color) {
    draw_poly(
        center.x - 7.0,
        center.y - 4.0,
        6,
        8.0,
        0.4,
        shade(color, 0.72),
    );
    draw_poly(center.x + 6.0, center.y - 2.0, 6, 6.0, 0.1, color);
    draw_circle(center.x - 8.0, center.y - 8.0, 2.0, highlight);
}

fn draw_ore(center: Vec2, color: Color, highlight: Color) {
    draw_rocks(center, shade(color, 0.65), highlight);
    draw_circle(center.x - 3.0, center.y - 6.0, 3.0, color);
    draw_circle(center.x + 6.0, center.y - 3.0, 2.0, highlight);
}

fn draw_crystal(center: Vec2, color: Color, glow: Color) {
    draw_circle(
        center.x,
        center.y - 12.0,
        20.0,
        Color::new(glow.r, glow.g, glow.b, 0.12),
    );
    draw_triangle(
        vec2(center.x, center.y - 28.0),
        vec2(center.x - 8.0, center.y - 4.0),
        vec2(center.x + 8.0, center.y - 4.0),
        color,
    );
    draw_triangle(
        vec2(center.x + 3.0, center.y - 25.0),
        vec2(center.x + 1.0, center.y - 6.0),
        vec2(center.x + 8.0, center.y - 4.0),
        shade(color, 1.22),
    );
}

fn draw_beacon(world: &World, camera: &IsoCamera, tick: f32) {
    let base = camera.project(
        world.beacon_position(),
        world.tile(world.beacon_tile()).elevation,
    );
    entities::draw_beacon(base, world.palette(), tick);
}

fn draw_machines(game: &Game, camera: &IsoCamera) {
    for machine in game
        .machines
        .iter()
        .filter(|machine| machine.world == game.current_world)
    {
        let world = game.current_world();
        let pos = camera.project(
            tile_center(machine.tile),
            world.tile(machine.tile).elevation,
        );
        match machine.kind {
            MachineKind::Miner => entities::draw_miner(pos, machine.progress),
            MachineKind::DeepMiner => entities::draw_deep_miner(pos, machine.progress),
            MachineKind::Furnace => entities::draw_furnace(pos, machine.progress),
            MachineKind::Assembler => entities::draw_assembler(pos, machine.progress),
            MachineKind::Belt => entities::draw_belt(pos, machine.progress),
        }
    }
}

fn draw_structures(game: &Game, camera: &IsoCamera) {
    for structure in game
        .structures
        .iter()
        .filter(|structure| structure.world == game.current_world)
    {
        let world = game.current_world();
        let pos = camera.project(
            tile_center(structure.tile),
            world.tile(structure.tile).elevation,
        );
        match structure.kind {
            StructureKind::StoneBlock => entities::draw_stone_block(pos, world.palette()),
            StructureKind::TimberFrame => entities::draw_timber_frame(pos, world.palette()),
            StructureKind::GlowLamp => entities::draw_glow_lamp(pos, world.palette(), game.tick),
            StructureKind::PowerRelay => {
                entities::draw_power_relay(pos, world.palette(), game.tick)
            }
        }
    }
}

fn draw_night_overlay(day_time: f32) {
    let sun = sun_strength(day_time);
    let night = 1.0 - sun;
    if night <= 0.02 {
        return;
    }

    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.015, 0.025, 0.075, night * 0.48),
    );
}

fn sun_strength(day_time: f32) -> f32 {
    let angle = day_time * std::f32::consts::TAU;
    ((angle.sin() + 1.0) * 0.5).powf(0.72)
}

fn draw_diamond(points: [Vec2; 4], color: Color) {
    draw_triangle(points[0], points[1], points[2], color);
    draw_triangle(points[0], points[2], points[3], color);
}

fn draw_diamond_outline(points: [Vec2; 4], color: Color) {
    for i in 0..4 {
        let a = points[i];
        let b = points[(i + 1) % 4];
        draw_line(a.x, a.y, b.x, b.y, 1.0, color);
    }
}

fn shade(color: Color, amount: f32) -> Color {
    Color::new(
        (color.r * amount).clamp(0.0, 1.0),
        (color.g * amount).clamp(0.0, 1.0),
        (color.b * amount).clamp(0.0, 1.0),
        color.a,
    )
}

fn lerp_color(a: Color, b: Color, t: f32) -> Color {
    Color::new(
        a.r + (b.r - a.r) * t,
        a.g + (b.g - a.g) * t,
        a.b + (b.b - a.b) * t,
        a.a + (b.a - a.a) * t,
    )
}
