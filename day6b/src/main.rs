use std::collections::HashMap;
use std::io;

fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let mut banks: Vec<u64> = string.split_whitespace().map(|x| x.parse().unwrap()).collect();
    let mut seen = HashMap::new();
    let mut counter = 0;
    loop {
        counter += 1;
        let mut index;
        let mut blocks;
        {
            let (index_tmp, blocks_tmp) = banks.iter().enumerate().rev().max_by_key(|&(_, x)| x).unwrap();
            index = index_tmp;
            blocks = *blocks_tmp;
        }
        banks[index] = 0;
        while blocks > 0 {
            index = (index + 1) % banks.len();
            banks[index] += 1;
            blocks -= 1;
        }
        if let Some(val) = seen.insert(banks.to_vec(), counter) {
            println!("{}", counter - val);
            break;
        }
    }
}
