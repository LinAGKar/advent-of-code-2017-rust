extern crate regex;

use regex::Regex;
use std::collections::HashMap;
use std::collections::VecDeque;
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

struct Program<'a> {
    pc: i64,
    regs: HashMap<char, i64>,
    instructions: &'a Vec<Instr>,
}

impl<'a> Program<'a> {
    fn new(instructions: &'a Vec<Instr>, number: i64) -> Program<'a> {
        Program {
            pc: 0,
            regs: (97u8..123u8).map(|x| (x as char, if x == 112 { number } else { 0 })).collect(),
            instructions: instructions,
        }
    }

    fn get_val(&self, x: &Value) -> i64 {
        match *x {
            Value::Reg(r) => self.regs[&r],
            Value::Num(v) => v,
        }
    }

    fn execute(&mut self, output: &mut VecDeque<i64>, input: &mut VecDeque<i64>) -> bool {
        while let Some(instr) = self.instructions.get(if self.pc < 0 { usize::max_value() } else { self.pc as usize }) {
            self.pc += 1;
            match *instr {
                Instr::SND(x) => {
                    output.push_back(self.regs[&x]);
                }

                Instr::SET(x, ref y) => {
                    let val = self.get_val(&y);
                    self.regs.insert(x, val);
                }

                Instr::ADD(x, ref y) => {
                    let val = self.regs[&x] + self.get_val(&y);
                    self.regs.insert(x, val);
                }

                Instr::MUL(x, ref y) => {
                    let val = self.regs[&x] * self.get_val(&y);
                    self.regs.insert(x, val);
                }

                Instr::MOD(x, ref y) => {
                    let val = self.regs[&x] % self.get_val(&y);
                    self.regs.insert(x, val);
                }

                Instr::RCV(x) => if let Some(val) = input.pop_front() {
                    self.regs.insert(x, val);
                } else {
                    self.pc -= 1;
                    return false;
                }

                Instr::JGZ(ref x, ref y) => if self.get_val(&x) > 0 {
                    self.pc += self.get_val(&y) - 1;
                }
            }
        }
        true
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

    let mut program0 = Program::new(&instructions, 0);
    let mut program1 = Program::new(&instructions, 1);
    let mut queue1 = VecDeque::new();
    let mut queue2 = VecDeque::new();
    let mut counter = 0;

    loop {
        let done = program0.execute(&mut queue2, &mut queue1);
        let len_before = queue1.len();
        let done = done || program1.execute(&mut queue1, &mut queue2);
        counter += queue1.len() - len_before;
        if done || queue1.is_empty() {
            break;
        }
    }
    println!("{}", counter);
}
