use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().map(|line| line.split_whitespace());
    let num: i32 = lines.next().unwrap().nth(2).unwrap().parse().unwrap();
    let factor_b: i32 = lines.nth(3).unwrap().nth(2).unwrap().parse().unwrap();
    let offset_b: i32 = lines.next().unwrap().nth(2).unwrap().parse().unwrap();
    let offset_c: i32 = lines.nth(1).unwrap().nth(2).unwrap().parse().unwrap();
    let increment: i32 = lines.nth(22).unwrap().nth(2).unwrap().parse().unwrap();

    let mut number_of_non_primes = 0;
    let mut curr = num * factor_b - offset_b;
    let stop = curr - offset_c;
    while curr <= stop {
        if (2..(curr as f64).sqrt() as i32 + 1).any(|x| curr % x == 0) {
            number_of_non_primes += 1;
        }
        curr -= increment;
    }
    println!("{}", number_of_non_primes);
}
