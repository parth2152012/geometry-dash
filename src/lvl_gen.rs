use crate::obstacles::{Spike, SpikeType};

pub fn spawn_next_chunk(spikes: &mut Vec<Spike>, spawn_x: &mut f32, difficulty: f32) {
    let floor_line_y = 400.0;

    // Difficulty scaling:
    // - Gap size decreases as difficulty increases (spikes get closer)
    // - More spikes spawn in each chunk as difficulty increases
    let spike_count = 3 + (difficulty / 50.0) as usize;
    let gap_min = 200.0 - (difficulty * 1.5);
    let gap_max = 300.0 - (difficulty * 2.0);

    // Ensure gaps don't go negative
    let gap_min = gap_min.max(10.0);
    let gap_max = gap_max.max(gap_min + 50.0);

    // Create random layout gaps between hazards
    for _ in 0..spike_count {
        // Macroquad's gen_range takes commas (min, max) instead of a range expression (min..max)
        let gap = macroquad::rand::gen_range(gap_min, gap_max);
        *spawn_x += gap;

        // Replaced rng.gen_bool(0.5) with a 50% float comparison
        let hazard_type = if macroquad::rand::gen_range(0.0, 1.0) < 0.5 {
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
