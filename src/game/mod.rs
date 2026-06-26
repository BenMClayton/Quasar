mod building;
mod inventory;
mod machine;
mod survival;
pub mod travel;

pub use building::{BuildKind, Cost, Structure, StructureKind};
pub use inventory::Inventory;
pub use machine::{Machine, MachineKind};
pub use survival::Survival;

use crate::{
    config::{BEACON_PLATE_COST, PLAYER_SPEED, SHIP_SPEED},
    input,
    world::{tile_from_position, TileKind, World, WorldId},
};
use macroquad::prelude::*;
use std::collections::HashSet;

type TileKey = (usize, i32, i32);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Mode {
    World,
    Space,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuPanel {
    None,
    Inventory,
    Crafting,
    Settings,
}

#[derive(Clone, Copy)]
pub struct Player {
    pub position: Vec2,
}

#[derive(Clone, Copy)]
pub struct Ship {
    pub position: Vec2,
    pub velocity: Vec2,
}

pub struct Message {
    pub text: String,
    pub timer: f32,
}

impl Message {
    fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            timer: 7.0,
        }
    }

    fn update(&mut self, dt: f32) {
        self.timer = (self.timer - dt).max(0.0);
    }

    fn set(&mut self, text: impl Into<String>) {
        self.text = text.into();
        self.timer = 5.5;
    }
}

pub struct Game {
    pub mode: Mode,
    pub worlds: Vec<World>,
    pub current_world: WorldId,
    pub player: Player,
    pub ship: Ship,
    pub inventory: Inventory,
    pub survival: Survival,
    pub machines: Vec<Machine>,
    pub structures: Vec<Structure>,
    pub selected_build: BuildKind,
    pub active_menu: MenuPanel,
    harvested: HashSet<TileKey>,
    pub message: Message,
    pub tick: f32,
}

impl Game {
    pub fn new() -> Self {
        let worlds = vec![
            World::new(WorldId(0), "Glass Orchard", 0x71d3_5ac1),
            World::new(WorldId(1), "Ash Machine", 0x11fe_0f5a),
            World::new(WorldId(2), "Tide Library", 0xc0de_5421),
            World::new(WorldId(3), "Neon Mycelium", 0x5eed_0097),
            World::new(WorldId(4), "Copper Snow", 0x8a17_03bd),
        ];
        let current_world = WorldId(0);

        Self {
            mode: Mode::World,
            current_world,
            player: Player {
                position: worlds[current_world.0].spawn_position(),
            },
            ship: Ship {
                position: travel::world_star_position(current_world),
                velocity: Vec2::ZERO,
            },
            inventory: Inventory {
                wood: 2,
                ..Default::default()
            },
            survival: Survival::new(),
            worlds,
            machines: Vec::new(),
            structures: Vec::new(),
            selected_build: BuildKind::Miner,
            active_menu: MenuPanel::None,
            harvested: HashSet::new(),
            message: Message::new("Gather resources, pick a build recipe, and repair the beacon."),
            tick: 0.0,
        }
    }

    pub fn current_world(&self) -> &World {
        &self.worlds[self.current_world.0]
    }

    pub fn update(&mut self, dt: f32) {
        self.tick += dt;
        self.message.update(dt);
        self.survival.update(dt, self.mode == Mode::World);
        self.update_machines(dt);

        match self.mode {
            Mode::World => self.update_world(dt),
            Mode::Space => self.update_space(dt),
        }
    }

    pub fn nearest_world(&self) -> Option<(WorldId, f32)> {
        travel::nearest_world(&self.worlds, self.ship.position)
    }

    pub fn local_day_time(&self) -> f32 {
        self.current_world().local_day_time(self.tick)
    }

    pub fn effective_kind_at(&self, world: &World, tile: IVec2) -> TileKind {
        if self.harvested.contains(&(world.id().0, tile.x, tile.y)) {
            TileKind::Meadow
        } else {
            world.tile(tile).kind
        }
    }

    pub fn machine_at_current_world(&self, tile: IVec2) -> Option<&Machine> {
        self.machines
            .iter()
            .find(|machine| machine.world == self.current_world && machine.tile == tile)
    }

    pub fn structure_at_current_world(&self, tile: IVec2) -> Option<&Structure> {
        self.structures
            .iter()
            .find(|structure| structure.world == self.current_world && structure.tile == tile)
    }

    fn update_world(&mut self, dt: f32) {
        self.update_menus();

        if self.active_menu != MenuPanel::None {
            self.update_menu_interactions();
            return;
        }

        self.update_hotbar_selection();

        let mut next = self.player.position + input::movement_axis() * PLAYER_SPEED * dt;
        let world = self.current_world();
        next.x = next.x.clamp(1.0, (world.width() - 2) as f32);
        next.y = next.y.clamp(1.0, (world.height() - 2) as f32);

        if world.is_walkable_position(next) {
            self.player.position = next;
        }

        if is_key_pressed(KeyCode::E) {
            self.harvest_nearby();
        }
        if is_key_pressed(KeyCode::B) {
            self.place_selected_build();
        }

        let beacon = self.current_world().beacon_position();
        if is_key_pressed(KeyCode::Space) && self.player.position.distance(beacon) < 2.4 {
            if self.inventory.copper_plate >= BEACON_PLATE_COST {
                self.inventory.copper_plate -= BEACON_PLATE_COST;
                self.mode = Mode::Space;
                self.ship.position = travel::world_star_position(self.current_world);
                self.ship.velocity = Vec2::ZERO;
                self.message
                    .set("Beacon repaired. The sky opened into the shared dark.");
            } else {
                self.message.set(format!(
                    "Beacon repair needs {} copper plates. Smelt copper ore with coal.",
                    BEACON_PLATE_COST
                ));
            }
        }
    }

    fn update_space(&mut self, dt: f32) {
        self.update_menus();

        if self.active_menu != MenuPanel::None {
            return;
        }

        let thrust = input::movement_axis();
        self.ship.velocity += thrust * SHIP_SPEED * dt;
        self.ship.velocity *= 0.982_f32.powf(dt * 60.0);
        self.ship.velocity = clamp_length(self.ship.velocity, SHIP_SPEED * 1.75);
        self.ship.position += self.ship.velocity * dt;

        if let Some((world_id, distance)) = self.nearest_world() {
            if distance < 24.0 && is_key_pressed(KeyCode::Space) {
                self.current_world = world_id;
                self.player.position = self.current_world().spawn_position();
                self.mode = Mode::World;
                self.message.set(format!(
                    "Landed in {}. Build, smelt, automate, repair the next beacon.",
                    self.current_world().name()
                ));
            }
        }
    }

    fn update_machines(&mut self, dt: f32) {
        let worlds = &self.worlds;
        for machine in &mut self.machines {
            machine.update(dt, worlds, &mut self.inventory);
        }
    }

    fn update_menus(&mut self) {
        if is_key_pressed(KeyCode::Escape) {
            self.active_menu = MenuPanel::None;
            return;
        }
        if is_key_pressed(KeyCode::F1) || input::clicked_settings_button() {
            self.active_menu = toggle_panel(self.active_menu, MenuPanel::Settings);
            return;
        }
        if is_key_pressed(KeyCode::I) {
            self.active_menu = toggle_panel(self.active_menu, MenuPanel::Inventory);
            return;
        }
        if is_key_pressed(KeyCode::C) {
            self.active_menu = toggle_panel(self.active_menu, MenuPanel::Crafting);
        }
    }

    fn update_hotbar_selection(&mut self) {
        let keys = hotbar_keys();

        for (index, key) in keys.into_iter().enumerate() {
            if is_key_pressed(key) {
                self.select_hotbar(index);
            }
        }

        if let Some(index) = input::clicked_hotbar_slot() {
            self.select_hotbar(index);
        }
    }

    fn update_menu_interactions(&mut self) {
        if self.active_menu != MenuPanel::Crafting {
            return;
        }

        let keys = hotbar_keys();

        for (index, key) in keys.into_iter().enumerate() {
            if is_key_pressed(key) {
                self.select_hotbar(index);
            }
        }

        if let Some(index) = input::clicked_crafting_slot() {
            self.select_hotbar(index);
        }
    }

    fn select_hotbar(&mut self, index: usize) {
        if index >= BuildKind::ALL.len() {
            return;
        }

        self.selected_build = BuildKind::ALL[index];
        self.message
            .set(format!("Selected: {}", self.selected_build.name()));
    }

    fn harvest_nearby(&mut self) {
        if let Some((tile, kind)) = self.nearest_resource_tile() {
            match kind {
                TileKind::Forest => {
                    self.harvested.insert(self.tile_key(tile));
                    self.inventory.wood += 2;
                    self.survival.feed(3.0);
                    self.message.set("Gathered timber and edible fruit.");
                }
                TileKind::Stone => {
                    self.inventory.stone += 2;
                    self.message
                        .set("Chipped stone from the outcrop. The node remains.");
                }
                TileKind::CopperOre => {
                    self.inventory.copper_ore += 1;
                    self.message
                        .set("Mined copper ore. The deposit can keep producing.");
                }
                TileKind::CoalOre => {
                    self.inventory.coal += 1;
                    self.message
                        .set("Pulled coal from the seam. The deposit remains.");
                }
                TileKind::Crystal => {
                    self.inventory.crystal += 1;
                    self.message
                        .set("Gathered crystal. The node can be mined again.");
                }
                _ => {}
            }
        } else {
            self.message.set("No nearby resource to harvest.");
        }
    }

    fn place_selected_build(&mut self) {
        let cost = self.selected_build.cost();
        if !cost.can_pay(&self.inventory) {
            self.message.set(format!(
                "{} needs {}.",
                self.selected_build.name(),
                cost.summary()
            ));
            return;
        }

        match self.selected_build {
            BuildKind::Miner => self.place_miner(cost),
            BuildKind::DeepMiner => self.place_miner_kind(cost, MachineKind::DeepMiner),
            BuildKind::Furnace => self.place_machine(cost, MachineKind::Furnace),
            BuildKind::Assembler => self.place_machine(cost, MachineKind::Assembler),
            BuildKind::Belt => self.place_machine(cost, MachineKind::Belt),
            BuildKind::StoneBlock
            | BuildKind::TimberFrame
            | BuildKind::GlowLamp
            | BuildKind::PowerRelay => self.place_structure(cost),
        }
    }

    fn place_miner(&mut self, cost: Cost) {
        self.place_miner_kind(cost, MachineKind::Miner);
    }

    fn place_miner_kind(&mut self, cost: Cost, machine_kind: MachineKind) {
        let Some((tile, kind)) = self.nearest_resource_tile() else {
            self.message
                .set("Stand near stone, coal, copper, or crystal to place a miner.");
            return;
        };

        if !matches!(
            kind,
            TileKind::Stone | TileKind::CopperOre | TileKind::CoalOre | TileKind::Crystal
        ) {
            self.message
                .set("Miners need stone, coal, copper, or crystal below them.");
            return;
        }
        if self.machine_at_current_world(tile).is_some() {
            self.message.set("There is already a machine there.");
            return;
        }

        cost.pay(&mut self.inventory);
        self.machines
            .push(Machine::new(self.current_world, tile, machine_kind));
        self.message
            .set(format!("Placed {}.", self.selected_build.name()));
    }

    fn place_machine(&mut self, cost: Cost, kind: MachineKind) {
        let tile = self.find_open_build_tile();
        if self.tile_is_occupied(tile) {
            self.message.set("That build tile is occupied.");
            return;
        }

        cost.pay(&mut self.inventory);
        self.machines
            .push(Machine::new(self.current_world, tile, kind));
        self.message
            .set(format!("Placed {}.", self.selected_build.name()));
    }

    fn place_structure(&mut self, cost: Cost) {
        let Some(kind) = self.selected_build.structure_kind() else {
            return;
        };
        let tile = self.find_open_build_tile();
        if self.tile_is_occupied(tile) {
            self.message.set("That build tile is occupied.");
            return;
        }

        cost.pay(&mut self.inventory);
        self.structures.push(Structure {
            world: self.current_world,
            tile,
            kind,
        });
        self.message
            .set(format!("Built {}.", self.selected_build.name()));
    }

    fn nearest_resource_tile(&self) -> Option<(IVec2, TileKind)> {
        let origin = tile_from_position(self.player.position);
        let mut best: Option<(IVec2, TileKind, i32)> = None;

        for y in origin.y - 3..=origin.y + 3 {
            for x in origin.x - 3..=origin.x + 3 {
                let tile = ivec2(x, y);
                if self.harvested.contains(&self.tile_key(tile)) {
                    continue;
                }

                let kind = self.current_world().tile(tile).kind;
                if !kind.is_resource() {
                    continue;
                }

                let distance = (origin.x - x).abs() + (origin.y - y).abs();
                if best.map_or(true, |(_, _, best_distance)| distance < best_distance) {
                    best = Some((tile, kind, distance));
                }
            }
        }

        best.map(|(tile, kind, _)| (tile, kind))
    }

    fn find_open_build_tile(&self) -> IVec2 {
        let player_tile = tile_from_position(self.player.position);
        let candidates = [
            player_tile + ivec2(1, 0),
            player_tile + ivec2(0, 1),
            player_tile + ivec2(-1, 0),
            player_tile + ivec2(0, -1),
            player_tile,
        ];

        candidates
            .into_iter()
            .find(|tile| !self.tile_is_occupied(*tile))
            .unwrap_or(player_tile)
    }

    fn tile_is_occupied(&self, tile: IVec2) -> bool {
        self.machine_at_current_world(tile).is_some()
            || self.structure_at_current_world(tile).is_some()
            || self
                .player
                .position
                .distance(tile.as_vec2() + vec2(0.5, 0.5))
                < 0.75
    }

    fn tile_key(&self, tile: IVec2) -> TileKey {
        (self.current_world.0, tile.x, tile.y)
    }
}

fn toggle_panel(current: MenuPanel, requested: MenuPanel) -> MenuPanel {
    if current == requested {
        MenuPanel::None
    } else {
        requested
    }
}

fn hotbar_keys() -> [KeyCode; 9] {
    [
        KeyCode::Key1,
        KeyCode::Key2,
        KeyCode::Key3,
        KeyCode::Key4,
        KeyCode::Key5,
        KeyCode::Key6,
        KeyCode::Key7,
        KeyCode::Key8,
        KeyCode::Key9,
    ]
}

fn clamp_length(value: Vec2, max: f32) -> Vec2 {
    if value.length_squared() > max * max {
        value.normalize() * max
    } else {
        value
    }
}
