use std::collections::HashMap;
use std::io;

struct CoordinateMap {
    x: i64,
    y: i64,
    values: HashMap<(i64, i64), u64>,
}

impl CoordinateMap {
    fn new() -> CoordinateMap {
        CoordinateMap { x: 0, y: 0, values: [((0, 0), 1)].iter().cloned().collect() }
    }

    fn move_by(&mut self, x: i64, y: i64) {
        self.x += x;
        self.y += y;
    }

    fn get(&self) -> u64 {
        self.get_at(self.x, self.y)
    }

    fn get_at(&self, x: i64, y: i64) -> u64 {
        match self.values.get(&(x, y)) {
            Some(val) => *val,
            None => 0,
        }
    }

    fn set(&mut self, val: u64) {
        self.values.insert((self.x, self.y), val);
    }

    fn set_to_surrounding_sum(&mut self) {
        let sum: u64 = (-1..2).map::<u64, _>(|x| (-1..2).filter(|y| (x, *y) != (0, 0)).map(|y| self.get_at(self.x + x, self.y + y)).sum()).sum();
        self.set(sum)
    }

    fn check_and_print_if_greq(&self, num: u64) -> bool {
        let val = self.get();
        let greater = val >= num;
        if greater {
            println!("{}", val);
        }
        greater
    }
}

fn main() {
    let mut string = String::new();
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let number: u64 = string.trim().parse().unwrap();
    let mut map = CoordinateMap::new();
    let mut move_distance = 1;
    let mut dir = (1, 0);
    'mainLoop: loop {
        for _ in 0..2 {
            let (x, y) = dir;
            for _ in 0..move_distance {
                if map.check_and_print_if_greq(number) { break 'mainLoop; }
                map.move_by(x, y);
                map.set_to_surrounding_sum();
            }
            dir = (-y, x);
        }
        move_distance += 1;
    }
}
