extern crate gcc;

fn main() {
    gcc::compile_library("libnoiseval.a", &["src/c/noiseval.c"]);
}
