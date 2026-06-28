use macroquad::prelude::*;

pub fn movement_axis() -> Vec2 {
    let mut direction = Vec2::ZERO;

    if is_key_down(KeyCode::A) || is_key_down(KeyCode::Left) {
        direction.x -= 1.0;
    }
    if is_key_down(KeyCode::D) || is_key_down(KeyCode::Right) {
        direction.x += 1.0;
    }
    if is_key_down(KeyCode::W) || is_key_down(KeyCode::Up) {
        direction.y -= 1.0;
    }
    if is_key_down(KeyCode::S) || is_key_down(KeyCode::Down) {
        direction.y += 1.0;
    }

    direction.normalize_or_zero()
}

pub fn clicked_crafting_slot() -> Option<usize> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let columns = crafting_columns();
    let panel_w = crafting_panel_width(columns);
    let panel_x = screen_width() * 0.5 - panel_w * 0.5;
    let panel_height = crafting_panel_height(columns);
    let panel_y = screen_height() * 0.5 - panel_height * 0.5;

    // Calculate actual number of rows that fit in the panel height to avoid checking non-existent slots
    const SLOT_HEIGHT: f32 = 110.0 + 86.0; // row spacing (110) + card content area (86)
    let max_rows = ((panel_height - 92.0) / SLOT_HEIGHT).floor() as usize;

    for index in 0..(columns * max_rows.min(9)) {
        let col = index % columns;
        let row = index / columns;
        let card_x = panel_x + 34.0 + col as f32 * 270.0;
        let card_y = panel_y + 92.0 + row as f32 * SLOT_HEIGHT;
        if mouse_x >= card_x
            && mouse_x <= card_x + 246.0
            && mouse_y >= card_y
            && mouse_y <= card_y + 86.0
        {
            return Some(index);
        }
    }

    None
}

pub fn scroll_delta() -> f32 {
    mouse_wheel().1
}

pub fn clicked_hotbar_slot() -> Option<usize> {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return None;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let slot = 46.0;
    let gap = 5.0;
    let count = 9;
    let width = count as f32 * slot + (count - 1) as f32 * gap;
    let x0 = screen_width() * 0.5 - width * 0.5;
    let y = screen_height() - slot - 18.0;

    for index in 0..count {
        let x = x0 + index as f32 * (slot + gap);
        if mouse_x >= x && mouse_x <= x + slot && mouse_y >= y && mouse_y <= y + slot {
            return Some(index);
        }
    }

    None
}

pub fn clicked_settings_button() -> bool {
    if !is_mouse_button_pressed(MouseButton::Left) {
        return false;
    }

    let (mouse_x, mouse_y) = mouse_position();
    let x = screen_width() - 56.0;
    let y = 18.0;

    mouse_x >= x && mouse_x <= x + 38.0 && mouse_y >= y && mouse_y <= y + 38.0
}

pub fn crafting_columns() -> usize {
    if screen_width() < 680.0 {
        1
    } else if screen_width() < 980.0 {
        2
    } else {
        3
    }
}

pub fn crafting_panel_width(columns: usize) -> f32 {
    (columns as f32 * 270.0 + 50.0)
        .min(screen_width() - 48.0)
        .max(320.0)
}

pub fn crafting_panel_height(columns: usize) -> f32 {
    if columns == 1 { 830.0_f32 } else { 500.0_f32 }
        .min(screen_height() - 48.0)
        .max(430.0)
}
