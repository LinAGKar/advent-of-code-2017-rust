extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut connections = HashMap::new();
    let re = Regex::new(r"(\d+) <-> (\d+(?:, \d+)*)").unwrap();
    for i in input.lines() {
        if let Some(caps) = re.captures(i) {
            connections.insert(caps[1].parse::<u64>().unwrap(), caps[2].split(", ").map(|x| x.parse::<u64>().unwrap()).collect::<Vec<u64>>());
        }
    }
    let mut not_visited: HashSet<&u64> = connections.keys().collect();
    let mut visited = HashSet::new();
    let mut count = 0;
    while !not_visited.is_empty() {
        count += 1;
        let mut visit_next = vec![**not_visited.iter().next().unwrap()];
        while !visit_next.is_empty() {
            for i in &visit_next {
                visited.insert(*i);
                not_visited.remove(i);
            }
            visit_next = visit_next.iter().flat_map(|x| connections[x].iter().filter(|y| !visited.contains(y)).map(|&y| y)).collect();
        }
    }
    println!("{}", count);
}
