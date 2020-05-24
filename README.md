# World and Noise Generation in Rust.

A library for simple generation of noisemaps and maps of objects in Rust.

The full documentation can be found [here](https://docs.rs/worldgen/).

# Usage

To use this library, simply add the following line to your Cargo.toml
dependencies section:

```
worldgen = "0.5.2"
```

# Introduction

To start generating a world, we need a source of noise. The `noise`
module contains different noise generators, for example for perlin noise:

```rust
let noise = PerlinNoise::new();
```

We can generate a single value from a generator using its `generate`
method, as follows:

```rust
// x, y, seed
let value = noise.generate(1.0, 1.0, 15);
```

This on its own is not very useful or convenient, however by plugging this
into a `NoiseMap` (from the `noisemap` module) we can generate a field of
continuous noise:

```rust
let nm = NoiseMap::new(noise)
    .set(Size::of(10, 10))
    .set(Step::of(0.05, 0.05));

let vec = nm.generate_chunk(0, 0);
```

These can be combined and scaled to your liking:

```rust
let nm = nm1 + nm2 * 5;
```

Finally, we can wrap this into a `World`, and produce a vector of specific
tiles (represented by anything you want) based on given constraints:

```rust
let world = World::new(nm)
    .add(Tile::new('~').when(Constraint::LT(0.0)))
    .add(Tile::new(','));

let tiles = world.generate(0, 0);
```

For more information on each of the three components, look at the
documentation of each relevent module.

# Full Example

```rust
use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{NoiseMapGenerator, NoiseMapGeneratorBase, NoiseMap, Seed, Step, Size};
use worldgen::world::{World, Tile};
use worldgen::world::tile::{Constraint, ConstraintType};

fn main() {
    let noise = PerlinNoise::new();

    let nm1 = NoiseMap::new(noise)
        .set(Seed::of("Hello?"))
        .set(Step::of(0.005, 0.005));

    let nm2 = NoiseMap::new(noise)
        .set(Seed::of("Hello!"))
        .set(Step::of(0.05, 0.05));

    let nm = Box::new(nm1 + nm2 * 3);

    let world = World::new()
        .set(Size::of(80, 50))

        // Water
        .add(Tile::new('~')
            .when(constraint!(nm.clone(), < -0.1)))

        // Grass
        .add(Tile::new(',')
            .when(constraint!(nm.clone(), < 0.45)))

        // Mountains
        .add(Tile::new('^')
            .when(constraint!(nm.clone(), > 0.8)))

        // Hills
        .add(Tile::new('n'));

    for row in world.generate(0, 0).iter() {
        for val in row.iter() {
            for c in val.iter() {
                print!("{}", c);
            }

            println!("");
        }

        println!("");
    }
}
```

