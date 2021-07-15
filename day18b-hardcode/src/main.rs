use std::io::Read;

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();

    let mut lines = input.lines().map(|line| line.split_whitespace());
    let count: u8 = lines.nth(8).unwrap().nth(2).unwrap().parse().unwrap();
    let seed: i64 = lines.next().unwrap().nth(2).unwrap().parse().unwrap();
    let a: i64 = lines.next().unwrap().nth(2).unwrap().parse().unwrap();
    let b: i64 = lines.nth(1).unwrap().nth(2).unwrap().parse().unwrap();
    let c: i64 = lines.next().unwrap().nth(2).unwrap().parse().unwrap();
    let d: i64 = lines.nth(2).unwrap().nth(2).unwrap().parse().unwrap();
    const MOD: i64 = i32::MAX as i64;
    let mut nums = Vec::new();
    (0..count).fold(seed, |p, _| {
        let p = (p * a % MOD * b + c) % MOD;
        nums.push(p % d);
        p
    });
    println!("{}", ((0..).find(|_| {
        let mut sorted = true;
        for j in 0..nums.len() - 1 {
            if nums[j] < nums[j + 1] {
                nums.swap(j, j + 1);
                sorted = false;
            }
        }
        sorted
    }).unwrap() / 2 + 1) * nums.len());
}
