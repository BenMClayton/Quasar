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
    // Generated now so later climate systems can remain deterministic.
    #[allow(dead_code)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tile_kind_is_resource_returns_true_for_resources() {
        // Verify that resource tiles correctly return true for is_resource
        assert!(TileKind::Forest.is_resource());
        assert!(TileKind::Stone.is_resource());
        assert!(TileKind::CopperOre.is_resource());
        assert!(TileKind::CoalOre.is_resource());
        assert!(TileKind::Crystal.is_resource());
    }

    #[test]
    fn tile_kind_is_resource_returns_false_for_non_resources() {
        // Verify that non-resource tiles correctly return false for is_resource
        assert!(!TileKind::Meadow.is_resource());
        assert!(!TileKind::Water.is_resource());
        assert!(!TileKind::Cliff.is_resource());
        assert!(!TileKind::Sand.is_resource());
        assert!(!TileKind::Snow.is_resource());
    }
}
