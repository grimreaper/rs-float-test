use itertools::{Itertools};
#[warn(unused_imports)]

use rand::Rng;
// use rand::distributions::Uniform;

fn main() {
    let mut rng = rand::thread_rng();
    let values: [f64; 500] = std::array::from_fn(|_| rng.gen_range(0.0..1.0));
    let mut sorted_lower = values.clone();
    sorted_lower.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let sorted_upper = sorted_lower.clone().into_iter().rev().collect_vec();

    // println!("{:?}", values);
    // println!("{:?}", sorted_lower);
    // println!("{:?}", sorted_upper);

    // now sum them each way
    let summed_lower = sorted_lower.into_iter().reduce(|a, b| a + b).unwrap();
    let summed_upper = sorted_upper.into_iter().reduce(|a, b| a + b).unwrap();

    println!("{}", summed_lower);
    println!("{}", summed_upper);

    // delta
    println!("{}", summed_lower - summed_upper);

}
