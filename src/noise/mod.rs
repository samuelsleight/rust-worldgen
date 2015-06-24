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
    fn generate(&self, x: f64, y: f64, seed: i32) -> f64;
}
