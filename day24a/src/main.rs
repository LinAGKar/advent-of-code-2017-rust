use std::io::{Read,stdin};

fn strongest(
    comps: &Vec<(Vec<u16>, Vec<Vec<usize>>)>,
    used: &mut Vec<bool>,
    next: &mut dyn Iterator<Item=(usize, usize)>,
) -> u16 {
    next.filter_map(|(index, port_index)| {
        if !used[index] {
            let comp = &comps[index];
            used[index] = true;
            let res = comp.0.iter().sum::<u16>() + strongest(
                comps, used, &mut comp.1[port_index].iter().filter_map(|&i| {
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
            Some(res)
        } else {
            None
        }
    }).max().unwrap_or(0)
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
        &comps, &mut vec![false; comps.len()], &mut comps.iter().enumerate().filter_map(|(i, (ports, _))| {
            (0..2).find_map(|j| {
                if ports[j] == 0 {
                    Some((i, 1 - j))
                } else {
                    None
                }
            })
        }),
    ));
}
