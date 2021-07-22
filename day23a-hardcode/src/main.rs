use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().map(|line| line.split_whitespace());
    let num: u16 = lines.next().unwrap().nth(2).unwrap().parse().unwrap();
    println!("{}", (num - 2).pow(2));
}
