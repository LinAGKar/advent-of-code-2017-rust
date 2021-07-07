use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: usize = input.trim().parse().unwrap();

    let mut pos = 0;
    let mut after_zero = 0;
    for i in 1..=50000000 {
        pos = (pos + input) % i + 1;
        if pos == 1 {
            after_zero = i;
        }
    }
    println!("{}", after_zero);
}
