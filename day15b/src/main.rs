use std::io;
use std::io::Read;

struct Generator {
    num: i64,
    factor: i64,
}

impl Iterator for Generator {
    type Item = i64;

    fn next(&mut self) -> Option<i64> {
        self.num = self.num * self.factor;
        self.num = (self.num & 0x7FFFFFFF) + (self.num >> 31);
        if self.num >= 0x7FFFFFFF {
            self.num -= 0x7FFFFFFF;
        }
        Some(self.num)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();
    let start_a: i64 = lines.next().unwrap().split_whitespace().nth(4).unwrap().parse().unwrap();
    let start_b: i64 = lines.next().unwrap().split_whitespace().nth(4).unwrap().parse().unwrap();
    println!("{}", Generator { num: start_a, factor: 16807 }
        .filter(|x| x % 4 == 0)
        .zip(Generator { num: start_b, factor: 48271 }.filter(|x| x % 8 == 0))
        .take(5000000).filter(|&(a, b)| a & 0xFFFF == b & 0xFFFF).count()
    );
}
