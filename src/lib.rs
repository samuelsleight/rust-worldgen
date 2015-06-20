#![feature(hash)]

#[cfg(test)]
use noisemap::{NoiseMap, Seed, Step, Size};

pub mod noisemap;

#[test]
fn it_works() {
    let nm = NoiseMap::new()
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
