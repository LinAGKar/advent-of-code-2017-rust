extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;


struct Node {
    name: String,
    children: Vec<&Node>,
    weight: u64,
}


fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Failed to read line");
    let re = Regex::new(r"(\w+) \((\d+)\) -> ((?:\w+, )*\w+)").expect("Failed to compile regex");
//     let mut children = HashSet::new();
    let mut programs = HashMap::new();
    for i in string.lines() {
        if let Some(cap) = re.captures(i) {
            for j in cap[3].split(", ") {
                if !programs.contains() {
                    let node = Node { name: j.to_string(), children: vec![], weight: 0 };
//                     children.insert(j.to_string(), );
                }
            }
            
            if !programs.contains(cap[1]) {
//                 let node = Node { name: cap[1].to_string(), children };
//                 programs.insert(, );
            }
            

        }
    }
}
