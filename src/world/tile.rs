#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Biome {
    Meadow,
    Forest,
    Wetland,
    Highland,
    Ash,
    Snow,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TileKind {
    Meadow,
    Forest,
    Stone,
    CopperOre,
    CoalOre,
    Crystal,
    Cliff,
    Water,
    Sand,
    Snow,
}

impl TileKind {
    pub fn is_resource(self) -> bool {
        matches!(
            self,
            TileKind::Forest
                | TileKind::Stone
                | TileKind::CopperOre
                | TileKind::CoalOre
                | TileKind::Crystal
        )
    }

    pub fn is_walkable(self) -> bool {
        !matches!(self, TileKind::Water | TileKind::Cliff)
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Tile {
    pub kind: TileKind,
    pub biome: Biome,
    pub elevation: f32,
    pub moisture: f32,
    pub heat: f32,
}

impl Tile {
    pub fn new(kind: TileKind, biome: Biome, elevation: f32, moisture: f32, heat: f32) -> Self {
        Self {
            kind,
            biome,
            elevation,
            moisture,
            heat,
        }
    }
}
