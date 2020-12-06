use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::collections::BTreeSet;

fn parse_part(part: &str, upper_char: char)  -> u32 {
    part.chars().enumerate()
        .filter(|(_, c)| *c == upper_char)
        .map(|(i, _)| 2u32.pow((part.len()-i-1) as u32)).sum()
}

fn find_seat(identifier: &str) -> (u32, u32) {
    let (row_identifier, col_identifier) = identifier.split_at(7);
    (parse_part(row_identifier, 'B'), parse_part(col_identifier, 'R'))
}

fn get_seats() -> impl Iterator<Item=(u32, u32)> {
    let input_name = "inputs/input5.txt";
    let file = File::open(input_name).unwrap();
    let lines = io::BufReader::new(file).lines();
    lines.map(|l| find_seat(l.unwrap().as_str()))
}

pub fn problem5a(_demo: bool) {
    let res = get_seats().map(|(r, c)| r * 8 + c).max().unwrap();
    println!("{:?}", res);
}


pub fn problem5b(_demo: bool) {
    let seats: BTreeSet<_> = get_seats().collect();
    for (r, c) in &seats {
        if seats.contains(&(*r, c+2)) && !seats.contains(&(*r, c+1)) {
            println!("{}", r * 8 + c+1);
        }
    }
}