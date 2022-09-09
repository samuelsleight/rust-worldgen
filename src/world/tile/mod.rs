//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/tile/mod.rs
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

use std::collections::HashMap;

pub use self::constraint::{Constraint, ConstraintType};
use crate::world::Size;

#[macro_use]
mod constraint;

/// Objects to generate in the world based on given constraints
pub struct Tile<T> {
    value: T,
    constraints: Vec<Constraint>,
}

impl<T: Clone> Tile<T> {
    /// Construct a new tile represented by the given object.
    pub fn new(value: T) -> Tile<T> {
        Tile {
            value,
            constraints: Vec::new(),
        }
    }

    /// Adds a constraint to the tile.
    pub fn when(self, constraint: Constraint) -> Tile<T> {
        Tile {
            constraints: {
                let mut cs = self.constraints;
                cs.push(constraint);
                cs
            },
            ..self
        }
    }

    /// Returns the value this tile is represented by.
    pub fn value(&self) -> T {
        self.value.clone()
    }

    /// Returns true if the given value would satisfy all of this tile's
    /// constraints.
    pub fn satisfied_by(
        &self,
        x: i64,
        y: i64,
        size: Size,
        chunk_x: i64,
        chunk_y: i64,
        nms: &mut HashMap<u64, Vec<Vec<f64>>>,
    ) -> bool {
        self.constraints
            .iter()
            .all(|constraint| constraint.satisfied_by(x, y, size, chunk_x, chunk_y, nms))
    }
}
