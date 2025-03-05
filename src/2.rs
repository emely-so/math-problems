fn main() {
    let mut rng = rand::thread_rng();
    let number1: u32 = rng.gen_range(1..=10);
    let number2: u32 = rng.gen_range(1..=10);
    let result = number1 + number2;
    println!("What is {} plus {}?", number1, number2);
    loop {
        let answer = read_line!();
        if answer == result {
            println!("Correct!");
            break;
        } else {
            println!("Incorrect. Try again.");
        }
    }
}
