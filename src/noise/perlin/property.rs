use super::PerlinNoise;

use std::default::Default;

pub trait Property : Default + Copy {
    fn set_to(self, perlin: PerlinNoise) -> PerlinNoise;
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
    fn set_to(self, perlin: PerlinNoise) -> PerlinNoise {
        perlin.set_octaves(self)
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
    fn set_to(self, perlin: PerlinNoise) -> PerlinNoise {
        perlin.set_frequency(self)
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
    fn set_to(self, perlin: PerlinNoise) -> PerlinNoise {
        perlin.set_persistence(self)
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
    fn set_to(self, perlin: PerlinNoise) -> PerlinNoise {
        perlin.set_lacunarity(self)
    }
}
