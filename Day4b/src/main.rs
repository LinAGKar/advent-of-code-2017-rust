use std::collections::HashSet;
use std::io;
use std::io::Read;

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Failed to read line");
    println!("{}", string.lines().map(|x| {
        let mut in_phrase = HashSet::new();
        for i in x.split_whitespace() {
            let mut word: Vec<char> = i.chars().collect();
            word.sort_by(|a, b| b.cmp(a));
            if !in_phrase.insert(word) {
                return false;
            }
        }
        true
    }).filter(|x| *x).count());
}
