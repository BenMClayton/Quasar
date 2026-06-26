use crate::{
    game::Inventory,
    world::{TileKind, World, WorldId},
};
use macroquad::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MachineKind {
    Miner,
    DeepMiner,
    Furnace,
    Assembler,
    Belt,
}

pub struct Machine {
    pub world: WorldId,
    pub tile: IVec2,
    pub kind: MachineKind,
    pub progress: f32,
    pub phase: f32,
}

impl Machine {
    pub fn new(world: WorldId, tile: IVec2, kind: MachineKind) -> Self {
        Self {
            world,
            tile,
            kind,
            progress: 0.0,
            phase: 0.0,
        }
    }

    pub fn update(&mut self, dt: f32, worlds: &[World], inventory: &mut Inventory) {
        self.phase += dt;

        match self.kind {
            MachineKind::Miner | MachineKind::DeepMiner => {
                self.progress += dt;
                let cycle_time = if self.kind == MachineKind::DeepMiner {
                    1.8
                } else {
                    3.6
                };

                if self.progress >= cycle_time {
                    self.progress = 0.0;
                    match worlds[self.world.0].tile(self.tile).kind {
                        TileKind::CopperOre => inventory.copper_ore += 1,
                        TileKind::CoalOre => inventory.coal += 1,
                        TileKind::Stone => inventory.stone += 1,
                        TileKind::Crystal => inventory.crystal += 1,
                        _ => {}
                    }
                }
            }
            MachineKind::Furnace => {
                if inventory.copper_ore > 0 && inventory.coal > 0 {
                    self.progress += dt;
                    if self.progress >= 5.0 {
                        self.progress = 0.0;
                        inventory.copper_ore -= 1;
                        inventory.coal -= 1;
                        inventory.copper_plate += 1;
                    }
                } else {
                    self.progress = (self.progress - dt * 0.25).max(0.0);
                }
            }
            MachineKind::Assembler => {
                if inventory.copper_plate >= 2 {
                    self.progress += dt;
                    if self.progress >= 6.0 {
                        self.progress = 0.0;
                        inventory.copper_plate -= 2;
                        inventory.copper_gear += 1;
                    }
                } else {
                    self.progress = (self.progress - dt * 0.2).max(0.0);
                }
            }
            MachineKind::Belt => {
                self.progress = (self.progress + dt * 1.8) % 1.0;
            }
        }
    }
}
