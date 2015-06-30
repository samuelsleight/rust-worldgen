# World and Noise Generation in Rust.

A library for simple generation of noisemaps and maps of objects in Rust.

# Usage

To use this library, simply add the following line to your Cargo.toml 
dependencies section:

```
worldgen = "0.1.0"
```

Then import the crate in your code:

```rust
extern crate worldgen;
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
                                                                              
let vec = nm.generate();
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
                                                                              
let tiles = world.generate();
```
                                                                              
For more information on each of the three components, look at the 
documentation of each relevent module.
                                                                              
# Full Example
                                                                              
```rust
use worldgen::noise::perlin::PerlinNoise;
use worldgen::noisemap::{NoiseMapGenerator, NoiseMap, Seed, Step, Size};
use worldgen::world::{World, Tile};
use worldgen::world::tile::Constraint;
                                                                              
let noise = PerlinNoise::new();
                                                                              
let nm1 = NoiseMap::new(noise)
    .set(Seed::of("Hello?"))
    .set(Step::of(0.005, 0.005))
    .set(Size::of(80, 50));
                                                                              
let nm2 = NoiseMap::new(noise)
    .set(Seed::of("Hello!"))
    .set(Step::of(0.05, 0.05));
                                                                              
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
```

