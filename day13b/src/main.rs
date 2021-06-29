extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else if a == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\d+): (\d+)$").unwrap();
    let layers: HashMap<i64, i64> = re
        .captures_iter(&input)
        .map(|caps| (caps[1].parse::<i64>().unwrap(), caps[2].parse::<i64>().unwrap() * 2 - 2))
        .collect();
    if let Some(res) = (0..layers
        .values()
        .fold(1, |acc, &x| lcm(acc, x))
    ).skip_while(|x| layers.iter().any(|(n, y)| (x + n) % y == 0)).next() {
        println!("{}", res);
    }
}
