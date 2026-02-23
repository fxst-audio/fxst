pub mod waves;

use std::f32::consts::TAU;

pub fn pr(phase_percent: f32) -> f32 {
    TAU * phase_percent
}

pub struct Harmonic {
    pub amplitude: f32,
    pub phase: f32,
}

impl Harmonic {
    pub const SILENT: Self = Harmonic { amplitude: 0.0, phase: 0.0 };
    pub const MAX: Self =  Harmonic { amplitude: 1.0, phase: 0.0 };
    pub const MIN: Self =  Harmonic { amplitude: -1.0, phase: 0.0 };
}

pub fn radians(rotation: f32) -> f32 {
    TAU * rotation
}

pub fn harmonic_phase(start_phase: f32, offset: f32, nth: u32) -> f32 {
    let osc_phase = start_phase + offset;
    osc_phase * nth as f32
}

pub fn render_harmonic_sin(harmonic: Harmonic, offset: f32, nth: u32) -> f32 {
    let position = harmonic_phase(harmonic.phase, offset, nth);
    position.sin() * harmonic.amplitude
}

pub fn phase_delta(frequency: f32, sample_rate: u32) -> f32 {
    TAU * frequency / sample_rate as f32
}

pub fn next_phase(delta: f32, mut current: f32) -> f32 {
    current += delta;
    if current >= TAU {
        current -= TAU;
    }
    current
}

pub fn harmonic_limit(start: f32, end: f32) -> u32 {
    // end = start * n
    // end / start = n
    (end / start).floor() as u32
}

pub trait Sound {
    fn compute(&self, index: u32) -> Option<Harmonic>;
    fn reset(&self) {}
    fn get_harmonic(&self, index: u32) -> Harmonic {
        self.compute(index).unwrap_or(Harmonic::SILENT)
    }
}

pub trait Effect {
    fn process(&self, index: u32, harmonic: Harmonic) -> Option<Harmonic>;
    fn reset(&self) {}
    fn get_harmonic(&self, index: u32, harmonic: Harmonic) -> Harmonic {
        self.process(index, harmonic).unwrap_or(Harmonic::SILENT)
    }
}