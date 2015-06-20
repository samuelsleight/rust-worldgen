extern crate libc;

use std::default::Default;

pub use noisemap::property::{Seed, Step, Size, Octaves, Frequency, Persistence, Lacunarity};
use noisemap::property::Property;

mod property;

#[link(name = "noiseval")]
extern {
    fn generate_random_value(x: libc::c_int, y: libc::c_int, seed: libc::c_int) -> libc::c_double;
}

#[derive(Default, Debug)]
pub struct NoiseMap {
    seed: Seed,
    step: Step,
    size: Size,
    octaves: Octaves,
    freq: Frequency,
    pers: Persistence,
    lacu: Lacunarity
}

impl NoiseMap {
    pub fn new() -> NoiseMap {
        Default::default()
    }

    pub fn set<T: Property>(self, property: T) -> NoiseMap {
        property.set_to(self)
    }

    pub fn generate(&self) -> Vec<Vec<f64>> {
        self.generate_chunk(0, 0)
    }

    pub fn generate_chunk(&self, x: i32, y: i32) -> Vec<Vec<f64>> {
        (y * self.size.h .. (y + 1) * self.size.h).map(|y| y as f64 * self.step.y)
            .map(|y| (x * self.size.w .. (x + 1) * self.size.w).map(|x| x as f64 * self.step.x)
                .map(|x| self.generate_value(x, y)).collect()
            ).collect()
    }

    fn generate_value(&self, x: f64, y: f64) -> f64 {
        let mut x = x * self.freq.value;
        let mut y = y * self.freq.value;
        let mut pers = 1.0f64;

        (0 .. self.octaves.value).fold(0.0, |value, octave| {
            let seed = self.seed.value + octave as i32;
            let value = value + self.generate_coherent_value(x, y, seed) * pers;

            x *= self.lacu.value;
            y *= self.lacu.value;
            pers *= self.pers.value;

            value
        })
    }

    fn generate_coherent_value(&self, x: f64, y: f64, seed: i32) -> f64 {
        unsafe {
            let x0 = if x > 0.0 { x as i32 } else { (x - 1.0) as i32 };
            let x1 = x0 + 1;

            let y0 = if y > 0.0 { y as i32 } else { (y - 1.0) as i32 };
            let y1 = y0 + 1;

            let xd = self.s_curve(x - x0 as f64);
            let yd = self.s_curve(y - y0 as f64);

            let x0y0 = generate_random_value(x0, y0, seed) as f64;
            let x1y0 = generate_random_value(x1, y0, seed) as f64;
            let x0y1 = generate_random_value(x0, y1, seed) as f64;
            let x1y1 = generate_random_value(x1, y1, seed) as f64;

            let v1 = self.interpolate(x0y0, x1y0, xd);
            let v2 = self.interpolate(x0y1, x1y1, xd);

            self.interpolate(v1, v2, yd)
        }
    }

    fn s_curve(&self, a: f64) -> f64 {
        a * a * (3.0 - 2.0 * a)
    }

    fn interpolate(&self, v1: f64, v2: f64, a: f64) -> f64 {
        ((1.0 - a) * v1) + (a * v2)
    }

    fn set_seed(self, seed: Seed) -> NoiseMap {
        NoiseMap {
            seed: seed,
            ..self
        }
    }

    fn set_step(self, step: Step) -> NoiseMap {
        NoiseMap {
            step: step,
            ..self
        }
    }

    fn set_size(self, size: Size) -> NoiseMap {
        NoiseMap {
            size: size,
            ..self
        }
    }

    fn set_octaves(self, octaves: Octaves) -> NoiseMap {
        NoiseMap {
            octaves: octaves,
            ..self
        }
    }

    fn set_frequency(self, freq: Frequency) -> NoiseMap {
        NoiseMap {
            freq: freq,
            ..self
        }
    }

    fn set_persistence(self, pers: Persistence) -> NoiseMap {
        NoiseMap {
            pers: pers,
            ..self
        }
    }

    fn set_lacunarity(self, lacu: Lacunarity) -> NoiseMap {
        NoiseMap {
            lacu: lacu,
            ..self
        }
    }
}
