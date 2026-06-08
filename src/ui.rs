use crate::assets::GameAssets;
use crate::state::GameState;
use macroquad::prelude::*;

pub fn draw_start_screen(
    assets: &GameAssets,
    play_btn_tex: &Texture2D,
    state: &mut GameState,
    mouse_world: Vec2,
    mouse_clicked: bool,
) {
    let virtual_width = 800.0;

    // 1. Draw centered graphic logo title banner inside virtual bounds
    let logo_w = 500.0;
    let logo_h = 180.0;
    let logo_x = (virtual_width - logo_w) / 2.0;
    let logo_y = 40.0;

    draw_texture_ex(
        &assets.logo_texture,
        logo_x,
        logo_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(logo_w, logo_h)),
            ..Default::default()
        },
    );

    // 2. Setup responsive Play Button virtual boundaries
    let btn_w = 120.0;
    let btn_h = 120.0;
    let btn_x = (virtual_width - btn_w) / 2.0;
    let btn_y = 240.0;

    // Render your green play button directly inside the camera grid layout
    draw_texture_ex(
        play_btn_tex,
        btn_x,
        btn_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(btn_w, btn_h)),
            ..Default::default()
        },
    );

    // Intercept clicks using screen-to-world mapped coordinates
    if mouse_clicked
        && mouse_world.x >= btn_x
        && mouse_world.x <= btn_x + btn_w
        && mouse_world.y >= btn_y
        && mouse_world.y <= btn_y + btn_h
    {
        *state = GameState::Playing;
    }
}

pub fn draw_game_over_screen(
    _assets: &GameAssets,
    play_btn_tex: &Texture2D,
    state: &mut GameState,
    score: i32,
    mouse_world: Vec2,
    mouse_clicked: bool,
) {
    let virtual_width = 800.0;

    // 1. Draw "GAME OVER" text cleanly centered
    let game_over_text = "GAME OVER";
    let text_size = 60;
    let text_measured = measure_text(game_over_text, None, text_size, 1.0);
    draw_text_ex(
        game_over_text,
        (virtual_width - text_measured.width) / 2.0,
        130.0,
        TextParams {
            font_size: text_size,
            color: RED,
            ..Default::default()
        },
    );

    // 2. Draw score text centered
    let score_text = format!("Score: {}", score);
    let score_size = 32;
    let score_measured = measure_text(&score_text, None, score_size, 1.0);
    draw_text_ex(
        &score_text,
        (virtual_width - score_measured.width) / 2.0,
        190.0,
        TextParams {
            font_size: score_size,
            color: WHITE,
            ..Default::default()
        },
    );

    // 3. Draw responsive Restart Button
    let btn_w = 120.0;
    let btn_h = 120.0;
    let btn_x = (virtual_width - btn_w) / 2.0;
    let btn_y = 250.0;

    draw_texture_ex(
        play_btn_tex,
        btn_x,
        btn_y,
        WHITE,
        DrawTextureParams {
            dest_size: Some(vec2(btn_w, btn_h)),
            ..Default::default()
        },
    );

    // Check if the restart click/touch falls within the virtual space bounding box
    if mouse_clicked
        && mouse_world.x >= btn_x
        && mouse_world.x <= btn_x + btn_w
        && mouse_world.y >= btn_y
        && mouse_world.y <= btn_y + btn_h
    {
        *state = GameState::Playing;
    }
}
