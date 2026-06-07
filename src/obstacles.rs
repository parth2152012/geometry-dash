use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub enum SpikeType {
    Small,
    Tall,
}

#[derive(Clone)]
pub struct Spike {
    pub x: f32,
    pub y: f32,
    pub spike_type: SpikeType,
    pub width: f32,
    pub height: f32,
}

impl Spike {
    pub fn new(x: f32, y: f32, spike_type: SpikeType) -> Self {
        let (width, height) = match spike_type {
            SpikeType::Small => (30.0, 30.0),
            SpikeType::Tall => (40.0, 60.0),
        };
        Self {
            x,
            y,
            spike_type,
            width,
            height,
        }
    }

    pub fn draw(&self, small_tex: &Texture2D, tall_tex: &Texture2D) {
        let tex = match self.spike_type {
            SpikeType::Small => small_tex,
            SpikeType::Tall => tall_tex,
        };
        draw_texture_ex(
            tex,
            self.x,
            self.y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(self.width, self.height)),
                ..Default::default()
            },
        );
    }
}
