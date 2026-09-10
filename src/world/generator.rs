use crate::world::{
    noise::{fractal_noise, hash_float, ridged_noise},
    palette::{palette_for, Palette},
    tile_center, Biome, Tile, TileKind, WorldId,
};
use macroquad::prelude::*;

pub struct World {
    id: WorldId,
    name: &'static str,
    width: i32,
    height: i32,
    tiles: Vec<Tile>,
    palette: Palette,
    spawn: IVec2,
    beacon: IVec2,
    rotation_period_seconds: f32,
    solar_phase: f32,
}

impl World {
    pub fn new(id: WorldId, name: &'static str, seed: u32) -> Self {
        generate_world(id, name, seed)
    }

    pub fn id(&self) -> WorldId {
        self.id
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn palette(&self) -> Palette {
        self.palette
    }

    pub fn spawn_position(&self) -> Vec2 {
        tile_center(self.spawn)
    }

    pub fn beacon_position(&self) -> Vec2 {
        tile_center(self.beacon)
    }

    #[cfg(test)]
    pub fn spawn_tile(&self) -> IVec2 {
        self.spawn
    }

    pub fn beacon_tile(&self) -> IVec2 {
        self.beacon
    }

    pub fn rotation_period_seconds(&self) -> f32 {
        self.rotation_period_seconds
    }

    pub fn local_day_time(&self, elapsed_seconds: f32) -> f32 {
        (self.solar_phase + elapsed_seconds / self.rotation_period_seconds).rem_euclid(1.0)
    }

    pub fn tile(&self, position: IVec2) -> Tile {
        if position.x < 0 || position.y < 0 || position.x >= self.width || position.y >= self.height
        {
            return Tile::new(TileKind::Cliff, Biome::Highland, 0.0, 0.0, 0.0);
        }

        self.tiles[(position.y * self.width + position.x) as usize]
    }

    pub fn is_walkable_position(&self, position: Vec2) -> bool {
        let tile = ivec2(position.x.floor() as i32, position.y.floor() as i32);
        self.tile(tile).kind.is_walkable()
    }
}

fn generate_world(id: WorldId, name: &'static str, seed: u32) -> World {
    let width = 96;
    let height = 96;
    let palette = palette_for(id);
    let spawn = ivec2(width / 2 - 8, height / 2 + 7);
    let beacon = ivec2(width / 2 + 13, height / 2 - 8);
    let mut tiles = Vec::with_capacity((width * height) as usize);

    for y in 0..height {
        for x in 0..width {
            tiles.push(generate_tile(x, y, width, height, id, seed, spawn, beacon));
        }
    }

    World {
        id,
        name,
        width,
        height,
        tiles,
        palette,
        spawn,
        beacon,
        rotation_period_seconds: 96.0 + id.0 as f32 * 37.0,
        solar_phase: ((seed % 997) as f32 / 997.0 + id.0 as f32 * 0.173).rem_euclid(1.0),
    }
}

#[allow(clippy::too_many_arguments)]
fn generate_tile(
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    id: WorldId,
    seed: u32,
    spawn: IVec2,
    beacon: IVec2,
) -> Tile {
    if x <= 0 || y <= 0 || x >= width - 1 || y >= height - 1 {
        return Tile::new(TileKind::Cliff, Biome::Highland, 0.9, 0.0, 0.0);
    }

    let fx = x as f32;
    let fy = y as f32;
    let center = vec2(width as f32 * 0.5, height as f32 * 0.5);
    let distance = (vec2(fx, fy).distance(center) / (width.min(height) as f32 * 0.58)).min(1.0);
    let falloff = distance.powf(2.25);
    let continent = fractal_noise(fx, fy, seed, 5, 34.0);
    let ridges = ridged_noise(fx, fy, seed ^ 0x7712_9090, 18.0);
    let moisture = fractal_noise(fx + 400.0, fy - 700.0, seed ^ 0x22cc_10ff, 4, 29.0);
    let heat = (1.0 - fy / height as f32) * 0.45
        + fractal_noise(fx - 120.0, fy + 300.0, seed ^ 0xaa55_0011, 3, 42.0) * 0.55;
    let elevation = (continent * 0.72 + ridges * 0.36 - falloff * 0.58).clamp(0.0, 1.0);

    let tile = ivec2(x, y);
    if tile.as_vec2().distance(spawn.as_vec2()) < 7.5
        || tile.as_vec2().distance(beacon.as_vec2()) < 6.0
    {
        return Tile::new(
            TileKind::Meadow,
            Biome::Meadow,
            elevation.max(0.45),
            moisture,
            heat,
        );
    }

    let biome = choose_biome(id, elevation, moisture, heat);
    let mut kind = choose_surface_kind(biome, elevation, moisture, heat);

    let copper = ridged_noise(fx + 900.0, fy - 200.0, seed ^ 0xc0a1_cafe, 13.0);
    let coal = ridged_noise(fx - 300.0, fy + 800.0, seed ^ 0x0dd5_a11e, 11.0);
    let crystal = ridged_noise(fx + 220.0, fy + 180.0, seed ^ 0x5eed_f00d, 16.0);
    let stone = ridged_noise(fx - 90.0, fy + 80.0, seed ^ 0x51ab_2026, 10.0);

    if matches!(
        kind,
        TileKind::Meadow | TileKind::Forest | TileKind::Sand | TileKind::Snow
    ) {
        if copper > ore_threshold(id, 0.88) && elevation > 0.36 {
            kind = TileKind::CopperOre;
        } else if coal > ore_threshold(id, 0.9) && elevation > 0.32 {
            kind = TileKind::CoalOre;
        } else if crystal > ore_threshold(id, 0.93) && elevation > 0.48 && id.0 % 2 == 1 {
            kind = TileKind::Crystal;
        } else if stone > 0.82 && elevation > 0.46 {
            kind = TileKind::Stone;
        } else if hash_float(x, y, seed ^ 0xf04e_2026) > 0.94 && moisture > 0.42 {
            kind = TileKind::Forest;
        }
    }

    Tile::new(kind, biome, elevation, moisture, heat)
}

fn ore_threshold(id: WorldId, base: f32) -> f32 {
    let world_bias = match id.0 {
        0 => 0.035,
        1 => -0.015,
        4 => -0.01,
        _ => 0.0,
    };

    (base + world_bias).min(0.97)
}

fn choose_biome(id: WorldId, elevation: f32, moisture: f32, heat: f32) -> Biome {
    if id.0 == 1 {
        return Biome::Ash;
    }
    if heat < 0.28 || elevation > 0.78 {
        Biome::Snow
    } else if elevation > 0.62 {
        Biome::Highland
    } else if moisture > 0.68 {
        Biome::Wetland
    } else if moisture > 0.44 {
        Biome::Forest
    } else {
        Biome::Meadow
    }
}

fn choose_surface_kind(biome: Biome, elevation: f32, moisture: f32, heat: f32) -> TileKind {
    if elevation < 0.22 {
        return TileKind::Water;
    }
    if elevation < 0.28 && heat > 0.35 {
        return TileKind::Sand;
    }
    if elevation > 0.84 {
        return TileKind::Cliff;
    }

    match biome {
        Biome::Forest if moisture > 0.48 => TileKind::Forest,
        Biome::Wetland if moisture > 0.74 => TileKind::Water,
        Biome::Highland if elevation > 0.58 => TileKind::Stone,
        Biome::Ash if elevation > 0.5 => TileKind::CoalOre,
        Biome::Snow => TileKind::Snow,
        _ => TileKind::Meadow,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generated_world_is_deterministic() {
        let a = World::new(WorldId(0), "test", 1234);
        let b = World::new(WorldId(0), "test", 1234);

        assert_eq!(a.tile(ivec2(20, 31)).kind, b.tile(ivec2(20, 31)).kind);
        assert_eq!(a.tile(ivec2(44, 57)).kind, b.tile(ivec2(44, 57)).kind);
    }

    #[test]
    fn spawn_and_beacon_are_walkable() {
        let world = World::new(WorldId(1), "test", 5678);

        assert!(world.tile(world.spawn_tile()).kind.is_walkable());
        assert!(world.tile(world.beacon_tile()).kind.is_walkable());
    }

    #[test]
    fn generated_world_contains_resources() {
        let world = World::new(WorldId(0), "test", 1234);
        let mut resource_tiles = 0;

        for y in 1..world.height() - 1 {
            for x in 1..world.width() - 1 {
                if world.tile(ivec2(x, y)).kind.is_resource() {
                    resource_tiles += 1;
                }
            }
        }

        assert!(resource_tiles > 50);
    }
}
