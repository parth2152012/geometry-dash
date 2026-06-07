use crate::obstacles::{Spike, SpikeType};
use rand::RngExt;

pub fn spawn_next_chunk(spikes: &mut Vec<Spike>, spawn_x: &mut f32) {
    let mut rng = rand::rng();

    // Create random layout gaps between hazards
    for _ in 0..3 {
        let gap = rng.random_range(200.0..400.0);
        *spawn_x += gap;

        let hazard_type = if rng.random_bool(0.5) {
            SpikeType::Small
        } else {
            SpikeType::Tall
        };

        // Match the base coordinate floor height layer
        spikes.push(Spike::new(*spawn_x, 400.0, hazard_type));
    }
}
