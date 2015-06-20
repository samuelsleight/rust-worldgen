use super::NoiseMap;

use std::default::Default;
use std::hash::{hash, Hash, SipHasher};

pub trait Property : Default + Copy {
    fn set_to(self, nm: NoiseMap) -> NoiseMap;
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
    fn set_to(self, nm: NoiseMap) -> NoiseMap {
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
    fn set_to(self, nm: NoiseMap) -> NoiseMap {
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
    fn set_to(self, nm: NoiseMap) -> NoiseMap {
        nm.set_size(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Octaves {
    pub value: u32
}

impl Octaves {
    pub fn of(value: u32) -> Octaves {
        Octaves {
            value: value
        }
    }
}

impl Default for Octaves {
    fn default() -> Octaves {
        Octaves {
            value: 8
        }
    }
}

impl Property for Octaves {
    fn set_to(self, nm: NoiseMap) -> NoiseMap {
        nm.set_octaves(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Frequency {
    pub value: f64
}

impl Frequency {
    pub fn of(value: f64) -> Frequency {
        Frequency {
            value: value
        }
    }
}

impl Default for Frequency {
    fn default() -> Frequency {
        Frequency {
            value: 1.0
        }
    }
}

impl Property for Frequency {
    fn set_to(self, nm: NoiseMap) -> NoiseMap {
        nm.set_frequency(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Persistence {
    pub value: f64
}

impl Persistence {
    pub fn of(value: f64) -> Persistence {
        Persistence {
            value: value
        }
    }
}

impl Default for Persistence {
    fn default() -> Persistence {
        Persistence {
            value: 0.5
        }
    }
}

impl Property for Persistence {
    fn set_to(self, nm: NoiseMap) -> NoiseMap {
        nm.set_persistence(self)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Lacunarity {
    pub value: f64
}

impl Lacunarity {
    pub fn of(value: f64) -> Lacunarity {
        Lacunarity {
            value: value
        }
    }
}

impl Default for Lacunarity {
    fn default() -> Lacunarity {
        Lacunarity {
            value: 2.0
        }
    }
}

impl Property for Lacunarity {
    fn set_to(self, nm: NoiseMap) -> NoiseMap {
        nm.set_lacunarity(self)
    }
}
