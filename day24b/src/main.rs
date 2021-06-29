extern crate regex;

use regex::Regex;
use std::cmp::Ordering::Equal;
use std::io;
use std::io::Read;

fn longest(comps: &Vec<(u64, u64)>, used: &mut Vec<bool>, end: u64, length: u64) -> (u64, u64) {
    let to_check: Vec<(usize, &(u64, u64))> = comps.iter().enumerate().filter(|&(n, &(a, b))| !used[n] && (a == end || b == end)).collect();
    to_check.iter().map(|&(n, &(a, b))| {
        used[n] = true;
        let (length, strength) = longest(comps, used, if a == end { b } else { a }, length + 1);
        let strength = a + b + strength;
        used[n] = false;
        (length, strength)
    }).max_by(|&(length_a, strength_a), &(length_b, strength_b)| match length_a.cmp(&length_b) {
        Equal => strength_a.cmp(&strength_b),
        x => x,
    }).unwrap_or((length, 0))
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\d+)/(\d+)$").unwrap();
    let comps: Vec<(u64, u64)> = re.captures_iter(&input).map(|caps| (caps[1].parse().unwrap(), caps[2].parse().unwrap())).collect();
    println!("{}", longest(&comps, &mut vec![false; comps.len()], 0, 0).1);
}
