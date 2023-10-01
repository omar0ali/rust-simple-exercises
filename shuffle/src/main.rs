use std::env::args;
use rand::Rng; // 0.8.5
fn main() {
    let mut args: Vec<String> = args().skip(1).collect(); // Skip the executable name
    let len = args.len();

    for i in 0..len {
        let random_index = rand::thread_rng().gen_range(0..len);
        args.swap(i, random_index);
    }

    println!("{:?}", args); // Print the shuffled vector
}
