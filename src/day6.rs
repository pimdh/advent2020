use std::fs::File;
use std::io;
use std::iter::Iterator;
use std::collections::BTreeSet;
use io::Read;

fn read(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{char, alpha1},
    multi::{separated_list1},
};


pub fn problem6a(demo: bool) {
    let path = if demo { "inputs/input6demo.txt" } else { "inputs/input6.txt" };
    let s = read(path);
    let parse_result: IResult<_, _> = separated_list1(tag("\n\n"), separated_list1(char('\n'), alpha1))(s.as_str());
    let raw = parse_result.unwrap().1;
    let unions = raw.iter().map(|gr|
        gr.iter()
            .map(|p| p.chars().collect::<BTreeSet<_>>())
            .fold(BTreeSet::new(), |acc, s| acc.union(&s).cloned().collect())
    );
    println!("{:?}", unions.map(|s| s.len()).sum::<usize>());
}

pub fn problem6b(demo: bool) {
    let path = if demo { "inputs/input6demo.txt" } else { "inputs/input6.txt" };
    let s = read(path);
    let parse_result: IResult<_, _> = separated_list1(tag("\n\n"), separated_list1(char('\n'), alpha1))(s.as_str());
    let raw = parse_result.unwrap().1;
    let unions = raw.iter().map(|gr|
        gr.iter()
            .map(|p| p.chars().collect::<BTreeSet<_>>())
            .fold(None, |acc_opt, s| match acc_opt {
                None => Some(s),
                Some(acc) => Some(acc.intersection(&s).cloned().collect()),
            }).unwrap()
    );
    println!("{:?}", unions.map(|s| s.len()).sum::<usize>());
}