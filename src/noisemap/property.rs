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

use std::cmp::{PartialOrd, Ord, Ordering};
use std::default::Default;
use std::hash::{Hash, Hasher, SipHasher};

/// A property is an option that can be set on a noise map.
pub trait Property : Default + Copy {
    fn set_to<T>(self, nm: NoiseMap<T>) -> NoiseMap<T>;
}

/// Sets the seed that is used for generating the noise.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Seed {
    pub value: u64
}

impl Seed {
    /// Sets the seed to an exact integer value.
    pub fn of_value(value: u64) -> Seed {
        Seed {
            value: value
        }
    }

    /// Sets the seed to the hash of whatever is provided.
    pub fn of<T: Hash>(value: T) -> Seed {
        let mut hasher = SipHasher::new();
        value.hash(&mut hasher);

        Seed {
            value: hasher.finish() 
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
#[derive(Default, Copy, Clone, Debug, PartialEq)]
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
/// nothing will be generated. In a combination it needs to be set for only
/// one of the member noisemaps, because the size will be set to whichever is
/// largest when the combination is created.
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq)]
pub struct Size {
    pub w: i64,
    pub h: i64
}

impl Size {
    pub fn of(w: i64, h: i64) -> Size {
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

impl PartialOrd for Size {
    fn partial_cmp(&self, other: &Size) -> Option<Ordering> {
        (self.w * self.h).partial_cmp(&(other.w * other.h))
    }
}

impl Ord for Size {
    fn cmp(&self, other: &Size) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}
