mod lvl_gen;
mod obstacles;

use obstacles::Spike;

use macroquad::prelude::*;

// --- CONFIGURATION CONSTANTS ---
const GRAVITY: f32 = 1200.0;
const JUMP_FORCE: f32 = -450.0;
const FORWARD_SPEED: f32 = 300.0;
const CUBE_SIZE: f32 = 40.0;
const GROUND_Y: f32 = 400.0;

struct Player {
    x: f32,
    y: f32,
    vy: f32,
    rotation: f32,
    is_grounded: bool,
}

#[macroquad::main("Geometry Dash Fixed Hitboxes")]
async fn main() {
    let mut player = Player {
        x: 100.0,
        y: GROUND_Y - CUBE_SIZE,
        vy: 0.0,
        rotation: 0.0,
        is_grounded: true,
    };

    // Remove this too will go in lvl_gen

    let spikes = vec![Spike::new(1000.0, GROUND_Y, 45.0, 50.0)];

    let mut game_over = false;
    let mut next_spawn_x = 1000.0; // Start spawning the first chunks further down the track

    loop {
        // --- UPDATE LOOP ---
        if !game_over {
            let dt = get_frame_time();

            // Constant forward movement
            player.x += FORWARD_SPEED * dt;

            // Handle Input
            if (is_key_down(KeyCode::Space) || is_mouse_button_down(MouseButton::Left))
                && player.is_grounded
            {
                player.vy = JUMP_FORCE;
                player.is_grounded = false;
            }

            // Apply gravity
            player.vy += GRAVITY * dt;
            player.y += player.vy * dt;

            // Strict floor collision
            if player.y >= GROUND_Y - CUBE_SIZE {
                player.y = GROUND_Y - CUBE_SIZE;
                player.vy = 0.0;
                player.is_grounded = true;
                player.rotation = (player.rotation / 90.0).round() * 90.0;
            }

            // Smooth rotation while in air
            if !player.is_grounded {
                player.rotation += 360.0 * dt;
            }

            // --- GEOMETRY DASH HITBOX LOGIC ---  lvl_gen
            for spike in &spikes {
                // Sizing matching the nested red hitboxes from the reference image
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
        } else {
            if is_key_pressed(KeyCode::Space) {
                player.x = 100.0;
                player.y = GROUND_Y - CUBE_SIZE;
                player.vy = 0.0;
                player.rotation = 0.0;
                player.is_grounded = true;
                game_over = false;
            }
        }

        if next_spawn_x - player.x < 800.0 {
            // 1. Call your function to get new spikes and the advanced cursor coordinate
            let (new_spikes, updated_x) = lvl_gen::generate_random_chunk(next_spawn_x, GROUND_Y);

            // 2. Append the new spikes to your main game loop array
            spikes.extend(new_spikes);

            // 3. Update your tracking pen so it never spawns backwards!
            next_spawn_x = updated_x;
        }

        // --- RENDER LOOP ---
        clear_background(BLACK);

        let camera_offset_x = player.x - 150.0;

        // Draw Ground Floor Line
        draw_line(0.0, GROUND_Y, screen_width(), GROUND_Y, 4.0, WHITE);

        // Draw Spikes
        for spike in &spikes {
            draw_triangle(
                vec2(spike.x - camera_offset_x, spike.y),
                vec2(
                    spike.x + (spike.w / 2.0) - camera_offset_x,
                    spike.y - spike.h,
                ),
                vec2(spike.x + spike.w - camera_offset_x, spike.y),
                RED,
            );
        }

        // --- NATIVE DRAWING FIXED PIVOT ---
        // Find the absolute center of our block in screen-space
        let screen_x = player.x - camera_offset_x;
        let center_x = screen_x + (CUBE_SIZE / 2.0);
        let center_y = player.y + (CUBE_SIZE / 2.0);

        draw_rectangle_ex(
            center_x,
            center_y,
            CUBE_SIZE,
            CUBE_SIZE,
            DrawRectangleParams {
                offset: vec2(0.5, 0.5), // Forces rotation to pivot from center
                rotation: player.rotation.to_radians(),
                color: BLUE,
            },
        );

        // Draw UI Elements
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

        next_frame().await
    }
}
