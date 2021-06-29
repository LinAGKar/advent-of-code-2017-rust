extern crate regex;

use regex::Regex;
use std::cmp::Ordering;
use std::io;
use std::io::Read;
use std::ops::Mul;
use std::ops::AddAssign;

#[derive(Debug, Eq, Clone, Copy)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

impl Vec3 {
    fn non_negative(&self) -> bool {
        self.x >= 0 && self.y >= 0 && self.z >= 0
    }
}

impl Ord for Vec3 {
    fn cmp(&self, other: &Vec3) -> Ordering {
        (self.x.abs() + self.y.abs() + self.z.abs()).cmp(&(other.x.abs() + other.y.abs() + other.z.abs()))
    }
}

impl PartialOrd for Vec3 {
    fn partial_cmp(&self, other: &Vec3) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Vec3) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        Vec3 { x: self.x * other.x, y: self.y * other.y, z: self.z * other.z }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

#[derive(Debug, Clone, Copy, Eq)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    fn step(&mut self) {
        self.vel += self.acc;
        self.pos += self.vel;
    }
}

impl Ord for Particle {
    fn cmp(&self, other: &Particle) -> Ordering {
        let mut this = *self;
        let mut other = *other;
        if this.acc != other.acc {
            this.acc.cmp(&other.acc)
        } else {
            while !(this.vel * this.acc).non_negative() || !(other.vel * other.acc).non_negative() {
                this.step();
                other.step();
            }
            if this.vel != other.vel {
                this.vel.cmp(&other.vel)
            } else {
                while !(this.pos * this.vel).non_negative() || !(other.pos * other.vel).non_negative() {
                    this.step();
                    other.step();
                }
                this.pos.cmp(&other.pos)
            }
        }
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        match self.cmp(other) { Ordering::Equal => true, _ => false }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>").unwrap();
    println!("{}", re.captures_iter(&input).map(|caps| Particle {
        pos: Vec3 { x: caps[1].parse().unwrap(), y: caps[2].parse().unwrap(), z: caps[3].parse().unwrap() },
        vel: Vec3 { x: caps[4].parse().unwrap(), y: caps[5].parse().unwrap(), z: caps[6].parse().unwrap() },
        acc: Vec3 { x: caps[7].parse().unwrap(), y: caps[8].parse().unwrap(), z: caps[9].parse().unwrap() },
    }).enumerate().min_by(|&(_, ref x), &(_, ref y)| x.cmp(y)).unwrap().0);
}
