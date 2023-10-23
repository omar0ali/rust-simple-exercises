use std::env::args;

fn square(num: f32) -> f32 {
    return num * num;
}


fn sqrt(find: f32) -> f32 {
    let mut square_number: f32 = 1.0;
    loop {
        if square(square_number) >= find {
            break;
        }
        square_number +=0.1;
    }
    return square_number-0.1;
}

fn main() {
    let args: Vec<String> = args().skip(1).collect();
    let size: usize = args.len();
    if size < 1 {
        println!("Usage: math_sqrt <Number>\n");
        return;
    }
    println!("Result: {} ", sqrt(args[0].parse().unwrap()));
}
