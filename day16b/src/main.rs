extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::str;

const LENGTH: usize = 16;


fn calc_dance(mapping: &[usize]) -> Vec<usize> {
    let mut state: Vec<usize> = (0..LENGTH).collect();
    let start: Vec<usize> = (0..LENGTH).collect();
    let mut counter = 0;
    let cycles = 1000000000;
    while counter < cycles {
        state = (0..LENGTH).map(|x| state[mapping[x as usize]]).collect();
        counter += 1;
        if state == start {
            counter = cycles / counter * counter;
        }
    }
    state
}


fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut programs: Vec<usize> = (0..LENGTH).collect();
    let mut indexes: Vec<usize> = (0..LENGTH).collect();
    let re = Regex::new(r"s(\d+)|x(\d+)/(\d+)|p([a-p])/([a-p])").unwrap();
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
                indexes.swap(a.as_str().bytes().next().unwrap() as usize - 97, b.as_str().bytes().next().unwrap() as usize - 97);
            }
        }
    }
    programs = calc_dance(&programs);
    let partner_swaps: HashMap<usize, usize> = calc_dance(&indexes).iter().enumerate().map(|(index, &val)| (val, index)).collect();
    let programs: Vec<u8> = programs.iter().map(|x| partner_swaps[x] as u8 + 97).collect();
    println!("{}", str::from_utf8(&programs).unwrap());
}
