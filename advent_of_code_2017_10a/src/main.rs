use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut skip_size = 0;
    let mut pos = 0;
    let mut list: Vec<i64> = (0..256).collect();
    for i in input.trim().split(",") {
        let num: i64 = i.parse().unwrap();
        for j in 0..num / 2 {
            let pos1 = (pos + j) as usize % list.len();
            let pos2 = (pos + num - 1 - j) as usize % list.len();
            list.swap(pos1, pos2);
        }
        pos = (pos + num + skip_size) % (list.len() as i64);
        skip_size += 1;
    }
    println!("{}", list[0] * list[1]);
}
