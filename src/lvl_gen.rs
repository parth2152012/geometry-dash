// Inside lvl_gen.rs
use crate::obstacles::Spike;
use rand::{Rng, RngExt};

pub fn generate_random_chunk(current_spawn_x: f32, ground_y: f32) -> (Vec<Spike>, f32) {
    let mut rng = rand::rng();
    let mut chunk_spikes = Vec::new();

    // Pick a random number of spikes for this chunk (3 to 5)
    let num_spikes = rng.random_range(3..=5);

    // Start tracking the spacing inside this specific chunk
    let mut local_x = current_spawn_x;

    for _ in 0..num_spikes {
        // Add a random gap between spikes so they don't overlap or look boring
        // 120 to 300 pixels gives the player enough room to jump or land safely
        local_x += rng.random_range(120.0..300.0);

        // Spawn the spike at the newly advanced coordinate track
        chunk_spikes.push(Spike::new(local_x, ground_y, 40.0, 40.0));
    }

    // Crucial step: Add a final "buffer gap" at the end of the chunk
    // so the next chunk doesn't spawn immediately on top of the last spike
    let advanced_spawn_x = local_x + 400.0;

    // Hand back the array of spikes AND the brand new updated cursor position
    (chunk_spikes, advanced_spawn_x)
}
