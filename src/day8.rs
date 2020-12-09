use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BTreeSet;

use nom::{
    IResult,
    character::complete::{char, alpha1, digit1, one_of},
    combinator::{recognize},
    sequence::{separated_pair},
    sequence::{tuple},
};


#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Instruction {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

type ROM = Vec<Instruction>;

struct ProgramState {
    acc: i32,
    pc: i32,
}

impl ProgramState {
    fn init() -> ProgramState {
        ProgramState {
            acc: 0,
            pc: 0,
        }
    }
}

fn read_line(line: &str) -> Instruction {
    let parse_result: IResult<_, _> = separated_pair(
        alpha1,
        char(' '),
        recognize(tuple((one_of("+-"), digit1)))
    )(line);
    let (instruction, digit) = parse_result.unwrap().1;
    let digit: i32 = digit.parse().unwrap();
    match instruction {
        "nop" => Instruction::Nop(digit),
        "acc" => Instruction::Acc(digit),
        "jmp" => Instruction::Jmp(digit),
        _ => panic!("Found unparsable instruction {}", instruction)
    }
}

fn read_rom(path: &str) -> ROM {
    let file = File::open(path).unwrap();
    io::BufReader::new(file).lines().map(|l| read_line(&l.unwrap())).collect()
}

#[allow(clippy::ptr_arg)]
fn step(rom: &ROM, mut state: ProgramState) -> ProgramState {
    match rom[state.pc as usize] {
        Instruction::Nop(_) => { state.pc += 1 }
        Instruction::Acc(acc_val) => { state.acc += acc_val; state.pc += 1; }
        Instruction::Jmp(jmp_val) => { state.pc += jmp_val },
    }
    state
}


#[allow(clippy::ptr_arg)]
fn run_until_loop_or_terminatation(rom: &ROM) -> (bool, ProgramState) {
    let mut state = ProgramState::init();
    let mut visited = BTreeSet::new();

    loop {
        visited.insert(state.pc);
        state = step(&rom, state);

        if state.pc == rom.len() as i32 {
            return (true, state);
        }
        else if visited.contains(&state.pc) {
            return (false, state);
        }
    }
}

pub fn problem8a(demo: bool) {
    let path = if demo { "inputs/input8demo.txt" } else { "inputs/input8.txt" };
    let rom = read_rom(path);
    let (terminated, state) = run_until_loop_or_terminatation(&rom);
    println!("{} {}", terminated, state.acc);
}

#[allow(clippy::ptr_arg)]
fn modify_rom(rom: &ROM, i: usize) -> Option<ROM> {
    let mut mod_rom = rom.clone();
    match mod_rom[i] {
        Instruction::Nop(v) => { mod_rom[i] = Instruction::Jmp(v) }
        Instruction::Jmp(v) => { mod_rom[i] = Instruction::Nop(v) }
        Instruction::Acc(_) => { return None }
    }
    Some(mod_rom)
}

pub fn problem8b(demo: bool) {
    let path = if demo { "inputs/input8demo.txt" } else { "inputs/input8.txt" };
    let rom = read_rom(path);
    for i in 0..rom.len() {
        let mod_rom = match modify_rom(&rom, i) {
            Some(r) => r,
            None => continue,
        };
        let (terminated, state) = run_until_loop_or_terminatation(&mod_rom);
        if terminated {
            println!("{} {}", i, state.acc);
            return;
        }
    }
}