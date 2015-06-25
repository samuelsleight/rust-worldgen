//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/noisemap/property.rs
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

use super::NoiseMap;

use std::default::Default;
use std::hash::{hash, Hash, SipHasher};

/// A property is an option that can be set on a noise map.
pub trait Property : Default + Copy {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T>;
}

/// Sets the seed that is used for generating the noise.
#[derive(Default, Copy, Clone, Debug)]
pub struct Seed {
    pub value: i32
}

impl Seed {
    /// Sets the seed to an exact integer value.
    pub fn of_value(value: i32) -> Seed {
        Seed {
            value: value
        }
    }

    /// Sets the seed to the hash of whatever is provided.
    pub fn of<T: Hash>(value: T) -> Seed {
        Seed {
            value: hash::<_, SipHasher>(&value) as i32
        }
    }
}

impl Property for Seed {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T> {
        nm.set_seed(self)
    }
}

/// Sets the increment in x and y for each coordinate in the 
/// noise map.
///
/// The default values of this are 0, so if you do not set this then
/// every value will be the same.
#[derive(Default, Copy, Clone, Debug)]
pub struct Step {
    pub x: f64,
    pub y: f64
}

impl Step {
    pub fn of(x: f64, y: f64) -> Step {
        Step {
            x: x,
            y: y
        }
    }
}

impl Property for Step {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T> {
        nm.set_step(self)
    }
}

/// Sets the size of the generated chunks.
///
/// The default values of this are 0, so if you do not set this then
/// nothing will be generated.
#[derive(Default, Copy, Clone, Debug)]
pub struct Size {
    pub w: i32,
    pub h: i32
}

impl Size {
    pub fn of(w: i32, h: i32) -> Size {
        Size {
            w: w,
            h: h
        }
    }
}

impl Property for Size {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T> {
        nm.set_size(self)
    }
}
