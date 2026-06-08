#[cfg(target_arch = "wasm32")]
extern crate macroquad;

use macroquad::prelude::*;

// Declare and structure all separate file components in project scope
mod assets;
mod lvl_gen;
mod obstacles;
mod player;
mod state;
mod ui;

use assets::GameAssets;
use obstacles::Spike;
use player::Player;
use state::GameState;

#[cfg(target_arch = "wasm32")]
#[no_mangle]
pub unsafe extern "C" fn __getrandom_v02_custom(dest: *mut u8, len: usize) -> i32 {
    let slice = std::slice::from_raw_parts_mut(dest, len);
    for byte in slice.iter_mut() {
        *byte = (macroquad::rand::rand() & 0xFF) as u8;
    }
    0 // Returns 0 to indicate success
}

#[macroquad::main("Geometry Dash Complete")]
async fn main() {
    // 1. Load and package all game image textures crisply via separate module
    let assets = GameAssets::load().await;

    // 2. Generate custom UI rendering profile skin properties blueprint sheet
    let menu_skin = ui::create_menu_skin(&assets);

    // 3. Initialize mutable entities and vector trackers
    let mut state = GameState::StartScreen;
    let mut player = Player::new();
    let mut spikes: Vec<Spike> = Vec::new();
    let mut next_spawn_x = 800.0;
    let mut score = 0;
    let mut game_speed = 6.0;
    let mut difficulty = 0.0;

    let floor_line_y = 400.0;

    loop {
        clear_background(DARKGRAY);

        // Always render your game's scrolling background texture frame layer first
        draw_texture_ex(
            &assets.bg_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );

        // Only render floor line during gameplay (not on menus)
        if state == GameState::Playing {
            draw_line(0.0, floor_line_y, screen_width(), floor_line_y, 4.0, WHITE);
        }

        match state {
            GameState::StartScreen => {
                // Delegate rendering work to your UI script, checking for play state modifications
                let mut current_state = state;
                ui::draw_start_screen(&assets, &menu_skin, &mut current_state);

                // If the user tapped play inside separate module, seed obstacles and shift states
                if current_state == GameState::Playing {
                    // Reset running variables fresh upon clean launch trigger action click
                    player = Player::new();
                    spikes.clear();
                    next_spawn_x = 800.0;
                    score = 0;
                    game_speed = 6.0;
                    difficulty = 0.0;
                    lvl_gen::spawn_next_chunk(&mut spikes, &mut next_spawn_x, difficulty);
                    state = GameState::Playing;
                }
            }
            GameState::Playing => {
                // 1. Update the player's gravity, jump, and rotation physics
                player.update(floor_line_y);

                // 2. Check if we need to generate more spikes ahead of the player
                if spikes.is_empty() || spikes.last().unwrap().x < screen_width() + 200.0 {
                    lvl_gen::spawn_next_chunk(&mut spikes, &mut next_spawn_x, difficulty);
                }

                // 3. Move all active obstacles left across the screen with increasing speed
                for spike in spikes.iter_mut() {
                    spike.x -= game_speed;
                }

                // 4. Increment score based on passed spikes and update difficulty
                for spike in spikes.iter() {
                    // Award 10 points when spike passes the player's X position
                    if spike.x < player.x && spike.x > player.x - 10.0 {
                        score += 10;
                        // Increase difficulty every 100 points (every 10 spikes passed)
                        difficulty = (score / 100) as f32 * 50.0;
                        // Increase game speed: starts at 6.0, caps at 12.0
                        game_speed = 6.0 + (difficulty / 100.0).min(6.0);
                    }
                }

                // 5. Check for crashes between the player and any spike hazard
                let player_rect = player.get_rect();
                for spike in spikes.iter() {
                    let spike_rect = Rect::new(
                        spike.x + 8.0,
                        spike.y + 5.0,
                        spike.width - 16.0,
                        spike.height - 5.0,
                    );

                    // Using Macroquad's native overlaps() method for clean collision calculation
                    if player_rect.overlaps(&spike_rect) {
                        // Crash! Send the player to game over screen
                        state = GameState::GameOver;
                        break;
                    }
                }

                // 6. Clean up old offscreen obstacles to prevent memory leaks
                spikes.retain(|s| s.x > -100.0);

                // 7. Draw all active spikes
                for spike in spikes.iter() {
                    spike.draw(&assets.spike_small_tex, &assets.spike_tall_tex);
                }

                // 8. Draw the player cube icon
                player.draw(&assets.player_tex);

                // 9. Draw score and difficulty on screen during gameplay
                draw_text_ex(
                    &format!("Score: {}", score),
                    10.0,
                    30.0,
                    TextParams {
                        font_size: 32,
                        color: WHITE,
                        ..Default::default()
                    },
                );

                let difficulty_level = (difficulty / 50.0) as i32 + 1;
                draw_text_ex(
                    &format!("Level: {}", difficulty_level),
                    10.0,
                    70.0,
                    TextParams {
                        font_size: 24,
                        color: YELLOW,
                        ..Default::default()
                    },
                );

                // 10. Press ESC to return to the menu manually
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::StartScreen;
                }
            }
            GameState::GameOver => {
                // Display game over screen with restart button
                let mut current_state = state;
                ui::draw_game_over_screen(&assets, &menu_skin, &mut current_state, score);

                // If user clicks restart button, reset the game
                if current_state == GameState::Playing {
                    player = Player::new();
                    spikes.clear();
                    next_spawn_x = 800.0;
                    score = 0;
                    game_speed = 6.0;
                    difficulty = 0.0;
                    lvl_gen::spawn_next_chunk(&mut spikes, &mut next_spawn_x, difficulty);
                    state = GameState::Playing;
                }

                // Allow pressing ESC to return to start menu
                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::StartScreen;
                }
            }
        }

        next_frame().await;
    }
}
