use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: usize = input.trim().parse().unwrap();
    let mut pos = 0;
    let mut counter = 1;
    let mut zero_pos = 0;
    let mut after_zero = 0;
    while counter <= 50000000 {
        pos = (pos + input) % counter + 1;
        if pos == (zero_pos + 1) % counter {
            after_zero = counter;
        }
        if pos <= zero_pos {
            zero_pos += 1;
        }
        counter += 1;
    }
    println!("{}", after_zero);
}
