use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input: usize = input.trim().parse().unwrap();
    let mut buffer = vec![0];
    let mut pos = 0;
    let mut counter = 1;
    while counter <= 2017 {
        pos = (pos + input) % buffer.len() + 1;
        buffer.insert(pos, counter);
        counter += 1;
    }
    println!("{}", buffer[(pos + 1) & buffer.len()]);
}
