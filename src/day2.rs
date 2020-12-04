use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use regex::Regex;


#[derive(Debug)]
struct Line {
    min_count: usize,
    max_count: usize,
    character: char,
    password: String,
}

lazy_static! {
    static ref REGEX: Regex = Regex::new(r"^(\d+)-(\d+) (\w): (\w+)$").unwrap();
}

fn parse_line(line: String) -> Line
{
    let caps = REGEX.captures(&line).unwrap();
    // println!("{}", &line);
    let l = Line {
        min_count: caps.get(1).unwrap().as_str().parse().unwrap(),
        max_count: caps.get(2).unwrap().as_str().parse().unwrap(),
        character: caps.get(3).unwrap().as_str().chars().next().unwrap(),
        password: String::from(caps.get(4).unwrap().as_str()),
    };
    // println!("{:?}", &l);
    l
}

fn read_input(demo: bool) -> impl Iterator<Item=Line> {
    let input_name = if demo {
        "inputs/input2demo.txt"
    } else {
        "inputs/input2a.txt"
    };
    let file = File::open(input_name).unwrap();
    io::BufReader::new(file).lines().map(|l| parse_line(l.unwrap()))
}

fn check_line_a(line: &Line) -> bool {
    let count = line.password.chars().filter(|c| c == &line.character).count();
    count >= line.min_count && count <= line.max_count
}

fn check_line_b(line: &Line) -> bool {
    let count = line.password.chars().enumerate().filter(|(i, c)| {
        (i+1 == line.max_count || i+1 == line.min_count) && c == &line.character
    }).count();
    count == 1
}


pub fn problem2a(demo: bool) {
    let correct = read_input(demo).filter(check_line_a).count();
    println!("{}", correct);
}

pub fn problem2b(demo: bool) {
    let correct = read_input(demo).filter(check_line_b).count();
    println!("{}", correct);
}