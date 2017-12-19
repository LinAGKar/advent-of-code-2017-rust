extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;


#[derive(Debug)]
enum Instr {
    SND(char),
    SET(char, char),
    SETI(char, i64),
    ADD(char, char),
    ADDI(char, i64),
    MUL(char, char),
    MULI(char, i64),
    MOD(char, char),
    MODI(char, i64),
    RCV(char),
    JGZ(char, char),
    JGZI(char, i64),
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to get input");
    let re = Regex::new(r"snd (\w)|set ([a-z]) (?:([a-z])|(-?\d+))|add ([a-z]) (?:([a-z])|(-?\d+))|mul ([a-z]) (?:([a-z])|(-?\d+))|mod ([a-z]) (?:([a-z])|(-?\d+))|rcv (\w)|jgz ([1-z]) (?:([a-z])|(-?\d+))").unwrap();
    let instructions: Vec<Instr> = re.captures_iter(&input).filter_map(|caps| {
        if let Some(x) = caps.get(1) {
            Some(Instr::SND(x.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(2), caps.get(3)) {
            Some(Instr::SET(x.as_str().chars().next().unwrap(), y.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(2), caps.get(4)) {
            Some(Instr::SETI(x.as_str().chars().next().unwrap(), y.as_str().parse().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(5), caps.get(6)) {
            Some(Instr::ADD(x.as_str().chars().next().unwrap(), y.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(5), caps.get(7)) {
            Some(Instr::ADDI(x.as_str().chars().next().unwrap(), y.as_str().parse().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(8), caps.get(9)) {
            Some(Instr::MUL(x.as_str().chars().next().unwrap(), y.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(8), caps.get(10)) {
            Some(Instr::MULI(x.as_str().chars().next().unwrap(), y.as_str().parse().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(11), caps.get(12)) {
            Some(Instr::MOD(x.as_str().chars().next().unwrap(), y.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(11), caps.get(13)) {
            Some(Instr::MODI(x.as_str().chars().next().unwrap(), y.as_str().parse().unwrap()))
        } else if let Some(x) = caps.get(14) {
            Some(Instr::RCV(x.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(15), caps.get(16)) {
            Some(Instr::JGZ(x.as_str().chars().next().unwrap(), y.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(15), caps.get(17)) {
            Some(Instr::JGZI(x.as_str().chars().next().unwrap(), y.as_str().parse().unwrap()))
        } else {
            None
        }
    }).collect();

    let mut pc = 0i64;
    let mut regs: HashMap<char, i64> = (97u8..123u8).map(|x| (x as char, 0)).collect();
    let mut freq = 0;
    while let Some(instr) = instructions.get(if pc < 0 { usize::max_value() } else { pc as usize }) {
        pc += 1;
        match *instr {
            Instr::SND(x) => {
                freq = regs[&x];
            }

            Instr::SET(x, y) => {
                let val = regs[&y];
                regs.insert(x, val);
            }

            Instr::SETI(x, y) => {
                regs.insert(x, y);
            }

            Instr::ADD(x, y) => {
                let val = regs[&x] + regs[&y];
                regs.insert(x, val);
            }

            Instr::ADDI(x, y) => {
                let val = regs[&x] + y;
                regs.insert(x, val);
            }

            Instr::MUL(x, y) => {
                let val = regs[&x] * regs[&y];
                regs.insert(x, val);
            }

            Instr::MULI(x, y) => {
                let val = regs[&x] * y;
                regs.insert(x, val);
            }

            Instr::MOD(x, y) => {
                let val = regs[&x] % regs[&y];
                regs.insert(x, val);
            }

            Instr::MODI(x, y) => {
                let val = regs[&x] % y;
                regs.insert(x, val);
            }

            Instr::RCV(x) => if regs[&x] != 0 {
                println!("{}", freq);
                regs.insert(x, freq);
                pc = -1;
            }

            Instr::JGZ(x, y) => if regs[&x] > 0 {
                pc += regs[&y] - 1;
            }

            Instr::JGZI(x, y) => if regs[&x] > 0 {
                pc += y - 1;
            }
        }
    }
}
