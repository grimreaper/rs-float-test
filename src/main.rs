#[warn(unused_imports)]

use rand::Rng;
// use rand::distributions::Uniform;

fn main() {
    let mut rng = rand::thread_rng();
    // let range = Uniform::new(0, 10.0);
    // range.
    // let values: Vec<f64> = (0..100).map(|_| rng.sample(&range)).collect();
    let values = [(); 100].map(|_| rng.gen_range(0.0..1.0));
    // let values: Vec<f64> = rand::thread_rng().sample_iter(&range).take(100).collect();

    println!("{:?}", values);
}
