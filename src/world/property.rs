//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/world/property.rs
//////////////////////////////////////////////////////////////////////////////
//  Copyright 2016 Samuel Sleight
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

use super::World;

pub use crate::noisemap::Size;

pub trait Property : Default + Copy {
    fn set_to<T: Clone>(self, w: World<T>) -> World<T>;
}

impl Property for Size {
    fn set_to<T: Clone>(self, w: World<T>) -> World<T> {
        w.set_size(self)
    }
}
