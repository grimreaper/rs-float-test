use std::rand;
use std::rand::Rng;
use rand_distr::{Distribution, Normal, NormalError};
use rand_distr::uniform;

fn main() {
    let mut rng = rand::thread_rng();
    let range = Uniform::new(0, 10.0);
    let vals: Vec<float64> = (0..100).map(|_| rng.sample(&range)).collect();

    println!(vals);
}
