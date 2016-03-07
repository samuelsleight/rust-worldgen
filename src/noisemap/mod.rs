//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/noisemap/mod.rs
//////////////////////////////////////////////////////////////////////////////
//  Copyright 2015 Samuel Sleight
//
//  Licensed under the Apache License, Version 2.0 (the "License");
//  you may not use this file except in compliance with the License.
//  You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
//////////////////////////////////////////////////////////////////////////////

//! Generators for finite noise maps
//!
//! A `NoiseMap` takes a `NoiseProvider` and uses it to generate a map
//! of noise.
//!
//! They have properties that can be set to specify the seed used for
//! noise generation, the size of the generated map, and the scale of the
//! coordinates used for generation.
//!
//! A simple noise map is created by wrapping a source of noise, and then
//! setting the required properties:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::{NoiseMap, NoiseMapGenerator, Seed, Size, Step};
//! let noise = PerlinNoise::new();
//!
//! let nm = NoiseMap::new(noise)
//!     .set(Seed::of("Hello!"))
//!     .set(Size::of(10, 10))
//!     .set(Step::of(0.02, 0.02));
//! ```
//!
//! By default, a noise map will generate values between -1 and 1, however
//! one can be scaled by multiplying it by an integer:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::{NoiseMap, NoiseMapGenerator};
//! # let noise = PerlinNoise::new();
//! # let nm = NoiseMap::new(noise);
//! let nm = nm * 3;
//! ```
//!
//! This produces a `ScaledNoiseMap`, which multiplies all of its values
//! by the factor when they are generated.
//!
//! Noise maps can also be combined by adding them together:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::{NoiseMap, NoiseMapGenerator};
//! # let noise = PerlinNoise::new();
//! # let nm1 = NoiseMap::new(noise);
//! # let nm2 = NoiseMap::new(noise);
//! let nm = nm1 + nm2 * 3;
//! ```
//!
//! This final result will be normalised back between -1 and 1.
//!
//! Once you have the noise map you want, you can then use it to produce
//! a vector of rows of values:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::{NoiseMap, NoiseMapGenerator, NoiseMapGeneratorBase};
//! # let noise = PerlinNoise::new();
//! # let nm = NoiseMap::new(noise);
//! for row in nm.generate_chunk(0, 0).iter() {
//!     for value in row.iter() {
//!         print!("{}", value);
//!     }
//!     println!("");
//! }
//! ```
//!
//! A noise map is essentially an infinite plane of numbers, and the `generate`
//! method produces the central chunk of the size specified. You can use the 
//! `generate_chunk` method to generate specific chunks and produce infinite
//! maps.

use noise::NoiseProvider;

use std::default::Default;
use std::ops::{Add, Mul};
use std::cmp;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

pub use self::property::{Seed, Step, Size, Property};

mod property;

static NEXT_NM_ID: AtomicUsize = ATOMIC_USIZE_INIT;

pub fn next_id() -> u64 {
    NEXT_NM_ID.fetch_add(1, Ordering::SeqCst) as u64
}

/// Base trait for noise maps.
///
/// `NoiseMap`, `ScaledNoiseMap`, and `NoiseMapCombination` all implement
/// this trait.
pub trait NoiseMapGeneratorBase {
    /// Generates a specific chunk of the noise map.
    ///
    /// This can be used to generate a larger map in smaller parts.
    ///
    /// # Example
    ///
    /// ```
    /// # use worldgen::noise::perlin::PerlinNoise;
    /// # use worldgen::noisemap::{NoiseMap, NoiseMapGenerator, NoiseMapGeneratorBase};
    /// # let noise = PerlinNoise::new();
    /// # let nm = NoiseMap::new(noise);
    /// for y in 0 .. 5 {
    ///     for row in nm.generate_chunk(0, y).iter() {
    ///         for value in row.iter() {
    ///             print!("{}", value);
    ///         }
    ///         println!("");
    ///     }
    /// }
    /// ```
    fn generate_chunk(&self, x: i64, y: i64) -> Vec<Vec<f64>>;

    fn generate_sized_chunk(&self, size: Size, x: i64, y: i64) -> Vec<Vec<f64>>;

    fn id(&self) -> u64;
}

pub trait NoiseMapGenerator : NoiseMapGeneratorBase + Clone + Mul<i64, Output=ScaledNoiseMap<Self>> {
    /// Set a property on the noise map.
    fn set<P: Property>(self, property: P) -> Self where Self: Sized;

    fn set_size(self, size: Size) -> Self where Self: Sized;
    fn set_seed(self, seed: Seed) -> Self where Self: Sized;
    fn set_step(self, step: Step) -> Self where Self: Sized;
    
    /// Returns the size of the noise map.
    fn get_size(&self) -> Size where Self: Sized;
}

/// The standard noise map.
///
/// This is the base noise map, and is created by wrapping a 
/// noise source. It has properties that allow the setting of the
/// generation seed, the size of the generated chunks, and the coordinate
/// scale.
#[derive(Default, Debug, Clone, Copy)]
pub struct NoiseMap<T> {
    seed: Seed,
    step: Step,
    size: Size,

    noise: T,
    
    id: u64
}

/// A scaled noise map.
///
/// Created when a noise map is multiplied:
///
/// ```
/// # use worldgen::noise::perlin::PerlinNoise;
/// # use worldgen::noisemap::NoiseMap;
/// # let noise = PerlinNoise::new();
/// # let nm = NoiseMap::new(noise);
///
/// let snm = nm * 5;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct ScaledNoiseMap<T> {
    nm: T,
    scale: i64,

    id: u64
}

/// A combination of noise maps.
///
/// Created when two noise maps are added together. The resulting noise
/// map values will be normalised to between -1 and 1. This will take on the
/// size of the largest noise map in the combination, meaning you only have
/// to set the size once.
///
/// ```
/// # use worldgen::noise::perlin::PerlinNoise;
/// # use worldgen::noisemap::NoiseMap;
/// # let noise = PerlinNoise::new();
/// # let nm1 = NoiseMap::new(noise);
/// # let nm2 = NoiseMap::new(noise);
///
/// let nmc = nm1 + nm2 * 5;
/// ```
#[derive(Debug, Clone, Copy)]
pub struct NoiseMapCombination<T1, T2> {
    nm1: T1,
    nm2: T2,

    outer: bool,
    total_scale: i64,

    id: u64

}

impl<T: NoiseProvider> NoiseMap<T> {
    /// Construct a new noise map with the default properties.
    pub fn new(noise: T) -> NoiseMap<T> {
        NoiseMap {
            noise: noise,

            id: next_id(),

            ..Default::default()
        }
    }
}

impl<T: NoiseProvider> NoiseMapGeneratorBase for NoiseMap<T> {
    fn generate_chunk(&self, x: i64, y: i64) -> Vec<Vec<f64>> {
        let size = self.size;
        self.generate_sized_chunk(size, x, y)
    }

    fn generate_sized_chunk(&self, size: Size, x: i64, y: i64) -> Vec<Vec<f64>> {
        (y * size.h .. (y + 1) * size.h).map(|y| y as f64 * self.step.y)
            .map(|y| (x * size.w .. (x + 1) * size.w).map(|x| x as f64 * self.step.x)
                .map(|x| self.noise.generate(x, y, self.seed.value)).collect()
            ).collect()
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl<T: NoiseProvider> NoiseMapGenerator for NoiseMap<T> {
    // type ScaledInner = NoiseMap<T>;

    fn set<P: Property>(self, property: P) -> NoiseMap<T> {
        property.set_to(self)
    }

    fn get_size(&self) -> Size {
        self.size
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

impl<T: NoiseMapGenerator> NoiseMapGeneratorBase for ScaledNoiseMap<T> {
    fn generate_chunk(&self, x: i64, y: i64) -> Vec<Vec<f64>> {
        let size = self.nm.get_size();
        self.generate_sized_chunk(size, x, y)
    }

    fn generate_sized_chunk(&self, size: Size, x: i64, y: i64) -> Vec<Vec<f64>> {
        self.nm.generate_sized_chunk(size, x, y).iter().map(|row| row.iter().map(|value| value * self.scale as f64).collect()).collect()
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl<T: NoiseMapGenerator> NoiseMapGenerator for ScaledNoiseMap<T> {
    // type ScaledInner = T;

    fn set<P: Property>(self, property: P) -> ScaledNoiseMap<T> {
        ScaledNoiseMap {
            nm: self.nm.set(property),
            ..self
        }
    }

    fn get_size(&self) -> Size {
        self.nm.get_size()
    }

    fn set_seed(self, seed: Seed) -> ScaledNoiseMap<T> {
        self.set(seed)
    }

    fn set_step(self, step: Step) -> ScaledNoiseMap<T> {
        self.set(step)
    }

    fn set_size(self, size: Size) -> ScaledNoiseMap<T> {
        self.set(size)
    }
}

impl<T> ScaledNoiseMap<T> {
    pub fn new(nm: T, scale: i64) -> ScaledNoiseMap<T> {
        ScaledNoiseMap {
            nm: nm,
            scale: scale,

            id: next_id()
        }
    }

    pub fn scale(&self) -> i64 {
        self.scale
    }
}


impl<T1: NoiseMapGenerator, T2: NoiseMapGenerator> NoiseMapGeneratorBase for NoiseMapCombination<T1, T2> {
    fn generate_chunk(&self, x: i64, y: i64) -> Vec<Vec<f64>> {
        let size = self.get_size();
        self.generate_sized_chunk(size, x, y)
    }

    fn generate_sized_chunk(&self, size: Size, x: i64, y: i64) -> Vec<Vec<f64>> {
        if self.outer {
            let nm1_map = self.nm1.generate_sized_chunk(size, x, y);
            let nm2_map = self.nm2.generate_sized_chunk(size, x, y);
            nm1_map.iter().zip(nm2_map.iter()).map(|(lr, rr)| lr.iter().zip(rr.iter()).map(|(lv, rv)| (lv + rv) / self.total_scale as f64).collect()).collect()
        } else {
            let nm1_map = self.nm1.generate_sized_chunk(size, x, y);
            let nm2_map = self.nm2.generate_sized_chunk(size, x, y);
            nm1_map.iter().zip(nm2_map.iter()).map(|(lr, rr)| lr.iter().zip(rr.iter()).map(|(lv, rv)| lv + rv).collect()).collect()
        }
    }

    fn id(&self) -> u64 {
        self.id
    }
}

impl<T1: NoiseMapGenerator, T2: NoiseMapGenerator> NoiseMapGenerator for NoiseMapCombination<T1, T2> {
    // type ScaledInner = NoiseMapCombination<T1, T2>;

    fn set<P: Property>(self, property: P) -> NoiseMapCombination<T1, T2> {
        NoiseMapCombination {
            nm1: self.nm1.set(property),
            nm2: self.nm2.set(property),
            ..self
        }
    }

    fn get_size(&self) -> Size {
        self.nm1.get_size()
    }

    fn set_seed(self, seed: Seed) -> NoiseMapCombination<T1, T2> {
        self.set(seed)
    }

    fn set_step(self, step: Step) -> NoiseMapCombination<T1, T2> {
        self.set(step)
    }

    fn set_size(self, size: Size) -> NoiseMapCombination<T1, T2> {
        self.set(size)
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

impl<T: NoiseProvider> Mul<i64> for NoiseMap<T> {
    type Output = ScaledNoiseMap<NoiseMap<T>>;

    fn mul(self, scale: i64) -> ScaledNoiseMap<NoiseMap<T>> {
        ScaledNoiseMap::new(self, scale)
    }
}

impl<T: NoiseMapGenerator> Mul<i64> for ScaledNoiseMap<T> {
    type Output = ScaledNoiseMap<ScaledNoiseMap<T>>;

    fn mul(self, scale: i64) -> ScaledNoiseMap<ScaledNoiseMap<T>> {
        ScaledNoiseMap::new(self, scale)
    }
}

impl<T1: NoiseMapGenerator, T2: NoiseMapGenerator> Mul<i64> for NoiseMapCombination<T1, T2> {
    type Output = ScaledNoiseMap<NoiseMapCombination<T1, T2>>;

    fn mul(self, scale: i64) -> ScaledNoiseMap<NoiseMapCombination<T1, T2>> {
        ScaledNoiseMap::new(self, scale)
    }
}

impl<T1: NoiseProvider, T2: NoiseProvider> Add<NoiseMap<T2>> for NoiseMap<T1> {
    type Output = NoiseMapCombination<NoiseMap<T1>, NoiseMap<T2>>;

    fn add(self, rhs: NoiseMap<T2>) -> Self::Output {
        NoiseMapCombination {
            nm1: self,
            nm2: rhs,

            outer: true,
            total_scale: 2,

            id: next_id(),
        }.set(cmp::max(self.get_size(), rhs.get_size()))
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
        let scale = self.scale;

        let s1 = self.get_size();
        let s2 = rhs.get_size();

        NoiseMapCombination {
            nm1: rhs,
            nm2: self,

            outer: true,
            total_scale: scale + 1,

            id: next_id(),
        }.set(cmp::max(s1, s2))
    }
}

impl<T1: NoiseMapGenerator, T2: NoiseMapGenerator> Add<ScaledNoiseMap<T2>> for ScaledNoiseMap<T1> {
    type Output = NoiseMapCombination<ScaledNoiseMap<T1>, ScaledNoiseMap<T2>>;

    fn add(self, rhs: ScaledNoiseMap<T2>) -> Self::Output {
        let scale1 = self.scale;
        let scale2 = rhs.scale;

        let s1 = self.get_size();
        let s2 = rhs.get_size();

        NoiseMapCombination {
            nm1: self,
            nm2: rhs,

            outer: true,
            total_scale: scale1 + scale2,

            id: next_id(),
        }.set(cmp::max(s1, s2))
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
        let scale = self.total_scale;

        let s1 = self.get_size();
        let s2 = rhs.get_size();

        NoiseMapCombination {
            nm1: rhs,
            nm2: self.inner(),

            outer: true,
            total_scale: 1 + scale,

            id: next_id(),
        }.set(cmp::max(s1, s2))
    }
}

impl<T: NoiseMapGenerator, T1: NoiseMapGenerator, T2: NoiseMapGenerator> Add<ScaledNoiseMap<T>> for NoiseMapCombination<T1, T2> {
    type Output = NoiseMapCombination<ScaledNoiseMap<T>, NoiseMapCombination<T1, T2>>;

    fn add(self, rhs: ScaledNoiseMap<T>) -> Self::Output {
        let scale1 = rhs.scale();
        let scale2 = self.total_scale;

        let s1 = self.get_size();
        let s2 = rhs.get_size();

        let inner = self.inner();

        NoiseMapCombination {
            nm1: rhs,
            nm2: inner,

            outer: true,
            total_scale: scale1 + scale2,

            id: next_id(),
        }.set(cmp::max(s1, s2))
    }
}

impl<L1: NoiseMapGenerator, L2: NoiseMapGenerator, R1: NoiseMapGenerator, R2: NoiseMapGenerator> Add<NoiseMapCombination<R1, R2>> for NoiseMapCombination<L1, L2> {
    type Output = NoiseMapCombination<NoiseMapCombination<L1, L2>, NoiseMapCombination<R1, R2>>;

    fn add(self, rhs: NoiseMapCombination<R1, R2>) -> Self::Output {
        let scale1 = rhs.total_scale;
        let scale2 = self.total_scale;

        let s1 = self.get_size();
        let s2 = rhs.get_size();

        let inner1 = self.inner();
        let inner2 = rhs.inner();

        NoiseMapCombination {
            nm1: inner1,
            nm2: inner2,

            outer: true,
            total_scale: scale1 + scale2,

            id: next_id(),
        }.set(cmp::max(s1, s2))
    }
}
