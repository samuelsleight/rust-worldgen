//! World and Noise Generation in Rust.
//!
//! Note that any/all of this documentation may change as the library goes
//! through changes.
//!
//! # Introduction
//!
//! To start generating a world, we need a source of noise. The `noise`
//! module contains different noise generators, for example for perlin noise:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! let noise = PerlinNoise::new();
//! ```
//!
//! We can generate a single value from a generator using its `generate`
//! method, as follows:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noise::NoiseProvider;
//! # let noise = PerlinNoise::new();
//! // x, y, seed
//! let value = noise.generate(1.0, 1.0, 15);
//! ```
//!
//! This on its own is not very useful or convenient, however by plugging this
//! into a `NoiseMap` (from the `noisemap` module) we can generate a field of 
//! continuous noise:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::{NoiseMap, NoiseMapGenerator, Size, Step};
//! # let noise = PerlinNoise::new();
//! let nm = NoiseMap::new(noise)
//!     .set(Size::of(10, 10))
//!     .set(Step::of(0.05, 0.05));
//!
//! let vec = nm.generate();
//! ```
//!
//! These can be combined and scaled to your liking:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::NoiseMap;
//! # let noise = PerlinNoise::new();
//! # let nm1 = NoiseMap::new(noise);
//! # let nm2 = NoiseMap::new(noise);
//! let nm = nm1 + nm2 * 5;
//! ```
//!
//! Finally, we can wrap this into a `World`, and produce a vector of specific
//! tiles (represented by anything you want) based on given constraints:
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::NoiseMap;
//! # use worldgen::world::{World, Tile};
//! # use worldgen::world::tile::Constraint;
//! # let noise = PerlinNoise::new();
//! # let nm = NoiseMap::new(noise);
//! let world = World::new(nm)
//!     .add(Tile::new('~').when(Constraint::LT(0.0)))
//!     .add(Tile::new(','));
//!
//! let tiles = world.generate();
//! ```
//!
//! For more information on each of the three components, look at the 
//! documentation of the relevant module.
//!
//! # Full Example
//!
//! ```
//! use worldgen::noise::perlin::PerlinNoise;
//! use worldgen::noisemap::{NoiseMapGenerator, NoiseMap, Seed, Step, Size};
//! use worldgen::world::{World, Tile};
//! use worldgen::world::tile::Constraint;
//!
//! let noise = PerlinNoise::new();
//!
//! let nm1 = NoiseMap::new(noise)
//!     .set(Seed::of("Hello?"))
//!     .set(Step::of(0.005, 0.005))
//!     .set(Size::of(80, 50));
//!
//! let nm2 = NoiseMap::new(noise)
//!     .set(Seed::of("Hello!"))
//!     .set(Step::of(0.05, 0.05))
//!     .set(Size::of(80, 50));
//!
//! let nm = nm1 + nm2 * 3;
//!
//! let world = World::new(nm)
//!     
//!     // Water
//!     .add(Tile::new('~')
//!          .when(Constraint::LT(-0.1)))
//!
//!     // Grass
//!     .add(Tile::new(',')
//!          .when(Constraint::LT(0.45)))
//!
//!     // Mountains
//!     .add(Tile::new('^')
//!          .when(Constraint::GT(0.8)))
//!
//!     // Hills
//!     .add(Tile::new('n'));
//!
//! for row in world.generate().iter() {
//!     for val in row.iter() {
//!         print!("{}", val);
//!     }
//!
//!     println!("");
//! }
//! ```
//!

#![feature(hash)]

#[cfg(test)]
use noisemap::{NoiseMap, NoiseMapGenerator, Seed, Step, Size};

#[cfg(test)]
use noise::perlin::PerlinNoise;

#[cfg(test)]
use world::{World, Tile};

#[cfg(test)]
use world::tile::Constraint;

pub mod noise;
pub mod noisemap;
pub mod world;

#[test]
fn it_works() {
    let noise = PerlinNoise::new();

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of("Hello?"))
        .set(Step::of(0.005, 0.005))
        .set(Size::of(80, 50));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of("Hello!"))
        .set(Step::of(0.05, 0.05))
        .set(Size::of(80, 50));

    let nm = nm1 + nm2 * 3;

    let world = World::new(nm)
        
        // Water
        .add(Tile::new('~')
             .when(Constraint::LT(-0.1)))

        // Grass
        .add(Tile::new(',')
             .when(Constraint::LT(0.45)))

        // Mountains
        .add(Tile::new('^')
             .when(Constraint::GT(0.8)))

        // Hills
        .add(Tile::new('n'));

    for row in world.generate().iter() {
        for val in row.iter() {
            print!("{}", val);
        }

        println!("");
    }
}
