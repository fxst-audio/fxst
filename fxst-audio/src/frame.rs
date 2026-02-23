pub fn window_size(frequency: f32, sample_rate: u32) -> u32 {
    (sample_rate as f32 / frequency).round() as u32
}

pub fn window_end(start: u32, frequency: f32, sample_rate: u32) -> u32 {
    window_size(frequency, sample_rate) + start
}