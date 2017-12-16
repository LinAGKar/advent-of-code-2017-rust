use std::io;

fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let register: i64 = string.trim().parse().unwrap();
    if register < 1 {
        panic!("Register must be at least 1");
    }
    println!("{}", if register == 1 { 0 } else {
        let ring = ((((register - 1) as f64).sqrt() - 1.0) / 2.0 + 1.0) as i64;
        let first = (1 + 2 * (ring - 1)).pow(2) + 1;
        let ring_size = ring * 8;
        let pos_on_side = (register - first) % (ring_size / 4);
        (pos_on_side - ring + 1).abs() + ring
    });
}
