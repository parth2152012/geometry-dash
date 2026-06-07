// src/main.rs

mod lvl_gen;
mod obstacles;

use macroquad::prelude::*;
use obstacles::{Spike, SpikeType};

const GRAVITY: f32 = 1200.0;
const JUMP_FORCE: f32 = -450.0;
const FORWARD_SPEED: f32 = 300.0;
const CUBE_SIZE: f32 = 40.0;
const GROUND_Y: f32 = 400.0;

#[derive(PartialEq)]
enum GameState {
    StartScreen,
    Playing,
}

struct Player {
    x: f32,
    y: f32,
    vy: f32,
    rotation: f32,
    is_grounded: bool,
}

#[macroquad::main("Geometry Dash Textured")]
async fn main() {
    // --- ASSET LOADING LAYER ---
    // Safely parse textures out of your assets folder on initialization
    let texture_bg = load_texture("assets/bg.png").await.unwrap();
    let texture_player = load_texture("assets/player.png").await.unwrap();
    let texture_spike_small = load_texture("assets/spike_small.png").await.unwrap();
    let texture_spike_tall = load_texture("assets/spike_tall.png").await.unwrap();

    let mut player = Player {
        x: 100.0,
        y: GROUND_Y - CUBE_SIZE,
        vy: 0.0,
        rotation: 0.0,
        is_grounded: true,
    };

    let mut spikes: Vec<Spike> = Vec::new();
    let mut next_spawn_x = 800.0;
    let mut game_over = false;
    let mut state = GameState::StartScreen;

    loop {
        match state {
            GameState::StartScreen => {
                // Centered text
                let title_text = "MY AWESOME GAME";
                let font_size = 50.0;
                let text_size = measure_text(title_text, None, font_size as _, 1.0);

                draw_text(
                    title_text,
                    screen_width() / 2.0 - text_size.width / 2.0,
                    screen_height() / 2.0 - 50.0,
                    font_size,
                    WHITE,
                );

                // Flashing "Press Enter" prompt
                let time = get_time();
                if (time.sin() * 5.0) > 0.0 {
                    let prompt = "Press ENTER to Start";
                    let prompt_size = measure_text(prompt, None, 30, 1.0);
                    draw_text(
                        prompt,
                        screen_width() / 2.0 - prompt_size.width / 2.0,
                        screen_height() / 2.0 + 50.0,
                        30.0,
                        YELLOW,
                    );
                }

                // Transition state
                if is_key_pressed(KeyCode::Enter) {
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // --- UPDATE LOOP ---
                if !game_over {
                    let dt = get_frame_time();
                    player.x += FORWARD_SPEED * dt;

                    if (is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left))
                        && player.is_grounded
                    {
                        player.vy = JUMP_FORCE;
                        player.is_grounded = false;
                    }

                    player.vy += GRAVITY * dt;
                    player.y += player.vy * dt;

                    if player.y >= GROUND_Y - CUBE_SIZE {
                        player.y = GROUND_Y - CUBE_SIZE;
                        player.vy = 0.0;
                        player.is_grounded = true;
                        player.rotation = (player.rotation / 90.0).round() * 90.0;
                    }

                    if !player.is_grounded {
                        player.rotation += 360.0 * dt;
                    }

                    if next_spawn_x - player.x < 1000.0 {
                        let (new_spikes, updated_x) =
                            lvl_gen::generate_random_chunk(next_spawn_x, GROUND_Y);
                        spikes.extend(new_spikes);
                        next_spawn_x = updated_x;
                    }

                    // --- COLLISION MATRIX ---
                    for spike in &spikes {
                        let hb_w = spike.w * 0.35;
                        let hb_h = spike.h * 0.70;
                        let hb_x = spike.x + (spike.w - hb_w) / 2.0;
                        let hb_y = spike.y - hb_h;

                        let overlaps = player.x < hb_x + hb_w
                            && player.x + CUBE_SIZE > hb_x
                            && player.y < hb_y + hb_h
                            && player.y + CUBE_SIZE > hb_y;

                        if overlaps {
                            game_over = true;
                        }
                    }

                    let camera_offset_x = player.x - 150.0;
                    spikes.retain(|spike| spike.x + spike.w > camera_offset_x);
                } else {
                    if is_key_pressed(KeyCode::Space) {
                        player.x = 100.0;
                        player.y = GROUND_Y - CUBE_SIZE;
                        player.vy = 0.0;
                        player.rotation = 0.0;
                        player.is_grounded = true;
                        spikes.clear();
                        next_spawn_x = 800.0;
                        game_over = false;
                    }
                }

                // --- RENDER LOOP (BACK-TO-FRONT SORTING) ---
                clear_background(BLACK);
                let camera_offset_x = player.x - 150.0;

                // 1. Draw Parallax Scrolling Background
                let bg_scroll_speed = 0.15;
                let bg_x = -(camera_offset_x * bg_scroll_speed) % screen_width();

                draw_texture_ex(
                    &texture_bg,
                    bg_x,
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );
                draw_texture_ex(
                    &texture_bg,
                    bg_x + screen_width(),
                    0.0,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(screen_width(), screen_height())),
                        ..Default::default()
                    },
                );

                // 2. Draw Floor Horizon
                draw_line(0.0, GROUND_Y, screen_width(), GROUND_Y, 4.0, WHITE);

                // 3. Draw Textured Spikes
                for spike in &spikes {
                    // Pick appropriate texture binding pointer based on chunk variation enum flags
                    let current_texture = match spike.variant {
                        SpikeType::Small => &texture_spike_small,
                        SpikeType::Tall => &texture_spike_tall,
                    };

                    draw_texture_ex(
                        current_texture,
                        spike.x - camera_offset_x,
                        spike.y - spike.h, // Textures render down from Top-Left, so subtract height to seat it on the floor
                        WHITE,
                        DrawTextureParams {
                            dest_size: Some(vec2(spike.w, spike.h)),
                            ..Default::default()
                        },
                    );
                }

                // 4. Draw Animated Rotated Player Sprite Face
                let screen_x = player.x - camera_offset_x;

                draw_texture_ex(
                    &texture_player,
                    screen_x,
                    player.y,
                    WHITE,
                    DrawTextureParams {
                        dest_size: Some(vec2(CUBE_SIZE, CUBE_SIZE)),
                        rotation: player.rotation.to_radians(),
                        // Centers pivot transform framework around the texture frame center point
                        pivot: Some(vec2(screen_x + CUBE_SIZE / 2.0, player.y + CUBE_SIZE / 2.0)),
                        ..Default::default()
                    },
                );

                // 5. Draw UI Font Elements
                if game_over {
                    draw_text(
                        "CRASHED! Press SPACE to Restart",
                        screen_width() / 2.0 - 180.0,
                        screen_height() / 2.0,
                        30.0,
                        YELLOW,
                    );
                } else {
                    draw_text(
                        &format!("Distance: {:.0}", player.x),
                        20.0,
                        30.0,
                        20.0,
                        GREEN,
                    );
                }
            }
        }
        next_frame().await
    }
}
