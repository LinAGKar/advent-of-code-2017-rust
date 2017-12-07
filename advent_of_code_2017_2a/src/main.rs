use std::io;
use std::io::Read;

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Failed to read line");
    println!("{}", string.lines().map(|x| {
        let numbers: Vec<i32> = x.split_whitespace().map(|y| y.parse().unwrap()).collect();
        numbers.iter().max().unwrap() - numbers.iter().min().unwrap()
    }).sum::<i32>());
}
