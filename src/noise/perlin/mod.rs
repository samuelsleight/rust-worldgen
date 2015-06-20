use std::default::Default;

use super::NoiseProvider;
use super::coherent::CoherentNoise;

pub use self::property::{Octaves, Frequency, Persistence, Lacunarity};
use self::property::Property;

mod property;

#[derive(Default, Debug, Copy, Clone)]
pub struct PerlinNoise {
    octaves: Octaves,
    freq: Frequency,
    pers: Persistence,
    lacu: Lacunarity
}

impl PerlinNoise {
    pub fn new() -> PerlinNoise {
        Default::default()
    }

    pub fn set<T: Property>(self, property: T) -> PerlinNoise {
        property.set_to(self)
    }

    fn set_octaves(self, octaves: Octaves) -> PerlinNoise {
        PerlinNoise {
            octaves: octaves,
            ..self
        }
    }

    fn set_frequency(self, freq: Frequency) -> PerlinNoise {
        PerlinNoise {
            freq: freq,
            ..self
        }
    }

    fn set_persistence(self, pers: Persistence) -> PerlinNoise {
        PerlinNoise {
            pers: pers,
            ..self
        }
    }

    fn set_lacunarity(self, lacu: Lacunarity) -> PerlinNoise {
        PerlinNoise {
            lacu: lacu,
            ..self
        }
    }
}

impl NoiseProvider for PerlinNoise {
    fn generate(&self, x: f64, y: f64, seed: i32) -> f64 {
        let mut x = x * self.freq.value;
        let mut y = y * self.freq.value;
        let mut pers = 1.0f64;

        (0 .. self.octaves.value).fold(0.0, |value, octave| {
            let seed = seed + octave as i32;
            let value = value + CoherentNoise.generate(x, y, seed) * pers;

            x *= self.lacu.value;
            y *= self.lacu.value;
            pers *= self.pers.value;

            value
        })
    }
}
