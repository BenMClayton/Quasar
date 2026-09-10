# Quasar

A stylised isometric Rust game prototype about a universe where every world is connected by the space between them.

You begin on a strange generated world, gather resources, smelt copper, repair a beacon, launch into space, then drift between other signals and descend into new worlds. The current prototype is intentionally small, but it already has the shape of the core loop:

- Walk around generated isometric worlds with distinct palettes, biomes, elevation, and resources.
- Harvest wood, stone, copper, and coal.
- Open inventory and crafting menus.
- Choose recipes in the crafting menu, then place the selected hotbar item.
- Place miners, furnaces, assemblers, belts, deeper miners, power relays, stone blocks, timber frames, and glow lamps.
- Mine ore deposits repeatedly by hand or with machinery.
- Smelt copper plates and spend them to repair the local beacon.
- Fly through a connected star map.
- Approach another signal and press `Space` to land.

## Controls

- `WASD` or arrow keys: move
- `E`: harvest a nearby resource
- `I`: open inventory
- `C`: open crafting
- click a crafting recipe or press `1-9` while crafting is open: select a recipe for the hotbar
- press `1-9` or click a hotbar slot during play: change the selected hotbar item
- `B`: place the selected hotbar item
- mouse wheel: zoom the isometric camera
- `Space`: repair/launch from a beacon or descend near a world signal
- `F1` or the `?` button: open settings and controls
- `Escape`: close open menus

## Current Progression

1. Gather wood from forests and stone from surface rocks.
2. Select and place miners on stone, coal, copper, or crystal deposits.
3. Select and place a furnace.
4. Feed the furnace with copper ore and coal to make copper plates.
5. Build an assembler to turn copper plates into copper gears.
6. Use gears, plates, and crystals to craft deeper mining and power-tier machines.
7. Repair the beacon with copper plates and travel to the next world.

## Run

Requires a current stable Rust toolchain.

```sh
cargo run
```

## Verification

```sh
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
```

## Project Layout

- `src/main.rs`: tiny entry point.
- `src/app.rs`: owns the game and renderer.
- `src/config.rs`: screen, projection, and tuning constants.
- `src/input.rs`: player input mapping.
- `src/game/`: survival state, inventory, machines, travel, and update rules.
- `src/world/`: deterministic world generation, noise, tiles, and palettes.
- `src/render/`: isometric projection, terrain, entities, space, and UI.

## Direction

The long-term fantasy is "Minecraft worlds are connected, but space is the hallway." Quasar should lean into:

- stylised pixel-art presentation with readable scale, isometric depth, and rich atmosphere;
- wildly different procedural world identities;
- planet-specific solar days based on rotation speed and solar phase;
- travel as discovery, not menu selection;
- Minecraft-style survival escalation: hand tools, shelter, mining, crafting tiers, danger, and world mastery;
- Factorio-style machinery: extractors, belts, furnaces, power, item routing, logistics, and eventually inter-world supply lines;
- Rust-first systems that can grow from this 2D prototype toward chunked terrain, crafting, automation, or multiplayer.

## Status

Quasar is a playable systems prototype rather than a finished game. The current
repository focuses on a coherent exploration/crafting loop and readable Rust
module boundaries; saves, audio, controller support, and production content are
not yet implemented.
