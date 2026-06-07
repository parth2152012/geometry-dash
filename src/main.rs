// src/main.rs

mod lvl_gen;
mod obstacles;

use macroquad::{
    color,
    prelude::*,
    ui::{Skin, hash, root_ui, widgets::Window},
};
use obstacles::{Spike, SpikeType};

const GRAVITY: f32 = 1200.;
const JUMP_FORCE: f32 = -450.;
const FORWARD_SPEED: f32 = 300.;
const CUBE_SIZE: f32 = 40.;
const GROUND_Y: f32 = 400.;

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
    let mut custom_font = load_ttf_font("assets/geometry-dash-font.ttf")
        .await
        .unwrap();
    let play_btn = load_image("assets/play_btn.png").await.unwrap();

    texture_bg.set_filter(FilterMode::Nearest);
    custom_font.set_filter(FilterMode::Nearest);

    let mut player = Player {
        x: 100.,
        y: GROUND_Y - CUBE_SIZE,
        vy: 0.,
        rotation: 0.,
        is_grounded: true,
    };

    let mut spikes: Vec<Spike> = Vec::new();
    let mut next_spawn_x = 800.;
    let mut game_over = false;
    let mut state = GameState::StartScreen;

    let ui_theme_skin: Skin = {
        // Keep your label_style and button_style exactly as they are now!
        let label_style = root_ui()
            .style_builder()
            .with_font(&custom_font)
            .unwrap()
            .font_size(50)
            .text_color(Color {
                r: (159.),
                g: (255.),
                b: (0.),
                a: (0.66),
            })
            .build();

        let button_style = root_ui()
            .style_builder()
            .background(play_btn.clone()) // Normal state image
            .background_hovered(play_btn.clone()) // Hovered state image
            .background_clicked(play_btn.clone()) // Pressed down state image
            .text_color(BLANK) // Hide any text overlays
            .build();

        // Add this new style layer block to clear out window background visuals:
        let window_style = root_ui()
            .style_builder()
            .background_margin(macroquad::math::RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .margin(macroquad::math::RectOffset::new(0.0, 0.0, 0.0, 0.0))
            .text_color(BLANK) // Hides the standard top text string header title
            .color(BLANK) // Turns the solid inner window background box invisible
            .color_clicked(BLANK)
            .color_hovered(BLANK)
            .build();

        Skin {
            label_style,
            button_style,
            window_style, // <-- Inject your clean invisible canvas profile layer here!
            ..root_ui().default_skin()
        }
    };

    loop {
        clear_background(DARKGRAY);

        // ALWAYS draw background assets first so they render under UI layers!
        draw_texture_ex(
            &texture_bg,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        match state {
            GameState::StartScreen => {
                // 1. Enlarge container canvas size so text strings do not wrap or clip off edges
                let window_width = 600.0;
                let window_height = 300.0;
                let window_size = vec2(window_width, window_height);

                let window_pos = vec2(
                    (screen_width() - window_width) / 2.0,
                    (screen_height() - window_height) / 2.0,
                );

                root_ui().push_skin(&ui_theme_skin);

                Window::new(macroquad::ui::hash!("main_menu"), window_pos, window_size)
                    .movable(false)
                    .titlebar(false)
                    .ui(&mut *root_ui(), |ui| {
                        // Keep your centered title text label
                        ui.label(vec2(20.0, 40.0), "GEOMETRY DASH");

                        // Use the structured widget builder to force a custom width and height
                        let play_clicked = macroquad::ui::widgets::Button::new("")
                            .size(vec2(120.0, 120.0)) // <-- Forces the button to be 120x120 pixels!
                            .position(vec2(220.0, 140.0))
                            .ui(ui); // Passes the UI context to render it

                        // Process the interaction state condition
                        if play_clicked {
                            state = GameState::Playing;
                        }
                    });

                root_ui().pop_skin();
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

                    if next_spawn_x - player.x < 1500.0 {
                        let (new_spikes, updated_x) =
                            lvl_gen::generate_random_chunk(next_spawn_x, GROUND_Y);
                        spikes.extend(new_spikes);
                        next_spawn_x = updated_x;
                    }

                    // --- COLLISION MATRIX ---
                    for spike in &spikes {
                        let hb_w = spike.w * 0.20;
                        let hb_h = spike.h * 0.50;
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
