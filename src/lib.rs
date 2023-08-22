use rand::prelude::*;

pub struct VmoHost {
}

impl VmoHost {
    pub fn fwd_accel(&self) -> f32 {
        thread_rng().gen_range(0.0..45.)
    }

    pub fn rot_accel(&self) -> f32 {
        thread_rng().gen_range(0.0..180.0)
    }
}

