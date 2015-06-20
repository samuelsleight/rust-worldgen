#![feature(hash)]

#[cfg(test)]
use noisemap::{NoiseMap, NoiseMapGenerator, Seed, Step, Size};

#[cfg(test)]
use noise::perlin::PerlinNoise;

pub mod noise;
pub mod noisemap;

#[test]
fn it_works() {
    let noise = PerlinNoise::new();

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of("Hello!"))
        .set(Step::of(0.005, 0.005))
        .set(Size::of(5, 15));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of("Hello!"))
        .set(Step::of(0.05, 0.05))
        .set(Size::of(5, 15));

    let nm = nm1 + nm2 * 10;

    println!("{:?}", nm);

    for row in nm.generate().iter() {
        for val in row.iter() {
            print!("{} ", val);
        }

        println!("");
    }
}
