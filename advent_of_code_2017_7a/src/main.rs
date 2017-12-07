extern crate regex;

use regex::Regex;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Failed to read line");
    let re = Regex::new(r"(\w+) \((\d+)\) -> ((?:\w+, )*\w+)").expect("Failed to compile regex");
    let mut children = HashSet::new();
    let mut programs = HashSet::new();
    for i in string.lines() {
        if let Some(cap) = re.captures(i) {
            programs.insert(cap[1].to_string());
            for j in cap[3].split(", ") {
                children.insert(j.to_string());
            }
        }
    }
    for i in programs {
        if !children.contains(&i) {
            println!("{}", i);
        }
    }
}
