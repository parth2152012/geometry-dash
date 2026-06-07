use macroquad::prelude::*;

pub struct Player {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
    pub velocity_y: f32,
    pub rotation: f32,
    pub is_grounded: bool,
}

impl Player {
    pub fn new() -> Self {
        Self {
            x: 150.0,
            y: 350.0, // Spawns right above the floor line
            width: 50.0,
            height: 50.0,
            velocity_y: 0.0,
            rotation: 0.0,
            is_grounded: false,
        }
    }

    pub fn update(&mut self, floor_y: f32) {
        // 1. Apply Gravity constant physics force
        let gravity = 0.6;
        self.velocity_y += gravity;
        self.y += self.velocity_y;

        // 2. Floor Collision Resolution Check
        let player_bottom = self.y + self.height;
        if player_bottom >= floor_y {
            self.y = floor_y - self.height;
            self.velocity_y = 0.0;
            self.is_grounded = true;

            // Snap rotation cleanly to the nearest 90 degrees when touching floor
            self.rotation = (self.rotation / 90.0).round() * 90.0;
        } else {
            self.is_grounded = false;
            // Spin smoothly while tumbling through mid-air jump frames
            self.rotation += 8.0;
        }

        // 3. Handle Jump Input Trigger Activation Check
        if self.is_grounded
            && (is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left))
        {
            let jump_force = -12.5;
            self.velocity_y = jump_force;
            self.is_grounded = false;
        }
    }

    pub fn draw(&self, texture: &Texture2D) {
        // Pivot center rotation math matrix translation
        draw_texture_ex(
            texture,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                rotation: self.rotation.to_radians(),
                ..Default::default()
            },
        );
    }

    // Custom alignment bounding box wrapper check routine for collision validation
    pub fn get_rect(&self) -> Rect {
        Rect::new(
            self.x + 5.0,
            self.y + 5.0,
            self.width - 10.0,
            self.height - 10.0,
        )
    }
}
