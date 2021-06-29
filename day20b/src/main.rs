extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::f64;
use std::io;
use std::io::Read;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

fn approx_eq(a: f64, b: f64) -> bool {
    a * 0.9999999999999 < b && b < a * 1.0000000000001
}

fn intersections(p_1: f64, v_1: f64, a_1: f64, p_2: f64, v_2: f64, a_2: f64) -> Vec<f64> {
    if a_1 != a_2 {
        let half_p = 0.5 + (v_1 - v_2) / (a_1 - a_2);
        let q = 2.0 * (p_1 - p_2) / (a_1 - a_2);
        let root = (half_p * half_p - q).sqrt();
        let t_1 = -half_p + root;
        let t_2 = -half_p - root;
        if t_1 == t_2 {
            vec![t_1]
        } else if !t_1.is_nan() && !t_1.is_nan() {
            vec![t_1, t_2]
        } else {
            vec![]
        }
    } else if v_1 != v_2 {
        vec![(p_2 - p_1) / (v_1 - v_2)]
    } else {
        if p_1 == p_2 {
            vec![f64::NAN]
        } else {
            vec![]
        }
    }
}

fn eq_or_nan(a: f64, b: f64, c: f64) -> Option<f64> {
    if a.is_nan() {
        if b.is_nan() {
            if c.is_nan() {
                Some(0.0)
            } else {
                Some(c)
            }
        } else if c.is_nan() || approx_eq(b, c) {
            Some(b)
        } else {
            None
        }
    } else if b.is_nan() {
        if c.is_nan() || approx_eq(a, c) {
            Some(a)
        } else {
            None
        }
    } else if c.is_nan() {
        if approx_eq(a, b) {
            Some(a)
        } else {
            None
        }
    } else if approx_eq(a, b) && approx_eq(b, c) {
        Some(a)
    } else {
        None
    }
}

#[derive(Debug, Clone, Copy)]
struct Particle {
    pos: Vec3,
    vel: Vec3,
    acc: Vec3,
}

impl Particle {
    fn intersects(&self, other: &Particle) -> Option<i64> {
        let ix = intersections(self.pos.x, self.vel.x, self.acc.x, other.pos.x, other.vel.x, other.acc.x);
        let iy = intersections(self.pos.y, self.vel.y, self.acc.y, other.pos.y, other.vel.y, other.acc.y);
        let iz = intersections(self.pos.z, self.vel.z, self.acc.z, other.pos.z, other.vel.z, other.acc.z);
        let mut i = Vec::new();
        for &x in ix.iter() {
            for &y in iy.iter() {
                for &z in iz.iter() {
                    if let Some(n) = eq_or_nan(x, y, z) {
                        if approx_eq(n.floor(), n) {
                            i.push(n as i64);
                        }
                    }
                }
            }
        }
        i.sort();
        i.iter().next().map(|&x| x)
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(
        r"p=<(-?\d+),(-?\d+),(-?\d+)>, v=<(-?\d+),(-?\d+),(-?\d+)>, a=<(-?\d+),(-?\d+),(-?\d+)>",
    ).unwrap();
    let particles: Vec<Particle> = re.captures_iter(&input).map(|caps| Particle {
        pos: Vec3 { x: caps[1].parse().unwrap(), y: caps[2].parse().unwrap(), z: caps[3].parse().unwrap() },
        vel: Vec3 { x: caps[4].parse().unwrap(), y: caps[5].parse().unwrap(), z: caps[6].parse().unwrap() },
        acc: Vec3 { x: caps[7].parse().unwrap(), y: caps[8].parse().unwrap(), z: caps[9].parse().unwrap() },
    }).collect();
    let mut intersections = HashMap::new();
    for i in particles.iter().enumerate() {
        for j in particles.iter().enumerate() {
            if j.0 > i.0 {
                match i.1.intersects(j.1) {
                    Some(val) => {
                        if !intersections.contains_key(&val) {
                            intersections.insert(val, Vec::new());
                        }
                        intersections.get_mut(&val).unwrap().push((i.0, j.0));
                    }
                    None => {}
                }
            }
        }
    }
    let mut destroyed = HashSet::new();
    let mut times: Vec<&i64> = intersections.keys().collect();
    times.sort();
    for i in &times {
        let mut to_destroy = HashSet::new();
        for j in &intersections[i] {
            if !destroyed.contains(&j.0) && !destroyed.contains(&j.1) {
                to_destroy.insert(j.0);
                to_destroy.insert(j.1);
            }
        }
        for &j in &to_destroy {
            destroyed.insert(j);
        }
    }
    println!("{}", particles.len() - destroyed.len());
}
