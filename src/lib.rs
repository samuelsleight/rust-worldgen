//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/lib.rs
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
//! # use worldgen::noisemap::{NoiseMap, NoiseMapGenerator, NoiseMapGeneratorBase, Size, Step};
//! # let noise = PerlinNoise::new();
//! let nm = NoiseMap::new(noise)
//!     .set(Size::of(10, 10))
//!     .set(Step::of(0.05, 0.05));
//!
//! let vec = nm.generate_chunk(0, 0);
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
//! Finally, we can wrap these into a `World`, and produce a vector of specific
//! tiles (represented by anything you want) based on given constraints:
//!
//! ```
//! # #[macro_use] extern crate worldgen;
//!
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::NoiseMap;
//! # use worldgen::world::{World, Tile};
//! # use worldgen::world::tile::{Constraint, ConstraintType};
//!
//! # fn main() {
//! # let noise = PerlinNoise::new();
//! # let nm = Box::new(NoiseMap::new(noise));
//! let world = World::new()
//!     .add(Tile::new('~').when(constraint!(nm, < 0.0)))
//!     .add(Tile::new(','));
//!
//! let tiles = world.generate(0, 0);
//! # }
//! ```
//!
//! For more information on each of the three components, look at the
//! documentation of the relevant module.
//!
//! # Full Example
//!
//! ```
//! #[macro_use] extern crate worldgen;
//!
//! use worldgen::noise::perlin::PerlinNoise;
//! use worldgen::noisemap::{NoiseMapGenerator, NoiseMapGeneratorBase, NoiseMap, Seed, Step, Size};
//! use worldgen::world::{World, Tile};
//! use worldgen::world::tile::{Constraint, ConstraintType};
//!
//! fn main() {
//!     let noise = PerlinNoise::new();
//!
//!     let nm1 = NoiseMap::new(noise)
//!         .set(Seed::of("Hello?"))
//!         .set(Step::of(0.005, 0.005));
//!
//!     let nm2 = NoiseMap::new(noise)
//!         .set(Seed::of("Hello!"))
//!         .set(Step::of(0.05, 0.05));
//!
//!     let nm = Box::new(nm1 + nm2 * 3);
//!
//!     let world = World::new()
//!         .set(Size::of(80, 50))
//!
//!         // Water
//!         .add(Tile::new('~')
//!             .when(constraint!(nm.clone(), < -0.1)))
//!
//!         // Grass
//!         .add(Tile::new(',')
//!             .when(constraint!(nm.clone(), < 0.45)))
//!
//!         // Mountains
//!         .add(Tile::new('^')
//!             .when(constraint!(nm.clone(), > 0.8)))
//!
//!         // Hills
//!         .add(Tile::new('n'));
//!
//!     for row in world.generate(0, 0).iter() {
//!         for val in row.iter() {
//!             for c in val.iter() {
//!                 print!("{}", c);
//!             }
//!
//!             println!("");
//!         }
//!
//!         println!("");
//!     }
//! }
//! ```
//!

#[cfg(test)]
use noisemap::{NoiseMap, NoiseMapGenerator, Seed, Size, Step};

#[cfg(test)]
use noise::perlin::PerlinNoise;

#[cfg(test)]
use world::{Tile, World};

#[cfg(test)]
use world::tile::{Constraint, ConstraintType};

pub mod noise;
pub mod noisemap;

#[macro_use]
pub mod world;

#[test]
fn it_works() {
    let noise = PerlinNoise::new();

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of("Hello?"))
        .set(Step::of(0.005, 0.005));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of("Hello!"))
        .set(Step::of(0.05, 0.05));

    let nm = Box::new(nm1 + nm2 * 3);

    let world = World::new()
        .set(Size::of(80, 50))
        // Water
        .add(Tile::new('~').when(constraint!(nm.clone(), < -0.1)))
        // Grass
        .add(Tile::new(',').when(constraint!(nm.clone(), < 0.45)))
        // Mountains
        .add(Tile::new('^').when(constraint!(nm, > 0.8)))
        // Hills
        .add(Tile::new('n'));

    if let Some(chunk) = world.generate(0, 0) {
        for row in chunk {
            for tile in row {
                print!("{}", tile);
            }

            println!();
        }

        println!();
    }
}
