use std::env::args;
use rand::Rng; // 0.8.5
fn main() {
    let mut args: Vec<String> = args().skip(1).collect(); // Skip the executable name
    if args.len() < 2 {
        println!("Usage: shuffle_app <item> <item> ...\nn > 2");
        return;
    }
    for i in 0..args.len() {
        let random_index: usize = rand::thread_rng().gen_range(0..args.len()); //Getting the size of arguments.
        args.swap(i, random_index);
    }

    println!("{:?}", args); // Print the shuffled vector
}
