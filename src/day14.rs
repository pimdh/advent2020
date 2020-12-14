use std::fs::{read_to_string};
use std::iter::Iterator;
use std::collections::BTreeMap;

use nom::{
    IResult,
    bytes::complete::{tag},
    character::complete::{digit1, alphanumeric1},
    combinator::{map, map_res},
    sequence::{tuple},
    multi::{separated_list1},
    branch::{alt},
};

use Instruction::{Write, SetMask};

#[derive(Debug, PartialEq, Eq, Clone)]
enum Instruction {
    Write(u64, u64),
    SetMask(String),
}

fn parse(s: &str) -> Vec<Instruction> {
    let mut parser = separated_list1(tag("\n"), alt((
        map(tuple((
            tag("mem["),
            map_res(digit1, str::parse),
            tag("] = "),
            map_res(digit1, str::parse),
        )), |(_, location, _, value)| Write(location, value)),
        map(tuple((
            tag("mask = "),
            alphanumeric1,
        )), |(_, val) : (_, &str)| SetMask(val.to_owned()))
    )));
    let res: IResult<_,_> = parser(s);
    res.unwrap().1
}

#[derive(Debug)]
struct State {
    mask: String,
    memory: BTreeMap<u64,u64>
}

fn build_mask_a(one_char: char, s: &str) -> u64 {
    let bits: String = s.chars().map(|c| if c == one_char { '1' } else { '0' }).collect();
    u64::from_str_radix(bits.as_str(), 2).unwrap()
}

fn apply_mask_a(mask: &str, val: u64) -> u64 {
    let one_mask = build_mask_a('1', mask);
    let zero_mask = build_mask_a('0', mask);
    !(!(val | one_mask) | zero_mask)
}

fn step_a(mut state: State, instr: Instruction) -> State {
    match instr {
        SetMask(mask) => { state.mask = mask },
        Write(addr, val) => { state.memory.insert(addr, apply_mask_a(&state.mask, val)); },
    }
    state
}

pub fn problem14a(demo: bool) {
    let path = if demo { "inputs/input14demo.txt" } else { "inputs/input14.txt" };
    let s = read_to_string(path).unwrap();
    let data = parse(s.as_str());
    let state = State { mask: String::new(), memory: BTreeMap::new() };
    let state = data.into_iter().fold(state, step_a);
    println!("{:?}", state.memory.values().sum::<u64>());
}

pub fn mask_address(addr: u64, mask: &str) -> Vec<u64> {
    if mask.is_empty() {
        vec![addr]
    } else {
        let first = mask.chars().next().unwrap();
        let this_mask = 1 << (mask.len()-1);
        match first {
            '0' => mask_address(addr, &mask[1..]),
            '1' => mask_address(addr | this_mask, &mask[1..]),
            _ => {
                let mut res = Vec::new();
                for next_addr in mask_address(addr, &mask[1..]) {
                    res.push(next_addr | this_mask);
                    res.push(!(!next_addr | this_mask));
                }
                res
            }
        }
    }
}

fn step_b(mut state: State, instr: Instruction) -> State {
    match instr {
        SetMask(mask) => { state.mask = mask },
        Write(addr, val) => {
            for addr_masked in mask_address(addr, &state.mask) {
                state.memory.insert(addr_masked, val);
            }
        }
    }
    state
}

pub fn problem14b(demo: bool) {
    let path = if demo { "inputs/input14demob.txt" } else { "inputs/input14.txt" };
    let s = read_to_string(path).unwrap();
    let data = parse(s.as_str());
    let state = State { mask: String::new(), memory: BTreeMap::new() };
    let state = data.into_iter().fold(state, step_b);
    println!("{:?}", state.memory.values().sum::<u64>());
}