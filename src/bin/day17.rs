use core::panic;

use aoc24::*;
use regex::Regex;

fn main() {
    let binding = read_input(17);
    let input = binding.as_str();
    println!("Part 1: {}", part1(&input));
    println!("Part 1 fast: {}", part1fast(&input));
    println!("Part 2: {}", part2(&input));
}

fn part1(input: &str) -> String {
    let mut cpu = CPU::from_str(input);
    cpu.execute(false);
    cpu.output_str()
}

fn part1fast(input: &str) -> String {
    let cpu = CPU::from_str(input);
    cpu.simfast()
}

// Unfortunately, my solution will only work for my individual input, as a general analysis program
// is NP.
// This program just seeks to find an initial value for A which results in the following output:
// 2417750317415530
fn part2(input: &str) -> String {
    let cpu = CPU::from_str(input);
    let imem = cpu.imem;
    let a = backtrack(0, &imem, 15);
    if a.is_some() {
        return a.unwrap().to_string();
    }
    return "failed".to_string();
}

// Start from back of imem
// on each iteration: check all 8 possibilities for digit corresponding to current insn
// start with lowest possible value, since the prompt asks for the lowest value
// for each possible digit: shift in to program, decrement index, make recursive call
// if no paths possible, return None. otherwise, return the correct program.
fn backtrack(program: u64, imem: &Vec<usize>, index: usize) -> Option<u64> {
    for i in 0..8 {
        let pushed = program << 3 | i;
        if result(pushed) == imem[index] {
            if index == 0 {
                return Some(pushed);
            }
            let further = backtrack(pushed, imem, index - 1);
            if further.is_some() {
                return further;
            }
        }
    }
    None
}

fn result(a: u64) -> usize {
    let last = a & 7;
    let result = (a >> (last ^ 7)) ^ last;
    (result & 7) as usize
}

struct CPU {
    regs: [usize; 3],
    imem: Vec<usize>,
    pc: usize,
    output: Vec<usize>,
    insns_executed: usize,
}

impl CPU {
    fn new(regs: [usize; 3], imem: Vec<usize>) -> Self {
        CPU {
            regs,
            imem,
            pc: 0,
            output: Vec::new(),
            insns_executed: 0,
        }
    }

    fn from_str(input: &str) -> Self {
        let sections = split_sections(input);
        let re =
            Regex::new(r"Register A: (?P<a>\d+)\nRegister B: (?P<b>\d+)\nRegister C: (?P<c>\d+)")
                .unwrap();
        let caps = re.captures(sections[0]).unwrap();
        let a = caps.name("a").unwrap().as_str().parse::<usize>().unwrap();
        let b = caps.name("b").unwrap().as_str().parse::<usize>().unwrap();
        let c = caps.name("c").unwrap().as_str().parse::<usize>().unwrap();
        let regs = [a, b, c];

        let program_re = Regex::new(r"Program: (?P<insns>.*)\n?").unwrap();
        let insn_input = program_re
            .captures(sections[1])
            .unwrap()
            .name("insns")
            .unwrap()
            .as_str();
        let insns = parse_row_major::<usize>(insn_input, ",")[0].clone();
        Self::new(regs, insns)
    }

    fn simfast(&self) -> String {
        let mut out = String::new();
        let mut a: u64 = self.regs[0].clone() as u64;
        while a > 0 {
            let last = a & 7;
            let result = ((a >> (last ^ 7)) ^ last) & 7;
            out.push_str(result.to_string().as_str());
            a >>= 3;
        }
        out
    }

    fn combo(&self, value: usize) -> usize {
        match value {
            0..=3 => value,
            4 => self.regs[0],
            5 => self.regs[1],
            6 => self.regs[2],
            _ => panic!(),
        }
    }

    fn print_regfile(&self) {
        println!(
            "\tA: {:o} B: {:o} C: {:o}",
            self.regs[0], self.regs[1], self.regs[2]
        );
    }

    fn print_step(&self) {
        const INSN_NAMES: [&str; 8] = ["adv", "bxl", "bst", "jnz", "bxc", "out", "bdv", "cdv"];
        if self.pc >= self.imem.len() - 1 {
            return;
        }
        println!(
            "{}: {} {}",
            self.pc,
            INSN_NAMES[self.imem[self.pc]],
            self.imem[self.pc + 1]
        );
    }

    /**
     * return false if halted
     */
    fn step(&mut self) -> bool {
        if self.pc >= self.imem.len() - 1 {
            return false;
        }
        let opcode = self.imem[self.pc];
        let operand = self.imem[self.pc + 1];
        let mut advance_pc = true;

        match opcode {
            0 => self.div(operand, 0),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => advance_pc = self.jnz(operand),
            4 => self.bxc(),
            5 => self.out(operand),
            6 => self.div(operand, 1),
            7 => self.div(operand, 2),
            _ => panic!(),
        }

        if advance_pc {
            self.pc += 2;
        }

        true
    }

    fn execute(&mut self, verbose: bool) -> String {
        if verbose {
            self.print_regfile();
            self.print_step();
        }
        while self.step() {
            if verbose {
                self.print_regfile();
                self.print_step();
            }
        }
        self.output_str()
    }

    // [dest] = A / (2 ** combo(op))
    fn div(&mut self, operand: usize, dest: usize) {
        self.regs[dest] = self.regs[0] >> self.combo(operand);
    }

    // B = combo(op) % 8
    fn bst(&mut self, operand: usize) {
        self.regs[1] = self.combo(operand) & 7;
    }

    // if A != 0 {PC = op}
    // returns true if no jump
    fn jnz(&mut self, operand: usize) -> bool {
        if self.regs[0] == 0 {
            return true;
        }
        self.pc = operand;
        return false;
    }

    fn bxl(&mut self, operand: usize) {
        self.regs[1] = self.regs[1] ^ operand;
    }

    fn bxc(&mut self) {
        self.regs[1] = self.regs[1] ^ self.regs[2];
    }

    // pushes combo result modulo 8 to output
    fn out(&mut self, operand: usize) {
        self.output.push(self.combo(operand) & 7);
    }

    fn output_str(&self) -> String {
        let mut ret = String::new();
        if self.output.len() == 0 {
            return ret;
        }
        for i in 0..(self.output.len() - 1) {
            ret.push_str(self.output[i].to_string().as_str());
            ret.push(',');
        }
        ret.push_str(self.output[self.output.len() - 1].to_string().as_str());
        ret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_INPUT: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), "4,6,3,5,6,3,5,2,1,0");
    }
}
