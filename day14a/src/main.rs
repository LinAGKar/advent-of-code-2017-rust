use std::io;

fn knot_hash(input: &Vec<u8>) -> Vec<u8> {
    let mut skip_size = 0;
    let mut pos = 0;
    let mut list: Vec<u8> = (0..256).map(|x| x as u8).collect();
    let end = vec![17, 31, 73, 47, 23];
    for _ in 0..64 {
        for j in vec![input, &end].iter().flat_map(|x| x.iter()) {
            let num = *j as u16;
            for k in 0..num / 2 {
                let pos1 = (pos + k) as usize % list.len();
                let pos2 = (pos + num - 1 - k) as usize % list.len();
                list.swap(pos1, pos2);
            }
            pos = (pos + num + skip_size) % (list.len() as u16);
            skip_size += 1;
        }
    }
    (0..16).map(|x| list[x * 16..(x + 1) * 16].iter().fold(0, |acc, &y| acc ^ y)).collect()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    println!("{}",
        (0..128).map(
            |x| knot_hash(&format!(
                "{}-{}",
                input.trim(),
                x,
            ).bytes().collect())
                .iter()
                .map(|y| (0..8).fold(0, |acc, z| acc + ((y >> z) & 1u8) as i32))
                .fold(0, |acc, y| acc + y)
        ).fold(0, |acc, x| acc + x)
    );
}
