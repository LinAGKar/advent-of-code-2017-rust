use std::collections::HashSet;
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
    let mut used = HashSet::new();

    for i in 0..128 {
        let hash = knot_hash(&format!("{}-{}", input.trim(), i).bytes().collect());
        for (n, j) in hash.iter().rev().enumerate() {
            for k in (0i16..8i16).filter(|x| (j >> x) & 1u8 == 1u8) {
                used.insert((i, ((n as i16) << 3) | k));
            }
        }
    }

    let mut counter = 0;
    while !used.is_empty() {
        counter += 1;
        let mut visit = vec![*used.iter().next().unwrap()];
        used.remove(&visit[0]);
        while !visit.is_empty() {
            let mut visit_next = Vec::new();
            for (x, y) in visit {
                for j in vec![(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)].iter() {
                    if let Some(&coord) = used.get(j) {
                        visit_next.push(coord);
                        used.remove(&coord);
                    }
                }
            }
            visit = visit_next;
        }
    }

    println!("{}", counter);
}
