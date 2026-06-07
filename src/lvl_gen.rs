use crate::obstacles::{Spike, SpikeType};
use rand::Rng;

pub fn spawn_next_chunk(spikes: &mut Vec<Spike>, spawn_x: &mut f32) {
    let mut rng = rand::thread_rng();
    let floor_line_y = 400.0;

    // Create random layout gaps between hazards
    for _ in 0..3 {
        let gap = rng.gen_range(200.0..400.0);
        *spawn_x += gap;

        let hazard_type = if rng.gen_bool(0.5) {
            SpikeType::Small
        } else {
            SpikeType::Tall
        };

        // Calculate height based on spike type
        let spike_height = match hazard_type {
            SpikeType::Small => 30.0,
            SpikeType::Tall => 60.0,
        };

        // Position spike so it sits ON the floor line (top of spike at floor_line_y - height)
        let spike_y = floor_line_y - spike_height;
        spikes.push(Spike::new(*spawn_x, spike_y, hazard_type));
    }
}
