#[derive(Clone, Debug)]
pub enum SpikeType {
    Small,
    Tall,
}

#[derive(Clone, Debug)]
pub struct Spike {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    pub variant: SpikeType, // Tells the renderer which PNG to draw
}

impl Spike {
    pub fn new(x: f32, y: f32, w: f32, h: f32, variant: SpikeType) -> Self {
        Self {
            x,
            y,
            w,
            h,
            variant,
        }
    }
}
