extern crate regex;

use regex::Regex;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\d+): (\d+)$").unwrap();
    println!("{}", re
        .captures_iter(&input)
        .map(|caps| (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap()))
        .filter(|&(n, x)| n % (x * 2 - 2) == 0)
        .fold(0, |acc, (n, x)| acc + n * x));
}
