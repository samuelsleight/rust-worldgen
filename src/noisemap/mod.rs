use noise::NoiseProvider;

use std::default::Default;
use std::ops::{Add, Mul};

pub use self::property::{Seed, Step, Size};
use self::property::Property;

mod property;

pub trait NoiseMapGenerator : Clone + Copy {
    fn generate(&self) -> Vec<Vec<f64>> {
        self.generate_chunk(0, 0)
    }

    fn generate_chunk(&self, x: i32, y: i32) -> Vec<Vec<f64>>;
}

#[derive(Default, Debug, Clone, Copy)]
pub struct NoiseMap<T> {
    seed: Seed,
    step: Step,
    size: Size,

    noise: T
}

#[derive(Debug, Clone, Copy)]
pub struct ScaledNoiseMap<T> {
    nm: T,
    scale: i32
}

#[derive(Debug, Clone, Copy)]
pub struct NoiseMapCombination<T1, T2> {
    nm1: T1,
    nm2: T2,

    outer: bool,
    total_scale: i32
}

impl<T: NoiseProvider> NoiseMap<T> {
    pub fn new(noise: T) -> NoiseMap<T> {
        NoiseMap {
            noise: noise,

            ..Default::default()
        }
    }
}

impl<T: NoiseProvider> NoiseMapGenerator for NoiseMap<T> {
    fn generate_chunk(&self, x: i32, y: i32) -> Vec<Vec<f64>> {
        (y * self.size.h .. (y + 1) * self.size.h).map(|y| y as f64 * self.step.y)
            .map(|y| (x * self.size.w .. (x + 1) * self.size.w).map(|x| x as f64 * self.step.x)
                .map(|x| self.noise.generate(x, y, self.seed.value)).collect()
            ).collect()
    }
}

impl<T> NoiseMap<T> {
    pub fn set<P: Property>(self, property: P) -> NoiseMap<T> {
        property.set_to(self)
    }

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

impl<T: NoiseMapGenerator> NoiseMapGenerator for ScaledNoiseMap<T> {
    fn generate_chunk(&self, x: i32, y: i32) -> Vec<Vec<f64>> {
        self.nm.generate_chunk(x, y).iter().map(|row| row.iter().map(|value| value * self.scale as f64).collect()).collect()
    }
}

impl<T> ScaledNoiseMap<T> {
    fn scale(&self) -> i32 {
        self.scale
    }
}

impl<T1: NoiseMapGenerator, T2: NoiseMapGenerator> NoiseMapGenerator for NoiseMapCombination<T1, T2> {
    fn generate_chunk(&self, x: i32, y: i32) -> Vec<Vec<f64>> {
        if self.outer {
            self.nm1.generate_chunk(x, y).iter()
                .zip(self.nm2.generate_chunk(x, y).iter())
                .map(|(lr, rr)| lr.iter().zip(rr.iter()).map(|(lv, rv)| (lv + rv) / self.total_scale as f64).collect()).collect()
        } else {
            self.nm1.generate_chunk(x, y).iter()
                .zip(self.nm2.generate_chunk(x, y).iter())
                .map(|(lr, rr)| lr.iter().zip(rr.iter()).map(|(lv, rv)| lv + rv).collect()).collect()
        }
    }
}

impl<T1, T2> NoiseMapCombination<T1, T2> {
    fn inner(self) -> NoiseMapCombination<T1, T2> {
        NoiseMapCombination {
            outer: false,
            ..self
        }
    }
}


impl<T> Mul<i32> for NoiseMap<T> {
    type Output = ScaledNoiseMap<NoiseMap<T>>;

    fn mul(self, scale: i32) -> ScaledNoiseMap<NoiseMap<T>> {
        ScaledNoiseMap {
            nm: self,
            scale: scale
        }
    }
}

impl<T> Mul<i32> for ScaledNoiseMap<T> {
    type Output = ScaledNoiseMap<T>;

    fn mul(self, scale: i32) -> ScaledNoiseMap<T> {
        ScaledNoiseMap {
            nm: self.nm,
            scale: self.scale * scale
        }
    }
}

impl<T1: NoiseProvider, T2: NoiseProvider> Add<NoiseMap<T2>> for NoiseMap<T1> {
    type Output = NoiseMapCombination<NoiseMap<T1>, NoiseMap<T2>>;

    fn add(self, rhs: NoiseMap<T2>) -> Self::Output {
        NoiseMapCombination {
            nm1: self,
            nm2: rhs,

            outer: true,
            total_scale: 2
        }
    }
}

impl<T1: NoiseProvider, T2: NoiseMapGenerator> Add<ScaledNoiseMap<T2>> for NoiseMap<T1> {
    type Output = NoiseMapCombination<NoiseMap<T1>, ScaledNoiseMap<T2>>;

    fn add(self, rhs: ScaledNoiseMap<T2>) -> Self::Output {
        rhs + self
    }
}

impl<T: NoiseProvider, T1: NoiseMapGenerator, T2: NoiseMapGenerator> Add<NoiseMapCombination<T1, T2>> for NoiseMap<T> {
    type Output = NoiseMapCombination<NoiseMap<T>, NoiseMapCombination<T1, T2>>;

    fn add(self, rhs: NoiseMapCombination<T1, T2>) -> Self::Output {
        rhs + self
    }
}

impl<T1: NoiseMapGenerator, T2: NoiseProvider> Add<NoiseMap<T2>> for ScaledNoiseMap<T1> {
    type Output = NoiseMapCombination<NoiseMap<T2>, ScaledNoiseMap<T1>>;

    fn add(self, rhs: NoiseMap<T2>) -> Self::Output {
        NoiseMapCombination {
            nm1: rhs,
            nm2: self,

            outer: true,
            total_scale: 1 + self.scale
        }
    }
}

impl<T1: NoiseMapGenerator, T2: NoiseMapGenerator> Add<ScaledNoiseMap<T2>> for ScaledNoiseMap<T1> {
    type Output = NoiseMapCombination<ScaledNoiseMap<T1>, ScaledNoiseMap<T2>>;

    fn add(self, rhs: ScaledNoiseMap<T2>) -> Self::Output {
        NoiseMapCombination {
            nm1: self,
            nm2: rhs,

            outer: true,
            total_scale: self.scale + rhs.scale
        }
    }
}

impl<T: NoiseMapGenerator, T1: NoiseMapGenerator, T2: NoiseMapGenerator> Add<NoiseMapCombination<T1, T2>> for ScaledNoiseMap<T> {
    type Output = NoiseMapCombination<ScaledNoiseMap<T>, NoiseMapCombination<T1, T2>>;

    fn add(self, rhs: NoiseMapCombination<T1, T2>) -> Self::Output {
        rhs + self
    }
}

impl<T: NoiseProvider, T1: NoiseMapGenerator, T2: NoiseMapGenerator> Add<NoiseMap<T>> for NoiseMapCombination<T1, T2> {
    type Output = NoiseMapCombination<NoiseMap<T>, NoiseMapCombination<T1, T2>>;

    fn add(self, rhs: NoiseMap<T>) -> Self::Output {
        NoiseMapCombination {
            nm1: rhs,
            nm2: self.inner(),

            outer: true,
            total_scale: 1 + self.total_scale
        }
    }
}

impl<T: NoiseMapGenerator, T1: NoiseMapGenerator, T2: NoiseMapGenerator> Add<ScaledNoiseMap<T>> for NoiseMapCombination<T1, T2> {
    type Output = NoiseMapCombination<ScaledNoiseMap<T>, NoiseMapCombination<T1, T2>>;

    fn add(self, rhs: ScaledNoiseMap<T>) -> Self::Output {
        NoiseMapCombination {
            nm1: rhs,
            nm2: self.inner(),

            outer: true,
            total_scale: rhs.scale() + self.total_scale
        }
    }
}

impl<L1: NoiseMapGenerator, L2: NoiseMapGenerator, R1: NoiseMapGenerator, R2: NoiseMapGenerator> Add<NoiseMapCombination<R1, R2>> for NoiseMapCombination<L1, L2> {
    type Output = NoiseMapCombination<NoiseMapCombination<L1, L2>, NoiseMapCombination<R1, R2>>;

    fn add(self, rhs: NoiseMapCombination<R1, R2>) -> Self::Output {
        NoiseMapCombination {
            nm1: self.inner(),
            nm2: rhs.inner(),

            outer: true,
            total_scale: rhs.total_scale + self.total_scale
        }
    }
}
