use std::fs::File;
use std::io::{self};
use std::iter::Iterator;
use io::Read;

use nom::{
    IResult,
    bytes::complete::{tag},
    character::complete::{digit1, alphanumeric1},
    combinator::{map, map_res},
    sequence::{separated_pair},
    multi::{separated_list1}
};

fn parse(s: &str) -> IResult<&str, (i64, Vec<Option<i64>>)> {
    separated_pair(
        map_res(digit1, str::parse),
        tag("\n"),
        separated_list1(tag(","), map(alphanumeric1, |x: &str| {
            x.parse().ok()
        })),
    )(s)
}

fn read(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

pub fn problem13a(demo: bool) {
    let path = if demo { "inputs/input13demo.txt" } else { "inputs/input13.txt" };
    let s = read(path);
    let (arrival_time, busses) = parse(s.as_str()).unwrap().1;
    let (bus, time_until) = busses.into_iter()
        .filter_map(|x| x)
        .map(|b| (b, b-(arrival_time % b)))
        .min_by_key(|&(_b, t)| t)
        .unwrap();
    println!("{:?}", bus * time_until);
}

// Strategy is that if we have once seen a sequence of M correct busses, we only need to check every group period of the first M buses.
pub fn problem13b(demo: bool) {
    let path = if demo { "inputs/input13demo.txt" } else { "inputs/input13.txt" };
    let s = read(path);
    let (_arrival_time, busses) = parse(s.as_str()).unwrap().1;
    let busses: Vec<_> = busses.into_iter().map(|x| x.unwrap_or(1)).collect();

    let mut dt = 1;
    let mut accepted = 0;
    let mut t = 1;
    loop {
        for (i, b) in busses.iter().enumerate().skip(accepted) {
            if (t+i as i64) % b == 0 {
                accepted += 1;
                dt *= b;
            } else {
                break
            }
        }
        if accepted == busses.len() {
            println!("FOUND {}", t);
            return
        }
        t += dt;
    }
}