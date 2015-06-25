//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/world/mod.rs
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

//! For generating maps of specfic objects based on thresholds.
//!
//! Once we have a `NoiseMap` we can use a `World` and generate a map
//! of specific tiles using threshold constraints.
//!
//! When you generate a world, it will generate the noisemap inside it,
//! and then check each value against every tile definition, in order
//! of their addition to the world.
//!
//! The following example will produce the character `~` when the noise value
//! is less than -0.7, the character `^` when the value is greater than 0.6, and
//! `,` otherwise.
//!
//! ```
//! # use worldgen::noise::perlin::PerlinNoise;
//! # use worldgen::noisemap::NoiseMap;
//! # use worldgen::world::{World, Tile};
//! # use worldgen::world::tile::Constraint;
//! # let noise = PerlinNoise::new();
//! # let nm = NoiseMap::new(noise);
//! let world = World::new(nm)
//!     .add(Tile::new('~').when(Constraint::LT(-0.7)))
//!     .add(Tile::new('^').when(Constraint::GT(0.6)))
//!     .add(Tile::new(','));
//! ```

use std::clone::Clone;

use noisemap::NoiseMapGenerator;

pub use self::tile::Tile;

pub mod tile;

/// The World class. 
///
/// `NM` is the `NoiseMap` class, `T` is the type for each tile. See the
/// module documentation for for information.
#[derive(Clone)]
pub struct World<NM, T> {
    nm: NM,
    tiles: Vec<Tile<T>>
}

impl<NM: NoiseMapGenerator, T: Clone> World<NM, T> {
    /// Constructs a new world using a given noisemap
    pub fn new(nm: NM) -> World<NM, T> {
        World {
            nm: nm,
            tiles: Vec::new()
        }
    }

    /// Add a tile definition to the world
    pub fn add(self, tile: Tile<T>) -> World<NM, T> {
        let mut new = self.clone();
        new.tiles.push(tile);
        new
    }

    /// Generates the central chunk of the world.
    ///
    /// The returned vector is a vector of rows of tiles
    ///
    /// This simply calls ```self.generate_chunk(0, 0)```.
    ///
    /// # Panics
    ///
    /// This function will panic if a value is found which satisfies no tiles
    /// constraints.
    ///
    /// # Example
    ///
    /// ```
    /// # use worldgen::noise::perlin::PerlinNoise;
    /// # use worldgen::noisemap::NoiseMap;
    /// # use worldgen::world::{World, Tile};
    /// # use worldgen::world::tile::Constraint;
    /// # let noise = PerlinNoise::new();
    /// # let nm = NoiseMap::new(noise);
    /// # let world: World<_, char> = World::new(nm);
    /// for row in world.generate().iter() {
    ///     for tile in row.iter() {
    ///         print!("{}", tile);
    ///     }
    ///     println!("");
    /// }
    /// ```
    pub fn generate(&self) -> Vec<Vec<T>> {
        self.generate_chunk(0, 0)
    }

    /// Generates a specific chunk of the world.
    ///
    /// # Panics
    ///
    /// This function will panic if a value is found which satisfies no tiles
    /// constraints.
    ///
    /// # Example
    ///
    /// ```
    /// # use worldgen::noise::perlin::PerlinNoise;
    /// # use worldgen::noisemap::NoiseMap;
    /// # use worldgen::world::{World, Tile};
    /// # use worldgen::world::tile::Constraint;
    /// # let noise = PerlinNoise::new();
    /// # let nm = NoiseMap::new(noise);
    /// # let world: World<_, char> = World::new(nm);
    /// for y in (0 .. 5) {
    ///     for row in world.generate_chunk(0, y).iter() {
    ///         for tile in row.iter() {
    ///             print!("{}", tile);
    ///         }
    ///         println!("");
    ///     }
    /// }
    /// ```
    pub fn generate_chunk(&self, x: i32, y: i32) -> Vec<Vec<T>> {
        self.nm.generate_chunk(x, y).iter()
            .map(|row| row.iter().map(|value| match self.tiles.iter().find(|tile| tile.satisfied_by(value)) {
                Some(tile) => tile.value(),
                None => panic!("No tile constraints for value")
            }).collect()
        ).collect()
    }
}
