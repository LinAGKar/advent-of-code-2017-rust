extern crate regex;

use regex::Regex;
use std::io;
use std::str;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut programs: Vec<u8> = (97..113).collect();
    let re = Regex::new(r"s(\d+)|x(\d+)/(\d+)|p(\w)/(\w)").unwrap();
    for i in input.split(',') {
        if let Some(caps) = re.captures(i) {
            if let Some(count) = caps.get(1) {
                let count: usize = count.as_str().parse().unwrap();
                let mut new = programs[programs.len()-count..].to_vec();
                new.extend_from_slice(&programs[..programs.len()-count]);
                programs = new;
            } else if let (Some(a), Some(b)) = (caps.get(2), caps.get(3)) {
                programs.swap(a.as_str().parse().unwrap(), b.as_str().parse().unwrap());
            } else if let (Some(a), Some(b)) = (caps.get(4), caps.get(5)) {
                let a = a.as_str().bytes().next().unwrap();
                let b = b.as_str().bytes().next().unwrap();
                programs = programs.iter().map(|&x| if x == a { b } else if x == b { a } else { x }).collect();
            }
        }
    }
    println!("{}", str::from_utf8(&programs).unwrap());
}
