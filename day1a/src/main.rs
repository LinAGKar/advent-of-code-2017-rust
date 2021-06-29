use std::io;

fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let numbers : Vec<u32> = string.trim().chars().map(|x| x.to_digit(10).expect("Non-digit found")).collect();
    println!("{}", (0..numbers.len()).filter(|x| {
        numbers[*x] == numbers[(x + 1) % numbers.len()]
    }).map(|x| numbers[x]).sum::<u32>());
}
