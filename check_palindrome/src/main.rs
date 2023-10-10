use std::env::args;

fn main() {
    let args: Vec<String> = args().collect();
    //TODO check args, Usage
    if args.len() > 2 {
        println!("Usage: {} <word>", args[0].to_string());
        return;
    }
    //TODO Implement palindrome function
    let word_check: String = args[1].to_string();
    println!("Word: {}", word_check);
    let mut j: usize = word_check.len()-1;
    println!("Length: {}",j);
    for i in word_check.chars() {
        if j <= (word_check.len()-1)/2 {
            break;
        }
        if i != word_check.chars().nth(j).unwrap() {
            println!("This is not a palindrome.");
            return;
        }
        j-=1;
    }
    println!("This is a palindrome.");
}