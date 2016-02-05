//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/noise/mod.rs
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

//! The Noise module provides generators for different kinds of noise.
//! 
//! There are currently two different sources for noise: coherent and perlin.
//! The coherent noise source provides no customisation and is very simple, and
//! mainly exists to be used by the perlin source, which is the recommended one
//! to use at the moment.
//!
//! These generators provide a method for generating a noise value at a specific
//! location, however are best used in combination with a `NoiseMap`

pub mod perlin;
pub mod coherent;

/// The trait for a noise generator.
pub trait NoiseProvider : Default + Clone + Copy {
    /// This method generates a value of noise at the given location, using a given seed.
    fn generate(&self, x: f64, y: f64, seed: u64) -> f64;
}
