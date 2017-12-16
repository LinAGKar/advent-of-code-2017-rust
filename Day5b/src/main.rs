use std::io;
use std::io::Read;

fn main() {
    let mut string = String::new();
    io::stdin().read_to_string(&mut string).expect("Failed to read line");
    let mut jumps: Vec<i64> = string.lines().map(|x| x.parse().unwrap()).collect();
    let mut pos = 0;
    let mut counter = 0;
    while pos >= 0 && pos < jumps.len() as i64 {
        counter += 1;
        let address = jumps[pos as usize];
        jumps[pos as usize] += if address < 3 { 1 } else { -1 };
        pos += address;
    }
    println!("{}", counter);
}
