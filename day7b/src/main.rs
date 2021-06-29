extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Node {
    name: String,
    weight: i64,
    own_weight: i64,
    children: Vec<Node>,
}

impl Node {
    fn new(name: &str, programs: &HashMap<String, (i64, HashSet<String>)>) -> Node {
        let &(weight, ref children) = programs.get(name).expect(
            &format!("Did not find program {} in program list", name),
        );
        let children: Vec<Node> = children.iter().map(|x| Node::new(x, programs)).collect();
        let total_weight = weight + children.iter().map(|x| x.weight).sum::<i64>();
        Node {
            name: name.to_string(),
            children: children,
            own_weight: weight,
            weight: total_weight,
        }
    }

    fn balance(&self, difference: Option<i64>) -> Option<i64> {
        match self.children.len() {
            0 => match difference {
                Some(diff) => Some(self.weight + diff),
                None => None,
            }

            1 => self.children[0].balance(difference),

            2 => {
                let child_diff = self.children[1].weight - self.children[0].weight;
                match difference {
                    Some(diff) => {
                        if child_diff == 0 {
                            Some(self.own_weight + diff)
                        } else if diff == child_diff {
                            self.children[0].balance(difference)
                        } else if diff == -child_diff {
                            self.children[1].balance(difference)
                        } else {
                            None
                        }
                    },
                    None => match self.children[0].balance(Some(child_diff)) {
                        Some(x) => Some(x),
                        None => self.children[1].balance(Some(-child_diff)),
                    }
                }
            }

            _ => {
                let mut result = None;
                for i in self.children.iter().enumerate() {
                    let differences: Vec<i64> = self
                        .children
                        .iter()
                        .enumerate()
                        .filter(|x| x.0 != i.0)
                        .map(|x| x.1.weight - i.1.weight)
                        .collect();
                    let first = differences[0];
                    if differences.iter().any(|y| *y != first) {
                        continue;
                    }
                    if let Some(diff) = difference {
                        if first == 0 {
                            result = Some(self.own_weight + diff);
                            break;
                        }
                        if diff != first {
                            continue;
                        }
                    }
                    result = i.1.balance(Some(first));
                    if let Some(_) = result {
                        break;
                    }
                }
                result
            }
        }
    }
}

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Failed to read line");
    let re = Regex::new(r"(\w+) \((\d+)\)(?: -> ((?:\w+, )*\w+))?").expect("Failed to compile regex");
    let mut children = HashSet::new();
    let mut programs = HashMap::new();
    for i in string.lines() {
        if let Some(cap) = re.captures(i) {
            let mut node = (cap[2].parse::<i64>().unwrap(), HashSet::new());
            if let Some(node_children) = cap.get(3) {
                for j in node_children.as_str().split(", ") {
                    children.insert(j.to_string());
                    node.1.insert(j.to_string());
                }
            }
            programs.insert(cap[1].to_string(), node);
        }
    }
    for name in programs.keys() {
        if !children.contains(name) {
            let root = Node::new(name, &programs);
            if let Some(x) = root.balance(None) {
                println!("{}", x);
            }
            break;
        }
    }
}
