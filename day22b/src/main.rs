use std::collections::HashMap;
use std::io;
use std::io::Read;

enum NodeStatus {
    Clean,
    Infected,
    Weakened,
    Flagged,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut nodes = HashMap::new();
    let lines = input.lines().count() as i16;
    for i in input.lines().enumerate() {
        let columns = i.1.chars().count() as i16;
        for j in i.1.chars().enumerate() {
            if j.1 == '#' {
                nodes.insert((j.0 as i16 - columns / 2, i.0 as i16 - lines / 2), NodeStatus::Infected);
            }
        }
    }
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;
    let mut counter = 0;
    for _ in 0..10000000 {
        let status = nodes.entry((x, y)).or_insert(NodeStatus::Clean);
        match *status {
            NodeStatus::Infected => {
                let tmp = dy;
                dy = dx;
                dx = -tmp;
                *status = NodeStatus::Flagged;
            }
            NodeStatus::Weakened => {
                counter += 1;
                *status = NodeStatus::Infected;
            }
            NodeStatus::Flagged => {
                dx = -dx;
                dy = -dy;
                *status = NodeStatus::Clean;
            }
            NodeStatus::Clean => {
                let tmp = dy;
                dy = -dx;
                dx = tmp;
                *status = NodeStatus::Weakened;
            }
        }
        x += dx;
        y += dy;
    }
    println!("{}", counter);
}
