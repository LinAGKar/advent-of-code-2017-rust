extern crate regex;


use regex::Regex;
use std::collections::HashMap;
use std::io;
use std::io::Read;


#[derive(Debug, Clone, Copy)]
enum Value {
    Num(i64),
    Reg(char),
}


#[derive(Debug, Clone, Copy)]
enum Instr {
    Set(char, Value),
    Sub(char, Value),
    Mul(char, Value),
    Jnz(Value, Value),
}


fn get_val(val: &Value, regs: &HashMap<char, i64>) -> Option<i64> {
    match *val {
        Value::Num(v) => Some(v),
        Value::Reg(v) => regs.get(&v).map(|&x| x),
    }
}


fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let re = Regex::new(r"set ([a-h]) ([a-h]|-?\d+)|sub ([a-h]) ([a-h]|-?\d+)|mul ([a-h]) ([a-h]|-?\d+)|jnz ([a-h]|-?\d+) ([a-h]|-?\d+)").unwrap();
    let instructions: Vec<Instr> = re.captures_iter(&input).filter_map(|caps| {
        if let (Some(x), Some(y)) = (caps.get(1), caps.get(2)) {
            Some(Instr::Set(x.as_str().chars().next().unwrap(), match y.as_str().parse() {
                Ok(val) => Value::Num(val),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
        } else if let (Some(x), Some(y)) = (caps.get(3), caps.get(4)) {
            Some(Instr::Sub(x.as_str().chars().next().unwrap(), match y.as_str().parse() {
                Ok(val) => Value::Num(val),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
        } else if let (Some(x), Some(y)) = (caps.get(5), caps.get(6)) {
            Some(Instr::Mul(x.as_str().chars().next().unwrap(), match y.as_str().parse() {
                Ok(val) => Value::Num(val),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
        } else if let (Some(x), Some(y)) = (caps.get(7), caps.get(8)) {
            Some(Instr::Jnz(match x.as_str().parse() {
                Ok(val) => Value::Num(val),
                Err(_) => Value::Reg(x.as_str().chars().next().unwrap()),
            }, match y.as_str().parse() {
                Ok(val) => Value::Num(val),
                Err(_) => Value::Reg(y.as_str().chars().next().unwrap()),
            }))
        } else {
            None
        }
    }).collect();
    let mut pc = 0i64;
    let mut regs: HashMap<char, i64> = (97u8..105u8).map(|x| (x as char, 0)).collect();
    let mut mul_counter = 0;
    while let Some(instr) = instructions.get(pc as usize) {
        pc += 1;
        match *instr {
            Instr::Set(x, y) => {
                let val = get_val(&y, &regs).unwrap();
                regs.insert(x, val);
            }
            Instr::Sub(x, y) => {
                let val = get_val(&y, &regs).unwrap();
                match regs.get_mut(&x) {
                    Some(x) => { *x -= val; }
                    None => { panic!(format!("Could not find register {}", x)); }
                }
            }
            Instr::Mul(x, y) => {
                mul_counter += 1;
                let val = get_val(&y, &regs).unwrap();
                match regs.get_mut(&x) {
                    Some(x) => { *x *= val; }
                    None => { panic!(format!("Could not find register {}", x)); }
                }
            }
            Instr::Jnz(x, y) => {
                if get_val(&x, &regs).unwrap() != 0 {
                    pc += get_val(&y, &regs).unwrap() - 1;
                }
            }
        }
    }
    println!("{}", mul_counter);
}
