use noise::NoiseProvider;

use std::default::Default;

pub use self::property::{Seed, Step, Size};
use self::property::Property;

mod property;

#[derive(Default, Debug)]
pub struct NoiseMap<T> {
    seed: Seed,
    step: Step,
    size: Size,

    noise: T
}

impl<T: NoiseProvider> NoiseMap<T> {
    pub fn new(noise: T) -> NoiseMap<T> {
        NoiseMap {
            noise: noise,

            ..Default::default()
        }
    }

    pub fn set<P: Property>(self, property: P) -> NoiseMap<T> {
        property.set_to(self)
    }

    pub fn generate(&self) -> Vec<Vec<f64>> {
        self.generate_chunk(0, 0)
    }

    pub fn generate_chunk(&self, x: i32, y: i32) -> Vec<Vec<f64>> {
        (y * self.size.h .. (y + 1) * self.size.h).map(|y| y as f64 * self.step.y)
            .map(|y| (x * self.size.w .. (x + 1) * self.size.w).map(|x| x as f64 * self.step.x)
                .map(|x| self.noise.generate(x, y, self.seed.value)).collect()
            ).collect()
    }
}

impl<T> NoiseMap<T> {
    fn set_seed(self, seed: Seed) -> NoiseMap<T> {
        NoiseMap {
            seed: seed,
            ..self
        }
    }

    fn set_step(self, step: Step) -> NoiseMap<T> {
        NoiseMap {
            step: step,
            ..self
        }
    }

    fn set_size(self, size: Size) -> NoiseMap<T> {
        NoiseMap {
            size: size,
            ..self
        }
    }
}
