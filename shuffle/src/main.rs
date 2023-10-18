use std::env::args;
use rand::Rng; // 0.8.5
fn main() {
    let mut args: Vec<String> = args().skip(1).collect(); // Skip the executable name
    for i in 0..args.len() {
        let random_index: usize = rand::thread_rng().gen_range(0..args.len());
        args.swap(i, random_index);
    }

    println!("{:?}", args); // Print the shuffled vector
}
