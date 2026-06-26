use crate::{
    game::{BuildKind, Cost, Game, MenuPanel, Mode},
    input,
};
use macroquad::prelude::*;

pub fn draw(game: &Game) {
    draw_status(game);
    draw_settings_button();
    draw_hotbar(game);
    draw_message(game);

    match game.active_menu {
        MenuPanel::Inventory => draw_inventory_menu(game),
        MenuPanel::Crafting => draw_crafting_menu(game),
        MenuPanel::Settings => draw_settings_overlay(),
        MenuPanel::None => {}
    }
}

fn draw_status(game: &Game) {
    let world = game.current_world();
    panel_alpha(22.0, 20.0, 372.0, 70.0, 0.58);
    text(
        world.name(),
        40.0,
        52.0,
        29,
        Color::from_rgba(248, 248, 226, 235),
    );

    let location = match game.mode {
        Mode::World => format!(
            "HP {:03}  Food {:03}  Sol {:02}  Rot {:03}s",
            game.survival.health.round() as i32,
            game.survival.hunger.round() as i32,
            (game.local_day_time() * 100.0).round() as i32,
            game.current_world().rotation_period_seconds().round() as i32
        ),
        Mode::Space => "Quasar drift".to_owned(),
    };
    text(
        &location,
        40.0,
        78.0,
        19,
        Color::from_rgba(206, 222, 214, 225),
    );
}

fn draw_hotbar(game: &Game) {
    let slot = 46.0;
    let gap = 5.0;
    let width = BuildKind::ALL.len() as f32 * slot + (BuildKind::ALL.len() - 1) as f32 * gap;
    let x0 = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - slot - 18.0;

    for (index, kind) in BuildKind::ALL.into_iter().enumerate() {
        let x = x0 + index as f32 * (slot + gap);
        let selected = game.selected_build == kind;
        draw_rectangle(
            x,
            y,
            slot,
            slot,
            if selected {
                Color::new(0.09, 0.105, 0.12, 0.78)
            } else {
                Color::new(0.03, 0.04, 0.05, 0.44)
            },
        );
        draw_rectangle_lines(
            x + 0.5,
            y + 0.5,
            slot - 1.0,
            slot - 1.0,
            if selected { 2.5 } else { 1.0 },
            if selected {
                Color::from_rgba(248, 214, 136, 230)
            } else {
                Color::from_rgba(170, 188, 182, 120)
            },
        );
        draw_build_icon(kind, x + slot * 0.5, y + slot * 0.5, 0.82);
        text(
            &(index + 1).to_string(),
            x + 4.0,
            y + 13.0,
            14,
            Color::from_rgba(230, 236, 220, 160),
        );
    }
}

fn draw_inventory_menu(game: &Game) {
    let w = 720.0_f32.min(screen_width() - 48.0).max(320.0);
    let h = 460.0;
    let x = screen_width() * 0.5 - w * 0.5;
    let y = screen_height() * 0.5 - h * 0.5;

    dim();
    panel(x, y, w, h);
    text(
        "Inventory",
        x + 36.0,
        y + 56.0,
        38,
        Color::from_rgba(248, 248, 226, 255),
    );
    text(
        "Materials gathered from planets and machines.",
        x + 38.0,
        y + 88.0,
        20,
        Color::from_rgba(184, 205, 198, 255),
    );

    let columns = if w < 560.0 { 2 } else { 3 };
    let slot_w = ((w - 88.0) / columns as f32 - 18.0).max(130.0);
    for (index, item) in inventory_items(game).into_iter().enumerate() {
        let col = index % columns;
        let row = index / columns;
        let sx = x + 44.0 + col as f32 * (slot_w + 18.0);
        let sy = y + 126.0 + row as f32 * 94.0;
        draw_item_slot(sx, sy, slot_w, item.0, item.1, item.2);
    }
}

fn draw_crafting_menu(game: &Game) {
    let columns = input::crafting_columns();
    let w = input::crafting_panel_width(columns);
    let h = input::crafting_panel_height(columns);
    let x = screen_width() * 0.5 - w * 0.5;
    let y = screen_height() * 0.5 - h * 0.5;

    dim();
    panel(x, y, w, h);
    text(
        "Crafting",
        x + 34.0,
        y + 56.0,
        38,
        Color::from_rgba(248, 248, 226, 255),
    );
    text(
        "Choose what your hotbar places. Materials are spent when you build.",
        x + 36.0,
        y + 88.0,
        20,
        Color::from_rgba(184, 205, 198, 255),
    );

    for (index, kind) in BuildKind::ALL.into_iter().enumerate() {
        let col = index % columns;
        let row = index / columns;
        let card_x = x + 34.0 + col as f32 * 270.0;
        let card_y = y + 92.0 + row as f32 * 110.0;
        draw_recipe_card(
            kind,
            kind.cost(),
            game.selected_build == kind,
            kind.cost().can_pay(&game.inventory),
            card_x,
            card_y,
        );
    }
}

fn draw_settings_overlay() {
    let w = 620.0;
    let h = 440.0;
    let x = screen_width() * 0.5 - w * 0.5;
    let y = screen_height() * 0.5 - h * 0.5;

    dim();
    panel(x, y, w, h);
    text(
        "Settings",
        x + 40.0,
        y + 58.0,
        38,
        Color::from_rgba(248, 248, 226, 255),
    );
    text(
        "Controls",
        x + 40.0,
        y + 108.0,
        27,
        Color::from_rgba(172, 211, 202, 255),
    );

    let controls = [
        ("Move", "WASD / Arrow Keys"),
        ("Harvest / Mine", "E"),
        ("Inventory", "I"),
        ("Crafting", "C"),
        ("Select Hotbar", "1-9 / Click Slot"),
        ("Place Hotbar Item", "B"),
        ("Zoom", "Mouse Wheel"),
        ("Beacon / Descend", "Space"),
        ("Settings", "F1 / ? Button"),
        ("Close Menu", "Escape"),
    ];

    for (index, (action, binding)) in controls.into_iter().enumerate() {
        let row_y = y + 142.0 + index as f32 * 31.0;
        text(
            action,
            x + 58.0,
            row_y,
            22,
            Color::from_rgba(232, 238, 222, 255),
        );
        text(
            binding,
            x + 330.0,
            row_y,
            22,
            Color::from_rgba(248, 214, 136, 255),
        );
    }
}

fn draw_settings_button() {
    let x = screen_width() - 56.0;
    let y = 18.0;

    draw_rectangle(x, y, 38.0, 38.0, Color::new(0.025, 0.038, 0.058, 0.52));
    draw_rectangle_lines(
        x + 0.5,
        y + 0.5,
        37.0,
        37.0,
        1.5,
        Color::from_rgba(170, 188, 182, 150),
    );
    text(
        "?",
        x + 13.0,
        y + 28.0,
        25,
        Color::from_rgba(238, 240, 222, 210),
    );
}

fn draw_message(game: &Game) {
    if game.message.timer <= 0.0 || game.active_menu != MenuPanel::None {
        return;
    }

    let alpha = game.message.timer.min(1.0);
    panel_alpha(24.0, 106.0, 760.0, 44.0, 0.58 * alpha);
    text_alpha(
        &game.message.text,
        42.0,
        135.0,
        22,
        Color::new(0.92, 0.96, 0.88, alpha),
    );
}

fn inventory_items(game: &Game) -> [(&'static str, u32, Color); 7] {
    [
        (
            "Wood",
            game.inventory.wood,
            Color::from_rgba(126, 218, 143, 255),
        ),
        (
            "Stone",
            game.inventory.stone,
            Color::from_rgba(171, 185, 184, 255),
        ),
        (
            "Copper Ore",
            game.inventory.copper_ore,
            Color::from_rgba(234, 137, 82, 255),
        ),
        (
            "Coal",
            game.inventory.coal,
            Color::from_rgba(86, 96, 105, 255),
        ),
        (
            "Copper Plate",
            game.inventory.copper_plate,
            Color::from_rgba(242, 207, 124, 255),
        ),
        (
            "Copper Gear",
            game.inventory.copper_gear,
            Color::from_rgba(224, 178, 104, 255),
        ),
        (
            "Crystal",
            game.inventory.crystal,
            Color::from_rgba(104, 232, 211, 255),
        ),
    ]
}

fn draw_item_slot(x: f32, y: f32, w: f32, label: &str, count: u32, color: Color) {
    draw_rectangle(x, y, w, 72.0, Color::new(0.045, 0.055, 0.07, 0.88));
    draw_rectangle_lines(
        x + 0.5,
        y + 0.5,
        w - 1.0,
        71.0,
        1.5,
        Color::from_rgba(118, 144, 139, 180),
    );
    draw_rectangle(x + 16.0, y + 17.0, 38.0, 38.0, color);
    text(
        label,
        x + 68.0,
        y + 31.0,
        21,
        Color::from_rgba(238, 240, 222, 255),
    );
    text(
        &count.to_string(),
        x + 68.0,
        y + 56.0,
        24,
        Color::from_rgba(248, 214, 136, 255),
    );
}

fn draw_recipe_card(kind: BuildKind, cost: Cost, selected: bool, affordable: bool, x: f32, y: f32) {
    let border = if selected {
        Color::from_rgba(248, 214, 136, 255)
    } else if affordable {
        Color::from_rgba(112, 154, 145, 255)
    } else {
        Color::from_rgba(89, 96, 104, 255)
    };

    draw_rectangle(x, y, 246.0, 86.0, Color::new(0.045, 0.055, 0.07, 0.9));
    draw_rectangle_lines(
        x + 0.5,
        y + 0.5,
        245.0,
        85.0,
        if selected { 3.0 } else { 1.5 },
        border,
    );
    draw_build_icon(
        kind,
        x + 28.0,
        y + 43.0,
        if affordable { 1.0 } else { 0.42 },
    );
    text(
        kind.name(),
        x + 58.0,
        y + 32.0,
        22,
        Color::from_rgba(238, 240, 222, 255),
    );
    text(
        &cost.summary(),
        x + 58.0,
        y + 62.0,
        17,
        if affordable {
            Color::from_rgba(188, 213, 202, 255)
        } else {
            Color::from_rgba(196, 111, 104, 255)
        },
    );
}

fn draw_build_icon(kind: BuildKind, x: f32, y: f32, alpha: f32) {
    let color = match kind {
        BuildKind::Miner => Color::new(0.45, 0.86, 0.78, alpha),
        BuildKind::Furnace => Color::new(1.0, 0.42, 0.16, alpha),
        BuildKind::Belt => Color::new(0.86, 0.68, 0.32, alpha),
        BuildKind::Assembler => Color::new(0.64, 0.74, 0.95, alpha),
        BuildKind::DeepMiner => Color::new(0.54, 0.98, 0.86, alpha),
        BuildKind::PowerRelay => Color::new(0.62, 0.94, 1.0, alpha),
        BuildKind::StoneBlock => Color::new(0.66, 0.71, 0.7, alpha),
        BuildKind::TimberFrame => Color::new(0.58, 0.38, 0.22, alpha),
        BuildKind::GlowLamp => Color::new(0.42, 0.95, 0.86, alpha),
    };

    draw_circle(
        x,
        y,
        17.0,
        Color::new(color.r, color.g, color.b, 0.18 * alpha),
    );
    draw_rectangle(x - 11.0, y - 11.0, 22.0, 22.0, color);
}

fn dim() {
    draw_rectangle(
        0.0,
        0.0,
        screen_width(),
        screen_height(),
        Color::new(0.0, 0.0, 0.0, 0.54),
    );
}

fn panel(x: f32, y: f32, w: f32, h: f32) {
    panel_alpha(x, y, w, h, 0.86);
}

fn panel_alpha(x: f32, y: f32, w: f32, h: f32, alpha: f32) {
    draw_rectangle(
        x + 4.0,
        y + 5.0,
        w,
        h,
        Color::new(0.0, 0.0, 0.0, 0.22 * alpha),
    );
    draw_rectangle(x, y, w, h, Color::new(0.025, 0.038, 0.058, alpha));
    draw_rectangle_lines(
        x + 0.5,
        y + 0.5,
        w - 1.0,
        h - 1.0,
        2.0,
        Color::new(0.58, 0.74, 0.7, alpha),
    );
}

fn text(value: &str, x: f32, y: f32, size: u16, color: Color) {
    text_alpha(value, x, y, size, color);
}

fn text_alpha(value: &str, x: f32, y: f32, size: u16, color: Color) {
    draw_text_ex(
        value,
        x + 2.0,
        y + 2.0,
        TextParams {
            font_size: size,
            color: Color::new(0.0, 0.0, 0.0, color.a * 0.84),
            ..Default::default()
        },
    );
    draw_text_ex(
        value,
        x,
        y,
        TextParams {
            font_size: size,
            color,
            ..Default::default()
        },
    );
}
