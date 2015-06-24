use super::PerlinNoise;

use std::default::Default;

/// A property is an option that can be set on a perlin noise
/// source.
pub trait Property : Default + Copy {
    fn set_to(self, perlin: PerlinNoise) -> PerlinNoise;
}

/// Octaves are the number of layers of coherent noise used
/// in the generation of perlin noise.
///
/// The default value for this is 8.
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

/// Frequency controls the 'width' of the noise. If you imagine noise
/// as hills and valleys, then frequency controls the distance between
/// them.
///
/// The default value for this is 1.0
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

/// Persistence controls how much each octave contributes to the
/// final noise value.
///
/// The default value for this is 0.5
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

/// Lacunarity controls the frequency of each octave in the final noise
/// value.
///
/// The default value for this is 2.0
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
