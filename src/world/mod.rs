mod generator;
pub(crate) mod noise;
mod palette;
mod tile;

pub use generator::World;
pub use palette::Palette;
pub use tile::{Biome, Tile, TileKind};

use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct WorldId(pub usize);

pub fn tile_from_position(position: Vec2) -> IVec2 {
    ivec2(position.x.floor() as i32, position.y.floor() as i32)
}

pub fn tile_center(tile: IVec2) -> Vec2 {
    tile.as_vec2() + vec2(0.5, 0.5)
}
