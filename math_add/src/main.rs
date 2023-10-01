use std::env::args;


fn main(){
    let args: Vec<String> = args().collect();
    if args.len() <= 1 {
        println!("Usage (return sum of numN) : {} <num1> <num2> ... <numN>", args[0]);
        return;
    }
    let mut total: f32 = 0_f32;
    for x in args.iter().skip(1) {
        total = total + x.to_string().parse::<f32>().unwrap();
    }
    println!("Result {}\n", total);
}
