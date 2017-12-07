use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Failed to read line");
    println!("{}", string.lines().map(|x| {
        let mut in_phrase = HashSet::new();
        for i in x.split_whitespace() {
            if !in_phrase.insert(i) {
                return false;
            }
        }
        true
    }).filter(|x| *x).count());
}
