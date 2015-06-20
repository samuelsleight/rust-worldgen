#![feature(hash)]

#[cfg(test)]
use noisemap::{NoiseMap, NoiseMapGenerator, Seed, Step, Size};

#[cfg(test)]
use noise::perlin::PerlinNoise;

#[cfg(test)]
use world::{World, Tile};

#[cfg(test)]
use world::tile::Constraint;

pub mod noise;
pub mod noisemap;
pub mod world;

#[test]
fn it_works() {
    let noise = PerlinNoise::new();

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of("Hello?"))
        .set(Step::of(0.005, 0.005))
        .set(Size::of(80, 50));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of("Hello!"))
        .set(Step::of(0.05, 0.05))
        .set(Size::of(80, 50));

    let nm = nm1 + nm2 * 3;

    let world = World::new(nm)
        
        // Water
        .add(Tile::new('~')
             .when(Constraint::LT(-0.1)))

        // Grass
        .add(Tile::new(',')
             .when(Constraint::LT(0.45)))

        // Mountains
        .add(Tile::new('^')
             .when(Constraint::GT(0.8)))

        // Hills
        .add(Tile::new('n'));

    for row in world.generate().iter() {
        for val in row.iter() {
            print!("{}", val);
        }

        println!("");
    }
}
