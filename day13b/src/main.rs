use regex::Regex;
use std::collections::{HashMap, HashSet};
use std::io::Read;

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else if a == 0 {
        b
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a * b).abs() / gcd(a, b)
}

fn main() {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"(?m)^(\d+): (\d+)$").unwrap();

    let layers: Vec<(i32, i32)> = re
        .captures_iter(&input)
        .map(|caps| {
            let depth: i32 = caps[1].parse().unwrap();
            let range: i32 = caps[2].parse().unwrap();
            let period = range * 2 - 2;
            (period, ((-depth) % period + period) % period)
        })
        .collect();

    let mut allowed_offsets = HashMap::new();

    for (period, offset) in layers {
        (*allowed_offsets.entry(period).or_insert_with(|| {
            (0..period).collect::<HashSet<_>>()
        })).remove(&offset);
    }

    let keys: Vec<_> = allowed_offsets.keys().copied().collect();
    let mut to_remove = HashSet::new();

    for &i in &keys {
        for &j in &keys {
            if i != j && j % i == 0 {
                to_remove.insert(i);
                let allowed = &allowed_offsets[&i];
                let forbidden: Vec<_> = (0..i).filter(|x| !allowed.contains(x)).collect();
                let allowed = allowed_offsets.get_mut(&j).unwrap();
                for k in forbidden {
                    for l in 0..j / i {
                        (*allowed).remove(&(l * i + k));
                    }
                }
            }
        }
    }

    for i in to_remove {
        allowed_offsets.remove(&i);
    }

    let mut congruences = Vec::new();

    let allowed_offsets: Vec<_> = allowed_offsets.into_iter().filter_map(|(period, allowed)| {
        if allowed.len() == 1 {
            congruences.push((period, allowed.iter().next().copied().unwrap()));
            None
        } else {
            Some((period, allowed))
        }
    }).collect();

    let (period, offset) = congruences.into_iter().reduce(|(prev_period, prev_offset), (period, offset)| {
        let offset = ((offset - prev_offset) % period + period) % period;
        let offset = (0..).find(|x| (prev_period * x) % period == offset).unwrap();
        (lcm(prev_period as i64, period as i64) as i32, prev_period * offset + prev_offset)
    }).unwrap();

    println!("{}", (0..).map(|x| x * period + offset).find(|x| {
        allowed_offsets.iter().all(|(period, allowed)| {
            allowed.contains(&(x % period))
        })
    }).unwrap());
}
