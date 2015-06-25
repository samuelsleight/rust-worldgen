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

/// A constraint that limits when a tile should be chosen for
/// the generated world.
#[derive(Clone, Copy)]
pub enum Constraint {
    /// This constraint is satisfied when the noise value is 
    /// lower than the given threshold.
    LT(f64),

    /// This constraint is satisfied when the noise value is 
    /// greater than the given threshold.
    GT(f64)
}

impl Constraint {
    /// Returns true is the given value would satisfy this constraint.
    pub fn satisfied_by(&self, value: &f64) -> bool {
        match self {
            &Constraint::LT(ref threshold) => value < threshold,
            &Constraint::GT(ref threshold) => value > threshold
        }
    }
}
