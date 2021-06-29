use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let table: Vec<Vec<char>> = input.lines().map(|x| x.chars().collect()).collect();
    let mut x = table[0].iter().enumerate().filter(|&(_, &x)| x == '|').next().unwrap().0 as i64;
    let mut y = 0i64;
    let mut dx = 0i64;
    let mut dy = 1i64;
    let mut steps = 1;

    let get = |x: i64, y: i64| {
        if x < 0 || y < 0 {
            None
        } else if let Some(val) = table.get(y as usize) {
            val.get(x as usize)
        } else {
            None
        }
    };

    while let Some(&next) = get(x + dx, y + dy) {
        if next == ' ' {
            if *get(x + dy, y - dx).unwrap() != ' ' {
                let tmp = dx;
                dx = dy;
                dy = -tmp;
            } else if *get(x - dy, y + dx).unwrap() != ' ' {
                let tmp = dx;
                dx = -dy;
                dy = tmp;
            } else {
                break;
            }
        }
        steps += 1;
        x += dx;
        y += dy;
    }
    println!("{}", steps);
}
