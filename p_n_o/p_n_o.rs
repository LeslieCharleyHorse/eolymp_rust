use std::io;

fn main() {
    // example below, replace it with your solution
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: i32 = input.trim().parse().unwrap();

    if n > 0 {println!("Positive");}
    else if n < 0 {println!("Negative");}
    else {println!("Zero");}
}
