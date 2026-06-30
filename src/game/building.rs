use crate::{game::Inventory, world::WorldId};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BuildKind {
    Miner,
    Furnace,
    Belt,
    Assembler,
    DeepMiner,
    PowerRelay,
    StoneBlock,
    TimberFrame,
    GlowLamp,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StructureKind {
    StoneBlock,
    TimberFrame,
    GlowLamp,
    PowerRelay,
}

pub struct Structure {
    pub world: WorldId,
    pub tile: IVec2,
    pub kind: StructureKind,
}

#[derive(Clone, Copy, Default, Debug)]
pub struct Cost {
    pub wood: u32,
    pub stone: u32,
    pub copper_ore: u32,
    pub coal: u32,
    pub copper_plate: u32,
    pub copper_gear: u32,
    pub crystal: u32,
}

impl BuildKind {
    pub const ALL: [BuildKind; 9] = [
        BuildKind::Miner,
        BuildKind::Furnace,
        BuildKind::Belt,
        BuildKind::Assembler,
        BuildKind::DeepMiner,
        BuildKind::PowerRelay,
        BuildKind::StoneBlock,
        BuildKind::TimberFrame,
        BuildKind::GlowLamp,
    ];

    pub fn name(self) -> &'static str {
        match self {
            BuildKind::Miner => "Miner",
            BuildKind::Furnace => "Furnace",
            BuildKind::Belt => "Belt",
            BuildKind::Assembler => "Assembler",
            BuildKind::DeepMiner => "Deep Miner",
            BuildKind::PowerRelay => "Power Relay",
            BuildKind::StoneBlock => "Stone Block",
            BuildKind::TimberFrame => "Timber Frame",
            BuildKind::GlowLamp => "Glow Lamp",
        }
    }

    pub fn cost(self) -> Cost {
        match self {
            BuildKind::Miner => Cost {
                wood: 2,
                stone: 4,
                ..Default::default()
            },
            BuildKind::Furnace => Cost {
                stone: 8,
                ..Default::default()
            },
            BuildKind::Belt => Cost {
                copper_plate: 1,
                ..Default::default()
            },
            BuildKind::Assembler => Cost {
                stone: 6,
                copper_plate: 4,
                ..Default::default()
            },
            BuildKind::DeepMiner => Cost {
                stone: 6,
                copper_plate: 3,
                copper_gear: 1,
                crystal: 1,
                ..Default::default()
            },
            BuildKind::PowerRelay => Cost {
                copper_plate: 2,
                copper_gear: 1,
                crystal: 2,
                ..Default::default()
            },
            BuildKind::StoneBlock => Cost {
                stone: 3,
                ..Default::default()
            },
            BuildKind::TimberFrame => Cost {
                wood: 3,
                ..Default::default()
            },
            BuildKind::GlowLamp => Cost {
                copper_plate: 1,
                crystal: 1,
                ..Default::default()
            },
        }
    }

    pub fn structure_kind(self) -> Option<StructureKind> {
        match self {
            BuildKind::StoneBlock => Some(StructureKind::StoneBlock),
            BuildKind::TimberFrame => Some(StructureKind::TimberFrame),
            BuildKind::GlowLamp => Some(StructureKind::GlowLamp),
            BuildKind::PowerRelay => Some(StructureKind::PowerRelay),
            BuildKind::Miner
            | BuildKind::Furnace
            | BuildKind::Belt
            | BuildKind::Assembler
            | BuildKind::DeepMiner => None,
        }
    }
}

impl Cost {
    pub fn can_pay(self, inventory: &Inventory) -> bool {
        inventory.wood >= self.wood
            && inventory.stone >= self.stone
            && inventory.copper_ore >= self.copper_ore
            && inventory.coal >= self.coal
            && inventory.copper_plate >= self.copper_plate
            && inventory.copper_gear >= self.copper_gear
            && inventory.crystal >= self.crystal
    }

    pub fn pay(self, inventory: &mut Inventory) {
        inventory.wood -= self.wood;
        inventory.stone -= self.stone;
        inventory.copper_ore -= self.copper_ore;
        inventory.coal -= self.coal;
        inventory.copper_plate -= self.copper_plate;
        inventory.copper_gear -= self.copper_gear;
        inventory.crystal -= self.crystal;
    }

    pub fn summary(self) -> String {
        let mut parts = Vec::new();

        if self.wood > 0 {
            parts.push(format!("{} wood", self.wood));
        }
        if self.stone > 0 {
            parts.push(format!("{} stone", self.stone));
        }
        if self.copper_ore > 0 {
            parts.push(format!("{} ore", self.copper_ore));
        }
        if self.coal > 0 {
            parts.push(format!("{} coal", self.coal));
        }
        if self.copper_plate > 0 {
            parts.push(format!("{} plate", self.copper_plate));
        }
        if self.copper_gear > 0 {
            parts.push(format!("{} gear", self.copper_gear));
        }
        if self.crystal > 0 {
            parts.push(format!("{} crystal", self.crystal));
        }

        if parts.is_empty() {
            "free".to_owned()
        } else {
            parts.join(", ")
        }
    }
}
