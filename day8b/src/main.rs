extern crate regex;

use regex::Regex;
use std::cmp;
use std::collections::HashMap;
use std::io;
use std::io::Read;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut regs = HashMap::new();
    let re = Regex::new(r"^(\w+) (inc|dec) (-?\d+) if (\w+) (>|<|==|!=|>=|<=) (-?\d+)$").unwrap();
    let mut max = i64::min_value();
    for line in input.lines() {
        if let Some(caps) = re.captures(line) {
            let comp_val: i64 = caps[6].parse().expect("Failed to parse comparison value");
            let comp_reg = match regs.get(&caps[4]) { Some(x) => *x, None => 0 };
            if match &caps[5] {
                ">" => comp_reg > comp_val,
                "<" => comp_reg < comp_val,
                "==" => comp_reg == comp_val,
                "!=" => comp_reg != comp_val,
                ">=" => comp_reg >= comp_val,
                "<=" => comp_reg <= comp_val,
                _ => panic!(),
            } {
                let reg = regs.entry(caps[1].to_string()).or_insert(0);
                let val: i64 = caps[3].parse().expect("Failed to parse operation value");
                match &caps[2] {
                    "dec" => { *reg -= val; }
                    "inc" => { *reg += val; }
                    _ => { panic!(); },
                }
                max = cmp::max(max, *reg);
            }
        }
    }

    println!("{}", max);
}
