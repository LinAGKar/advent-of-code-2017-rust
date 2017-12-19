extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;


#[derive(Debug)]
enum Value {
    Reg(char),
    Num(i64),
}


#[derive(Debug)]
enum Instr {
    SND(char),
    SET(char, Value),
    ADD(char, Value),
    MUL(char, Value),
    MOD(char, Value),
    RCV(char),
    JGZ(Value, Value),
}


fn get_val(regs: &HashMap<char, i64>, x: &Value) -> i64 {
    match *x {
        Value::Reg(r) => regs[&r],
        Value::Num(v) => v,
    }
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to get input");
    let re = Regex::new(r"snd ([a-z])|set ([a-z]) ([a-z]|-?\d+)|add ([a-z]) ([a-z]|-?\d+)|mul ([a-z]) ([a-z]|-?\d+)|mod ([a-z]) ([a-z]|-?\d+)|rcv ([a-w])|jgz ([a-z]|-?\d+) ([a-z]|-?\d+)").unwrap();
    let instructions: Vec<Instr> = re.captures_iter(&input).filter_map(|caps| {
        if let Some(x) = caps.get(1) {
            Some(Instr::SND(x.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(2), caps.get(3)) {
            Some(Instr::SET(x.as_str().chars().next().unwrap(), match y.as_str().parse() {
                Ok(v) => Value::Num(v),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
        } else if let (Some(x), Some(y)) = (caps.get(4), caps.get(5)) {
            Some(Instr::ADD(x.as_str().chars().next().unwrap(), match y.as_str().parse() {
                Ok(v) => Value::Num(v),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
        } else if let (Some(x), Some(y)) = (caps.get(6), caps.get(7)) {
            Some(Instr::MUL(x.as_str().chars().next().unwrap(), match y.as_str().parse() {
                Ok(v) => Value::Num(v),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
        } else if let (Some(x), Some(y)) = (caps.get(8), caps.get(9)) {
            Some(Instr::MOD(x.as_str().chars().next().unwrap(), match y.as_str().parse() {
                Ok(v) => Value::Num(v),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
        } else if let Some(x) = caps.get(10) {
            Some(Instr::RCV(x.as_str().chars().next().unwrap()))
        } else if let (Some(x), Some(y)) = (caps.get(11), caps.get(12)) {
            Some(Instr::JGZ(match x.as_str().parse() {
                Ok(v) => Value::Num(v),
                Err(_) => Value::Reg(x.as_str().chars().next().unwrap()),
            }, match y.as_str().parse() {
                Ok(v) => Value::Num(v),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
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

            Instr::SET(x, ref y) => {
                let val = get_val(&regs, &y);
                regs.insert(x, val);
            }

            Instr::ADD(x, ref y) => {
                let val = regs[&x] + get_val(&regs, &y);
                regs.insert(x, val);
            }

            Instr::MUL(x, ref y) => {
                let val = regs[&x] * get_val(&regs, &y);
                regs.insert(x, val);
            }

            Instr::MOD(x, ref y) => {
                let val = regs[&x] % get_val(&regs, &y);
                regs.insert(x, val);
            }

            Instr::RCV(x) => if regs[&x] != 0 {
                println!("{}", freq);
                regs.insert(x, freq);
                pc = -1;
            }

            Instr::JGZ(ref x, ref y) => if get_val(&regs, &x) > 0 {
                pc += get_val(&regs, &y) - 1;
            }
        }
    }
}
