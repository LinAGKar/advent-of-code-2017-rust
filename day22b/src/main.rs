use std::collections::HashSet;
use std::io;
use std::io::Read;


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut infected = HashSet::new();
    let lines = input.lines().count() as i64;
    for i in input.lines().enumerate() {
        let columns = i.1.chars().count() as i64;
        for j in i.1.chars().enumerate() {
            if j.1 == '#' {
                infected.insert((j.0 as i64 - columns / 2, i.0 as i64 - lines / 2));
            }
        }
    }
    let mut x = 0;
    let mut y = 0;
    let mut dx = 0;
    let mut dy = -1;
    let mut counter = 0;
    let mut weakened = HashSet::new();
    let mut flagged = HashSet::new();
    for _ in 0..10000000 {
        if infected.contains(&(x, y)) {
            let tmp = dy;
            dy = dx;
            dx = -tmp;
            infected.remove(&(x, y));
            flagged.insert((x, y));
        } else if weakened.contains(&(x, y)) {
            counter += 1;
            weakened.remove(&(x, y));
            infected.insert((x, y));
        } else if flagged.contains(&(x, y)) {
            dx = -dx;
            dy = -dy;
            flagged.remove(&(x, y));
        } else {
            let tmp = dy;
            dy = -dx;
            dx = tmp;
            weakened.insert((x, y));
        }
        x += dx;
        y += dy;
    }
    println!("{}", counter);
}
