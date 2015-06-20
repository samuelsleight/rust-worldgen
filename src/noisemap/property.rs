use super::NoiseMap;

use std::default::Default;
use std::hash::{hash, Hash, SipHasher};

pub trait Property : Default + Copy {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T>;
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Seed {
    pub value: i32
}

impl Seed {
    pub fn of_value(value: i32) -> Seed {
        Seed {
            value: value
        }
    }

    pub fn of<T: Hash>(value: T) -> Seed {
        Seed {
            value: hash::<_, SipHasher>(&value) as i32
        }
    }
}

impl Property for Seed {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T> {
        nm.set_seed(self)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Step {
    pub x: f64,
    pub y: f64
}

impl Step {
    pub fn of(x: f64, y: f64) -> Step {
        Step {
            x: x,
            y: y
        }
    }
}

impl Property for Step {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T> {
        nm.set_step(self)
    }
}

#[derive(Default, Copy, Clone, Debug)]
pub struct Size {
    pub w: i32,
    pub h: i32
}

impl Size {
    pub fn of(w: i32, h: i32) -> Size {
        Size {
            w: w,
            h: h
        }
    }
}

impl Property for Size {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T> {
        nm.set_size(self)
    }
}

