use crate::harmonic::{Harmonic, Sound};

pub struct SineWave {}

impl Sound for SineWave {
    fn compute(&self, index: u32) -> Option<Harmonic> {
        if index == 1 { return Some(Harmonic::MAX); }
        None
    }
}

pub struct SawWave {}

impl Sound for SawWave {
    fn compute(&self, index: u32) -> Option<Harmonic> {
        Some(Harmonic {
            amplitude: 1.0 / index as f32,
            phase: 0.0
        })
    }
}

pub struct SquareWave {}

impl Sound for SquareWave {
    fn compute(&self, index: u32) -> Option<Harmonic> {
        if index.is_multiple_of(2) { return None; }
        Some(Harmonic {
            amplitude: 1.0 / index as f32,
            phase: 0.0
        })
    }
}

pub struct TriWave {}

impl Sound for TriWave {
    fn compute(&self, index: u32) -> Option<Harmonic> {
        if index > 8 || index % 2 == 0 { return Some(Harmonic::SILENT); }
        let k = (index - 1) / 2;
        let phase = if k % 2 == 0 { 0.5 } else { 0.0 };

        Some(Harmonic {
            amplitude: 1.0 / index.pow(2) as f32,
            phase: 0.0
        })
    }
}