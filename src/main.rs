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

struct Particle {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    color: Color,
    lifetime: f32,
    size: f32,
}

#[macroquad::main("Geometry Dash Complete")]
async fn main() {
    // Seed random number generator tracking from system epochs
    macroquad::rand::srand(macroquad::time::get_time() as u64);

    // Establish fixed virtual resolution spaces to make layout fully responsive
    let virtual_width = 800.0;
    let virtual_height = 450.0;

    // 1. Load and package all game image textures crisply via separate module
    let assets = GameAssets::load().await;

    // Convert the CPU image asset into a GPU-bound Texture2D once on startup
    let play_btn_tex = Texture2D::from_image(&assets.play_btn_img);

    // 2. Initialize mutable entities and vector trackers
    let mut state = GameState::StartScreen;
    let mut player = Player::new();
    let mut spikes: Vec<Spike> = Vec::new();
    let mut next_spawn_x = 800.0;
    let mut score = 0;
    let mut game_speed = 6.0;
    let mut difficulty = 0.0;

    // Tracker array for holding all explosive player fragments
    let mut particles: Vec<Particle> = Vec::new();

    let floor_line_y = 400.0;

    loop {
        // 1. Reset to standard screen camera space first and wipe the whole display monitor
        set_default_camera();
        clear_background(BLACK); // Creates the clean outer border space

        // 2. Build the camera projection matrix with corrected Y-down orientation
        let mut camera = Camera2D::from_display_rect(Rect::new(
            0.0,
            virtual_height,
            virtual_width,
            -virtual_height,
        ));

        // 3. Calculate aspect ratio fitting with a custom scale multiplier
        let target_aspect = virtual_width / virtual_height;
        let screen_w = screen_width();
        let screen_h = screen_height();
        let current_aspect = screen_w / screen_h;

        let (mut view_w, mut view_h) = if current_aspect > target_aspect {
            (screen_h * target_aspect, screen_h)
        } else {
            (screen_w, screen_w / target_aspect)
        };

        // 🎛️ SCALE CONTROL: Change 0.85 (85%) to make everything bigger or smaller overall!
        let game_scale_factor = 0.85;
        view_w *= game_scale_factor;
        view_h *= game_scale_factor;

        // Center the view box inside your current window limits
        let view_x = (screen_w - view_w) / 2.0;
        let view_y = (screen_h - view_h) / 2.0;

        // 🛠️ FIX: Convert floats to an i32 integer tuple block for the native viewport
        camera.viewport = Some((view_x as i32, view_y as i32, view_w as i32, view_h as i32));
        set_camera(&camera);

        // Gather window inputs and map screen positions directly into virtual game space coordinates
        let (mx, my) = mouse_position();
        let mouse_world = camera.screen_to_world(vec2(mx, my));
        let mouse_clicked = is_mouse_button_pressed(MouseButton::Left);

        // Always render your game's scrolling background texture scaled to virtual limits
        draw_texture_ex(
            &assets.bg_texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(virtual_width, virtual_height)),
                ..Default::default()
            },
        );

        // Only render floor line during gameplay (not on menus)
        if state == GameState::Playing {
            draw_line(0.0, floor_line_y, virtual_width, floor_line_y, 4.0, WHITE);
        }

        match state {
            GameState::StartScreen => {
                // Render menu layout utilizing safe world coordinates and texture map references
                ui::draw_start_screen(
                    &assets,
                    &play_btn_tex,
                    &mut state,
                    mouse_world,
                    mouse_clicked,
                );

                // If menu interactions switch state to playing, populate layout track elements
                if state == GameState::Playing {
                    player = Player::new();
                    spikes.clear();
                    particles.clear();
                    next_spawn_x = 800.0;
                    score = 0;
                    game_speed = 6.0;
                    difficulty = 0.0;
                    lvl_gen::spawn_next_chunk(&mut spikes, &mut next_spawn_x, difficulty);
                }
            }
            GameState::Playing => {
                // 1. Update the player's gravity, jump, and rotation physics
                player.update(floor_line_y);

                // 2. Check if we need to generate more spikes ahead using virtual width bounds
                if spikes.is_empty() || spikes.last().unwrap().x < virtual_width + 200.0 {
                    lvl_gen::spawn_next_chunk(&mut spikes, &mut next_spawn_x, difficulty);
                }

                // 3. Move all active obstacles left across the screen with increasing speed
                for spike in spikes.iter_mut() {
                    spike.x -= game_speed;
                }

                // 4. Increment score based on passed spikes and update difficulty
                for spike in spikes.iter() {
                    if spike.x < player.x && spike.x > player.x - 10.0 {
                        score += 10;
                        difficulty = (score / 100) as f32 * 50.0;
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

                    if player_rect.overlaps(&spike_rect) {
                        // 💥 Spawn fiery particle explosion around player center point coordinates
                        for _ in 0..35 {
                            particles.push(Particle {
                                x: player.x + 15.0,
                                y: player.y + 15.0,
                                vx: macroquad::rand::gen_range(-250.0, 250.0),
                                vy: macroquad::rand::gen_range(-450.0, 30.0),
                                color: Color::new(
                                    1.0,
                                    macroquad::rand::gen_range(0.1, 0.6),
                                    0.0,
                                    1.0,
                                ),
                                lifetime: macroquad::rand::gen_range(0.4, 0.9),
                                size: macroquad::rand::gen_range(4.0, 9.0),
                            });
                        }

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

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::StartScreen;
                }
            }
            GameState::GameOver => {
                // Draw game over elements passing clean world input maps and textures
                ui::draw_game_over_screen(
                    &assets,
                    &play_btn_tex,
                    &mut state,
                    score,
                    mouse_world,
                    mouse_clicked,
                );

                if state == GameState::Playing {
                    player = Player::new();
                    spikes.clear();
                    particles.clear();
                    next_spawn_x = 800.0;
                    score = 0;
                    game_speed = 6.0;
                    difficulty = 0.0;
                    lvl_gen::spawn_next_chunk(&mut spikes, &mut next_spawn_x, difficulty);
                }

                if is_key_pressed(KeyCode::Escape) {
                    state = GameState::StartScreen;
                }
            }
        }

        // 🎨 Process and render running particle behaviors inside camera viewport limits
        let dt = get_frame_time();
        particles.retain_mut(|p| {
            p.lifetime -= dt;
            p.x += p.vx * dt;
            p.y += p.vy * dt;
            p.vy += 1100.0 * dt; // Gravity

            let mut render_color = p.color;
            render_color.a = (p.lifetime / 0.9).clamp(0.0, 1.0); // Decay fade effect

            draw_rectangle(p.x, p.y, p.size, p.size, render_color);
            p.lifetime > 0.0
        });

        next_frame().await;
    }
}
