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

use std::collections::HashMap;

use self::property::Property;
pub use self::property::Size;
pub use self::tile::Tile;

#[macro_use]
pub mod tile;

mod property;

/// The World class. 
///
/// `NM` is the `NoiseMap` class, `T` is the type for each tile. See the
/// module documentation for for information.
pub struct World<T> {
    tiles: Vec<Tile<T>>,

    size: Size
}

impl<T> Default for World<T> {
    fn default() -> World<T> {
        World {
            tiles: Vec::new(),

            size: Default::default()
        }
    }
}

impl<T: Clone> World<T> {
    /// Constructs a new world using a given noisemap
    pub fn new() -> World<T> {
        World {
            tiles: Vec::new(),

            ..Default::default()
        }
    }

    /// Add a tile definition to the world
    pub fn add(self, tile: Tile<T>) -> World<T> {
        let mut new = self;
        new.tiles.push(tile);
        new
    }
    
    /// Set a property on the world
    pub fn set<P: Property>(self, property: P) -> World<T> {
        property.set_to(self)
    }

    pub fn set_size(self, size: Size) -> World<T> {
        let mut new = self;
        new.size = size;
        new
    }

    pub fn generate(&self, chunk_x: i64, chunk_y: i64) -> Option<Vec<Vec<T>>> {
        let mut nms = HashMap::new();

        (chunk_y * self.size.h .. (chunk_y + 1) * self.size.h).map(|y|
            (chunk_x * self.size.w .. (chunk_x + 1) * self.size.w).map(|x| {
                match self.tiles.iter().find(|tile| tile.satisfied_by(x, y, self.size, chunk_x, chunk_y, &mut nms)) {
                    Some(tile) => Some(tile.value()),
                    None => return None
                }
            }
            ).collect()
        ).collect()
    }
        /*
        self.nm.generate(x, y).iter()
            .map(|row| row.iter().map(|value| match self.tiles.iter().find(|tile| tile.satisfied_by(value)) {
                Some(tile) => tile.value(),
                None => panic!("No tile constraints for value")
            }).collect()
        ).collect()
        */
}
