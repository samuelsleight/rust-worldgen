#![feature(hash)]

#[cfg(test)]
use noisemap::{NoiseMap, Seed, Step, Size};

#[cfg(test)]
use noise::perlin::PerlinNoise;

pub mod noise;
pub mod noisemap;

#[test]
fn it_works() {
    let noise = PerlinNoise::new();

    let nm = NoiseMap::new(noise)
        .set(Seed::of("Hello"))
        .set(Step::of(0.005, 0.002))
        .set(Size::of(5, 5));

    println!("{:?}", nm);

    for y in 0 .. 8 {
        for row in nm.generate_chunk(0, y).iter() {
            for val in row.iter() {
                print!("{} ", val);
            }

            println!("");
        }
    }
}
