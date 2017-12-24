extern crate regex;

use regex::Regex;
use std::io;
use std::io::Read;


fn strongest(comps: &Vec<(u64, u64)>, used: &mut Vec<bool>, end: u64) -> u64 {
    let to_check: Vec<(usize, &(u64, u64))> = comps.iter().enumerate().filter(|&(n, &(a, b))| !used[n] && (a == end || b == end)).collect();
    to_check.iter().map(|&(n, &(a, b))| {
        used[n] = true;
        let res = a + b + strongest(comps, used, if a == end { b } else { a });
        used[n] = false;
        res
    }).max().unwrap_or(0)
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\d+)/(\d+)$").unwrap();
    let comps: Vec<(u64, u64)> = re.captures_iter(&input).map(|caps| (caps[1].parse().unwrap(), caps[2].parse().unwrap())).collect();
    println!("{}", strongest(&comps, &mut vec![false; comps.len()], 0));
}
