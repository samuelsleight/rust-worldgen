//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/tile/constraint.rs
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

use crate::noisemap::NoiseMapGeneratorBase;
use crate::world::Size;

#[derive(Copy, Clone)]
pub enum ConstraintType {
    /// This constraint is satisfied when the noise value is
    /// lower than the given threshold.
    LT(f64),

    /// This constraint is satisfied when the noise value is
    /// greater than the given threshold.
    GT(f64),
}

/// A constraint that limits when a tile should be chosen for
/// the generated world.
pub struct Constraint {
    nm: Box<dyn NoiseMapGeneratorBase>,
    constraint: ConstraintType,
}

#[macro_export]
macro_rules! constraint {
    ($nm:expr, < $v:expr) => {
        Constraint::new($nm, ConstraintType::LT($v))
    };
    ($nm:expr, > $v:expr) => {
        Constraint::new($nm, ConstraintType::GT($v))
    };
}

impl Constraint {
    pub fn new(nm: Box<dyn NoiseMapGeneratorBase>, constraint: ConstraintType) -> Constraint {
        Constraint { nm, constraint }
    }

    /// Returns true if the given value would satisfy this constraint.
    pub fn satisfied_by(
        &self,
        x: i64,
        y: i64,
        size: Size,
        chunk_x: i64,
        chunk_y: i64,
        nms: &mut HashMap<u64, Vec<Vec<f64>>>,
    ) -> bool {
        let id = self.nm.id();

        let nm = nms
            .entry(id)
            .or_insert_with(|| self.nm.generate_sized_chunk(size, chunk_x, chunk_y));

        let x = (x as i64).rem_euclid(size.w);
        let y = (y as i64).rem_euclid(size.h);

        match self.constraint {
            ConstraintType::LT(threshold) => nm[y as usize][x as usize] < threshold,
            ConstraintType::GT(threshold) => nm[y as usize][x as usize] > threshold,
        }
    }
}
