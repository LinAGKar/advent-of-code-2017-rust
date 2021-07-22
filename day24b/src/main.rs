use std::io::{Read,stdin};

fn strongest(
    comps: &Vec<(Vec<u16>, Vec<Vec<usize>>)>,
    used: &mut Vec<bool>,
    length: u8,
    next: &mut dyn Iterator<Item=(usize, usize)>,
) -> (u8, u16) {
    next.filter_map(|(index, port_index)| {
        if !used[index] {
            let comp = &comps[index];
            used[index] = true;
            let (this_len, strength) = strongest(
                comps, used, length + 1, &mut comp.1[port_index].iter().filter_map(|&i| {
                    let next_comp = &comps[i];
                    (0..2).find_map(|j| {
                        if next_comp.0[j] == comp.0[port_index] {
                            Some((i, 1 - j))
                        } else {
                            None
                        }
                    })
                }),
            );
            used[index] = false;
            Some((this_len, comp.0.iter().sum::<u16>() + strength))
        } else {
            None
        }
    }).max().unwrap_or((length, 0))
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let comps: Vec<Vec<u16>> = input.lines().map(|line| {
        line.split('/').map(|num| num.parse().unwrap()).collect()
    }).collect();
    let comps: Vec<_> = comps.iter().enumerate().map(|(i, ports)| {
        (
            ports.clone(),
            ports.iter().map(|port| {
                comps.iter().enumerate().filter_map(|(j, ports)| if j != i && ports.contains(port) {
                    Some(j)
                } else {
                    None
                }).collect::<Vec<_>>()
            }).collect::<Vec<_>>(),
        )
    }).collect();

    println!("{}", strongest(
        &comps, &mut vec![false; comps.len()], 0, &mut comps.iter().enumerate().filter_map(|(i, (ports, _))| {
            (0..2).find_map(|j| {
                if ports[j] == 0 {
                    Some((i, 1 - j))
                } else {
                    None
                }
            })
        }),
    ).1);
}
