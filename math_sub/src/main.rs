use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    //TODO Usage
    if args.len() <= 1 {
        println!("Usage: {} <num1> <num2> ... <numN>", args[0]);
        return;
    }
    let mut total: f32 = args[1].parse::<f32>().unwrap();
    for i in args.iter().skip(2) {
        total = total - i.parse::<f32>().unwrap();
    }
    println!("Result: {}", total);
}
