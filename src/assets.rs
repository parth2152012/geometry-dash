use macroquad::prelude::*;

pub struct GameAssets {
    pub bg_texture: Texture2D,
    pub logo_texture: Texture2D,
    pub play_btn_img: Image,
    pub spike_small_tex: Texture2D,
    pub spike_tall_tex: Texture2D,
    pub player_tex: Texture2D, // <-- Add this field
}

impl GameAssets {
    pub async fn load() -> Self {
        let bg_texture = load_texture("./assets/bg.png").await.unwrap();
        bg_texture.set_filter(FilterMode::Nearest);

        let logo_texture = load_texture("./assets/logo.png").await.unwrap();
        logo_texture.set_filter(FilterMode::Nearest);

        let play_btn_img = load_image("./assets/play_btn.png").await.unwrap();

        let spike_small_tex = load_texture("./assets/spike_small.png").await.unwrap();
        spike_small_tex.set_filter(FilterMode::Nearest);

        let spike_tall_tex = load_texture("./assets/spike_tall.png").await.unwrap();
        spike_tall_tex.set_filter(FilterMode::Nearest);

        // Load your player icon block asset cleanly
        let player_tex = load_texture("./assets/player.png").await.unwrap();
        player_tex.set_filter(FilterMode::Nearest);

        Self {
            bg_texture,
            logo_texture,
            play_btn_img,
            spike_small_tex,
            spike_tall_tex,
            player_tex, // <-- Add this assignment
        }
    }
}
