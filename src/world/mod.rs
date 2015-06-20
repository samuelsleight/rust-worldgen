use std::clone::Clone;

use noisemap::NoiseMapGenerator;

pub use self::tile::Tile;

pub mod tile;

#[derive(Clone)]
pub struct World<NM, T> {
    nm: NM,
    tiles: Vec<Tile<T>>
}

impl<NM: NoiseMapGenerator, T: Clone> World<NM, T> {
    pub fn new(nm: NM) -> World<NM, T> {
        World {
            nm: nm,
            tiles: Vec::new()
        }
    }

    pub fn add(self, tile: Tile<T>) -> World<NM, T> {
        let mut new = self.clone();
        new.tiles.push(tile);
        new
    }

    pub fn generate(&self) -> Vec<Vec<T>> {
        self.generate_chunk(0, 0)
    }

    pub fn generate_chunk(&self, x: i32, y: i32) -> Vec<Vec<T>> {
        self.nm.generate_chunk(x, y).iter()
            .map(|row| row.iter().map(|value| match self.tiles.iter().find(|tile| tile.satisfied_by(value)) {
                Some(tile) => tile.value(),
                None => panic!("No tile constraints for value")
            }).collect()
        ).collect()
    }
}
