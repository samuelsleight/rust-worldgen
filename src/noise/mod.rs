pub mod perlin;
pub mod coherent;

pub trait NoiseProvider : Default {
    fn generate(&self, x: f64, y: f64, seed: i32) -> f64;
}
