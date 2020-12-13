use adventofcode_2020::read_lines;

use std::collections::HashSet;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Acc,
    Jmp,
    Nop,
}

impl Instruction {
    fn swap(&mut self) {
        match *self {
            Acc => unimplemented!(),
            Jmp => *self = Nop,
            Nop => *self = Jmp,
        }
    }
}

use self::Instruction::*;

struct BootCode(Vec<(Instruction, i64)>);

impl BootCode {
    fn run(&self) -> Result<i64, i64> {
        let mut prev_locs = HashSet::new();

        let mut pc = 0;
        let mut acc = 0;

        loop {
            if !prev_locs.insert(pc) {
                break Err(acc);
            }
            match self.0.get(pc) {
                Some((Acc, off)) => acc += off,
                Some((Nop, _)) => (),
                Some((Jmp, off)) => {
                    pc = (pc as i64 + off) as usize;
                    continue;
                }
                None => break Ok(acc),
            }
            pc += 1;
        }
    }
}

fn main() {
    let mut boot_code = BootCode(read_lines("day8.txt").map(|line| {
        let ins = match &line[..3] {
            "acc" => Acc,
            "jmp" => Jmp,
            "nop" => Nop,
            _ => unimplemented!(),
        };
        (ins, line[4..].parse().unwrap())
    }).collect());

    println!("{}", boot_code.run().unwrap_err());

    for i in 0..boot_code.0.len() {
        match boot_code.0[i].0 {
            Acc => continue,
            _ => boot_code.0[i].0.swap(),
        }

        if let Ok(n) = boot_code.run() {
            println!("{}", n);
            break
        }

        boot_code.0[i].0.swap();
    }
}