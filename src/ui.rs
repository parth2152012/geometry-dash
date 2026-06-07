use crate::assets::GameAssets;
use crate::state::GameState;
use macroquad::prelude::*;
use macroquad::ui::{Skin, hash, root_ui, widgets};

pub fn create_menu_skin(assets: &GameAssets) -> Skin {
    let label_style = root_ui().style_builder().text_color(WHITE).build();

    let button_style = root_ui()
        .style_builder()
        .background(assets.play_btn_img.clone())
        .background_hovered(assets.play_btn_img.clone())
        .background_clicked(assets.play_btn_img.clone())
        .text_color(BLANK) // Clear text strings overlapping your button image
        .build();

    let window_style = root_ui()
        .style_builder()
        .background_margin(macroquad::math::RectOffset::new(0.0, 0.0, 0.0, 0.0))
        .margin(macroquad::math::RectOffset::new(0.0, 0.0, 0.0, 0.0))
        .text_color(BLANK)
        .color(BLANK) // Removes the default dark grey background box window canvas frame
        .color_clicked(BLANK)
        .color_hovered(BLANK)
        .build();

    Skin {
        label_style,
        button_style,
        window_style,
        ..root_ui().default_skin()
    }
}

pub fn draw_start_screen(assets: &GameAssets, skin: &Skin, state: &mut GameState) {
    let win_w = 600.0;
    let win_h = 400.0;
    let win_pos = vec2(
        (screen_width() - win_w) / 2.0,
        (screen_height() - win_h) / 2.0,
    );

    // 1. Draw centered graphic logo title banner directly onto canvas
    let logo_w = 500.0;
    let logo_h = 180.0;
    draw_texture_ex(
        &assets.logo_texture,
        (screen_width() - logo_w) / 2.0,
        win_pos.y + 10.0,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(logo_w, logo_h)),
            ..Default::default()
        },
    );

    // 2. Open our completely transparent, non-movable click tracking menu container
    root_ui().push_skin(skin);

    widgets::Window::new(hash!("main_menu"), win_pos, vec2(win_w, win_h))
        .movable(false)
        .titlebar(false) // Eliminates the tiny thin border line element entirely
        .ui(&mut *root_ui(), |ui| {
            // Render your pixel-perfect green play button widget inside menu framework
            let play_clicked = widgets::Button::new("")
                .size(vec2(120.0, 120.0))
                .position(vec2(240.0, 220.0))
                .ui(ui);

            if play_clicked {
                *state = GameState::Playing;
            }
        });

    root_ui().pop_skin();
}

pub fn draw_game_over_screen(assets: &GameAssets, skin: &Skin, state: &mut GameState, score: i32) {
    let win_w = 600.0;
    let win_h = 400.0;
    let win_pos = vec2(
        (screen_width() - win_w) / 2.0,
        (screen_height() - win_h) / 2.0,
    );

    // 1. Draw "GAME OVER" text centered
    let game_over_text = "GAME OVER";
    let text_size = 80.0;
    let text_width = measure_text(game_over_text, None, text_size as u16, 1.0).width;
    draw_text_ex(
        game_over_text,
        (screen_width() - text_width) / 2.0,
        win_pos.y + 80.0,
        TextParams {
            font_size: text_size as u16,
            color: RED,
            ..Default::default()
        },
    );

    // 2. Draw score display
    let score_text = format!("Score: {}", score);
    let score_text_size = 40.0;
    let score_width = measure_text(&score_text, None, score_text_size as u16, 1.0).width;
    draw_text_ex(
        &score_text,
        (screen_width() - score_width) / 2.0,
        win_pos.y + 160.0,
        TextParams {
            font_size: score_text_size as u16,
            color: WHITE,
            ..Default::default()
        },
    );

    // 3. Draw restart button UI
    root_ui().push_skin(skin);

    widgets::Window::new(hash!("game_over_menu"), win_pos, vec2(win_w, win_h))
        .movable(false)
        .titlebar(false)
        .ui(&mut *root_ui(), |ui| {
            let restart_clicked = widgets::Button::new("")
                .size(vec2(120.0, 120.0))
                .position(vec2(240.0, 220.0))
                .ui(ui);

            if restart_clicked {
                *state = GameState::Playing;
            }
        });

    root_ui().pop_skin();
}
