//////////////////////////////////////////////////////////////////////////////
//  File: rust-worldgen/noise/coherent.rs
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

//! A provider of coherent noise.
//! This generates a noise value for each corner of the unit square the given
//! point is in, and then calculates the noise value based on these.

use super::NoiseProvider;

fn generate_random_value(x: i32, y: i32, seed: i32) -> f64 {
    let m = wrapping! {
        let n = ((x * 157) + (y * 31337) + (seed * 2633)) & 0x7fffffff;
        (n << 13) ^ n
    };

    1.0 - (wrapping! { ((m * (m * m * 15731 + 789221) + 1376312579) & 0x7fffffff) as f64 } / 1073741824.0)
}

fn s_curve(a: f64) -> f64 {
    a * a * (3.0 - 2.0 * a)
}

fn interpolate(v1: f64, v2: f64, a: f64) -> f64 {
    ((1.0 - a) * v1) + (a * v2)
}

#[derive(Default, Copy, Clone)]
pub struct CoherentNoise;

impl NoiseProvider for CoherentNoise {
    fn generate(&self, x: f64, y: f64, seed: u64) -> f64 {
        let x0 = if x > 0.0 { x as i32 } else { (x - 1.0) as i32 };
        let x1 = x0 + 1;

        let y0 = if y > 0.0 { y as i32 } else { (y - 1.0) as i32 };
        let y1 = y0 + 1;

        let xd = s_curve(x - x0 as f64);
        let yd = s_curve(y - y0 as f64);

        let x0y0 = generate_random_value(x0, y0, seed as i32);
        let x1y0 = generate_random_value(x1, y0, seed as i32);
        let x0y1 = generate_random_value(x0, y1, seed as i32);
        let x1y1 = generate_random_value(x1, y1, seed as i32);

        let v1 = interpolate(x0y0, x1y0, xd);
        let v2 = interpolate(x0y1, x1y1, xd);

        interpolate(v1, v2, yd)
    }
}

