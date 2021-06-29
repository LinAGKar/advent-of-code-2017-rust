use std::collections::HashMap;
use std::collections::vec_deque::VecDeque;
use std::io;
use std::io::Read;

#[derive(Debug)]
struct Instr {
    write: bool,
    motion: i64,
    next_state: char,
}

impl Instr {
    fn new() -> Instr {
        Instr {
            write: false,
            motion: 0,
            next_state: '\0',
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut curr_state = '\0';
    let mut run_time: u64 = 0;
    if input.lines().count() < 2 {
        panic!("Not enough lines in input");
    }
    let mut reading_state = '\0';
    let mut states = HashMap::new();
    for (n, i) in input.lines().enumerate() {
        if n == 0 {
            curr_state = i.split_whitespace().nth(3).unwrap().chars().next().unwrap();
        } else if n == 1 {
            run_time = i.split_whitespace().nth(5).unwrap().parse().unwrap()
        } else {
            match (n - 2) % 10 {
                1 => {
                    reading_state = i.split_whitespace().nth(2).unwrap().chars().next().unwrap();
                    states.insert(reading_state, (Instr::new(), Instr::new()));
                }

                3 => {
                    states.get_mut(&reading_state).unwrap().0.write = i.split_whitespace().nth(4).unwrap() == "1.";
                }

                4 => {
                    states.get_mut(&reading_state).unwrap().0.motion = if i.split_whitespace().nth(6).unwrap() == "right." { 1 } else { -1 };
                }

                5 => {
                    states.get_mut(&reading_state).unwrap().0.next_state = i.split_whitespace().nth(4).unwrap().chars().next().unwrap();
                }

                7 => {
                    states.get_mut(&reading_state).unwrap().1.write = i.split_whitespace().nth(4).unwrap() == "1.";
                }

                8 => {
                    states.get_mut(&reading_state).unwrap().1.motion = if i.split_whitespace().nth(6).unwrap() == "right." { 1 } else { -1 };
                }

                9 => {
                    states.get_mut(&reading_state).unwrap().1.next_state = i.split_whitespace().nth(4).unwrap().chars().next().unwrap();
                }

                _ => {}
            }
        }
    }

    let mut pos = 0i64;
    let mut mem = VecDeque::new();
    mem.push_back(false);

    for _ in 0..run_time {
        let instr = &states[&curr_state];
        let instr = if mem[pos as usize] { &instr.1 } else { &instr.0 };
        mem[pos as usize] = instr.write;
        pos += instr.motion;
        while pos < 0 {
            mem.push_front(false);
            pos += 1;
        }
        while pos as usize >= mem.len() {
            mem.push_back(false);
        }
        curr_state = instr.next_state;
    }

    println!("{}", mem.iter().filter(|&x| *x).count());
}
