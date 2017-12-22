extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;


fn string_to_grid(x: &str) -> Vec<bool> {
    x.chars().filter_map(|x| match x { '#' => Some(true), '.' => Some(false), _ => None }).collect()
}


fn rotate(input: &Vec<bool>) -> Vec<bool> {
    let size_f = (input.len() as f64).sqrt();
    let size = size_f as usize;
    if size as f64 != size_f {
        panic!("Grid is not square");
    }
    (0..input.len()).map(|i| {
        let x = i % size;
        let y = i / size;
        let old_x = y;
        let old_y = size - 1 - x;
        input[old_y * size + old_x]
    }).collect()
}


fn flip(input: &Vec<bool>) -> Vec<bool> {
    let size_f = (input.len() as f64).sqrt();
    let size = size_f as usize;
    if size as f64 != size_f {
        panic!("Grid is not square");
    }
    (0..input.len()).map(|i| {
        let x = i % size;
        let y = i / size;
        let old_x = size - 1 - x;
        input[y * size + old_x]
    }).collect()
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^([.#]{2}/[.#]{2}) => ([.#]{3}(?:/[.#]{3}){2})$|^([.#]{3}(?:/[.#]{3}){2}) => ([.#]{4}(?:/[.#]{4}){3})$").unwrap();
    let mut mapping: HashMap<Vec<bool>, Vec<bool>> = re.captures_iter(&input).filter_map(|caps| {
        if let (Some(from), Some(to)) = (caps.get(1), caps.get(2)) {
            Some((string_to_grid(from.as_str()), string_to_grid(to.as_str())))
        } else if let (Some(from), Some(to)) = (caps.get(3), caps.get(4)) {
            Some((string_to_grid(from.as_str()), string_to_grid(to.as_str())))
        } else {
            None
        }
    }).collect();
    let mut to_add = HashMap::new();
    for (from, to) in &mapping {
        let mut from = from.to_owned();
        for _ in 0..3 {
            from = rotate(&from);
            if !mapping.contains_key(&from) && !to_add.contains_key(&from) {
                to_add.insert(from.to_owned(), to.to_owned());
            }
        }
        from = flip(&from);
        if !mapping.contains_key(&from) && !to_add.contains_key(&from) {
            to_add.insert(from.to_owned(), to.to_owned());
        }
        for _ in 0..3 {
            from = rotate(&from);
            if !mapping.contains_key(&from) && !to_add.contains_key(&from) {
                to_add.insert(from.to_owned(), to.to_owned());
            }
        }
    }
    mapping.extend(to_add);
    let mut image = vec![false, true, false, false, false, true, true, true, true];
    for _ in 0..18 {
        let size_f = (image.len() as f64).sqrt();
        let size = size_f as usize;
        if size as f64 != size_f {
            panic!("Grid is not square");
        }
        let block_size = if size % 2 == 0 { 2 } else { 3 };
        let new_block_size = block_size + 1;
        let blocks = size / block_size;
        let mut new_image = vec![false; (new_block_size * blocks).pow(2)];
        for i in 0..blocks.pow(2) {
            let x = i % blocks;
            let y = i / blocks;
            let mut old_block = vec![false; block_size.pow(2)];
            for j in 0..block_size.pow(2) {
                let block_x = j % block_size;
                let block_y = j / block_size;
                let x = x * block_size + block_x;
                let y = y * block_size + block_y;
                old_block[j] = image[y * size + x];
            }
            let new_block = &mapping[&old_block];
            for j in 0..new_block_size.pow(2) {
                let block_x = j % new_block_size;
                let block_y = j / new_block_size;
                let x = x * new_block_size + block_x;
                let y = y * new_block_size + block_y;
                new_image[y * blocks * new_block_size + x] = new_block[j];
            }
        }
        image = new_image;
    }
    println!("{}", image.iter().filter(|&x| *x).count());
}
