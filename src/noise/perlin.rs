//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/noise/perlin.rs
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

//! A provider of perlin noise.
//!
//! This noise source acts as an alias for the common use case of
//! octaved coherent noise.

use super::{coherent::CoherentNoise, octaved::OctavedNoise};

/// The perlin noise source
///
/// # Example
///
/// ```
/// # use worldgen::noise::perlin::PerlinNoise;
/// # use worldgen::noise::octaved::Octaves;;
/// # use worldgen::noise::NoiseProvider;
/// let noise = PerlinNoise::new()
///     .set(Octaves::of(5));
///
/// let value = noise.generate(1.5, 2.5, 15);
/// ```
pub enum PerlinNoise {}

impl PerlinNoise {
    pub fn new() -> OctavedNoise<CoherentNoise> {
        OctavedNoise::new(CoherentNoise)
    }
}
