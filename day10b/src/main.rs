use std::io;
use std::io::Read;

fn main() {
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let mut input: Vec<u8> = input.iter().take_while(|&x| *x != 0x0A).map(|&x| x).collect();
    input.append(&mut vec![17, 31, 73, 47, 23]);
    let mut skip_size = 0;
    let mut pos = 0;
    let mut list: Vec<u8> = (0..256).map(|x| x as u8).collect();
    for _ in 0..64 {
        for j in input.iter() {
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
    println!("{}", (0..16).map(|x| list[x * 16..(x + 1) * 16].iter().fold(0, |acc, &y| acc ^ y)).map(|x| format!("{:02x}", x)).collect::<Vec<String>>().join(""));
}
