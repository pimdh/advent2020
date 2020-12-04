use std::fs::File;
use std::io;
use io::Read;
use std::collections::BTreeSet;

use nom::{
    IResult,
    bytes::complete::{tag, take_while1},
    character::complete::{char, space1, line_ending, alphanumeric1, digit1},
    sequence::{terminated, separated_pair, preceded},
    combinator::{eof, verify},
    multi::{separated_list1},
    branch::alt,
};


fn parser(input: &str) -> Vec<Vec<(&str, &str)>> {
    let p: IResult<_, _> = terminated(
        separated_list1(
            tag("\n\n"),
            separated_list1(
                alt((space1, line_ending)),
                separated_pair(
                    alphanumeric1,
                    char(':'),
                    take_while1(|c: char| c.is_alphanumeric() || c == '#'),
                )
            )
        ),
        eof
    )(input);
    p.unwrap().1
}

fn read(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

fn int_str_between(val: &str, min: i32, max: i32) -> bool {
    let v: i32 = val.parse().unwrap();
    v >= min && v <= max
}

fn matches<I, F, O>(parser: F, val: I) -> bool
where
    F: nom::Parser<I, O, nom::error::Error<I>>,
    I: Copy + nom::InputLength,
{
    terminated(parser, eof)(val).is_ok()
}

fn len_equals(len: usize) -> impl Fn(&str) -> bool {
    move |s: &str| s.len() == len
}

fn is_color_str(val: &str) -> bool {
    matches(
        preceded(
            char('#'),
            verify(take_while1(|c: char| c.is_ascii_hexdigit()), len_equals(6))),
        val)
}


fn validate(key: &str, val: &str) -> bool {
    match key {
        "byr" => int_str_between(val, 1920, 2002),
        "iyr" => int_str_between(val, 2010, 2020),
        "eyr" => int_str_between(val, 2020, 2030),
        "hgt" => {
            let (num, unit) = val.split_at(val.len()-2);
            match unit {
                "cm" => int_str_between(num, 150, 193),
                "in" => int_str_between(num, 59, 76),
                _ => false
            }
        },
        "hcl" => is_color_str(val),
        "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].iter().cloned().any(|c| c == val),
        "pid" => matches(verify(digit1, len_equals(9)), val),
        "cid" => true,
        _ => false
    }
}

pub fn problem4a(demo: bool) {
    let path = if demo { "inputs/input4demo.txt" } else { "inputs/input4a.txt" };
    let s = read(path);
    let parsed = parser(&s);

    let required: BTreeSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();
    let res = parsed.iter().filter(|record| { 
        let record_key_set: BTreeSet<_> = record.iter().map(|(k, _)| k).cloned().collect();
        record_key_set.is_superset(&required)
    }).count();

    println!("{:?}", res);
}

pub fn problem4b(demo: bool) {
    let path = if demo { "inputs/input4demo.txt" } else { "inputs/input4a.txt" };
    let s = read(path);
    let parsed = parser(&s);

    let required: BTreeSet<_> = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"].iter().cloned().collect();
    let res = parsed.iter().filter(|record| { 
        let record_key_set: BTreeSet<_> = record.iter().map(|(k, _)| k).cloned().collect();
        let valid = record.iter().all(|(k, v)| validate(k, v));
        record_key_set.is_superset(&required) && valid
    }).count();

    println!("{:?}", res);
}