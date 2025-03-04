use rand::Rng;
use std::time::{Duration, Instant};

fn main() {
    let mut rng = rand::thread_rng();

    // Generate a random number between 1 and 10
    let num: i32 = rng.gen_range(1..=10);

    // Print the result
    println!("The generated number is: {}", num);
}
