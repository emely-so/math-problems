// Generates a random integer between 1 and 10
fn main() {
    let mut rng = rand::thread_rng();
    let random_number: i32 = rng.gen_range(1..=10);
    println!("The randomly generated number is {}", random_number);
}
