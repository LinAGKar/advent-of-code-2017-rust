use std::io;
use std::io::Read;

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Failed to read line");
    println!("{}", string.lines().map(|x| {
        let numbers: Vec<i32> = x.split_whitespace().map(|y| y.parse().unwrap()).collect();
        for i in &numbers {
            for j in &numbers {
                if i as *const _ != j as *const _ && j % i == 0 { return j / i; }
            }
        }
        panic!("Found no evenly divisible numbers");
    }).sum::<i32>());
}
