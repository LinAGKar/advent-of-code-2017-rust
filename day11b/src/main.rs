use std::cmp::max;
use std::io;
use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn euclid_abs(&self) -> i64 {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{:?}", input.trim().split(",").map(|x| match x {
        "n" => Vec3 { x: 0, y: 1, z: 1 },
        "s" => Vec3 { x: 0, y: -1, z: -1 },
        "ne" => Vec3 { x: 1, y: 1, z: 0 },
        "sw" => Vec3 { x: -1, y: -1, z: 0 },
        "nw" => Vec3 { x: -1, y: 0, z: 1 },
        "se" => Vec3 { x: 1, y: 0, z: -1 },
        _ => Vec3 { x: 0, y: 0, z: 0 },
    }).fold((Vec3 { x: 0, y: 0, z: 0 }, <i64>::min_value()), |acc, x| {
        let new_pos = acc.0 + x;
        let max_dist = max(acc.1, new_pos.euclid_abs() / 2);
        (new_pos, max_dist)
    }).1);
}
