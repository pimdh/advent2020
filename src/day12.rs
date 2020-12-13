use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::ops::{Add, Neg, AddAssign};

use nom::{
    IResult,
    character::complete::{alpha1, digit1},
    combinator::{map_res},
    sequence::{tuple},
};

fn read_input(path: &str) -> impl Iterator<Item=(char, i64)> {
    let file = File::open(path).unwrap();
    io::BufReader::new(file).lines().map(|l| {
        let l = l.unwrap();
        let res: IResult<_, _> = tuple((
            map_res(alpha1, str::parse),
            map_res(digit1, str::parse),
        ))(l.as_str());
        res.unwrap().1
    })
}

fn disccos(theta: i64) -> i64 {
    match theta.rem_euclid(360) {
        0 => 1,
        90 => 0,
        180 => -1,
        270 => 0,
        _ => panic!("Invalid disccos value {}", theta)
    }
}

fn discsin(theta: i64) -> i64 {
    disccos(90-theta)
}


#[derive(PartialEq, Eq, Debug, Clone, Copy, Default)]
struct Point {
    x: i64,
    y: i64,
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other
    }
}

impl Neg for Point {
    type Output = Self;

    fn neg(self) -> Self {
        Self {x: -self.x, y: -self.y}
    }
}

impl Point {
    fn mul(self, v: i64) -> Self {
        Self {x: v * self.x, y: v * self.y }
    }

    fn rot(self, theta: i64) -> Self {
        Self {
            x: disccos(theta) * self.x + -discsin(theta) * self.y,
            y: disccos(theta) * self.y + discsin(theta) * self.x,
        }
    }

    fn norm(self) -> i64 {
        self.x.abs() + self.y.abs()
    }
}

fn point(x: i64, y: i64) -> Point {
    Point {x, y}
}

const NORTH: Point = Point {x: 0, y: 1};
const EAST: Point = Point {x: 1, y: 0};
const SOUTH: Point = Point {x: 0, y: -1};
const WEST: Point = Point {x: -1, y: 0};


#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct State {
    pos: Point,
    way: Point,
}

fn step_a(mut s: State, (instruction, amount): (char, i64)) -> State {
    match instruction {
        'N' => { s.pos += NORTH.mul(amount) },
        'E' => { s.pos += EAST.mul(amount)},
        'S' => { s.pos += SOUTH.mul(amount) },
        'W' => { s.pos += WEST.mul(amount) },
        'L' => { s.way = s.way.rot(amount) },
        'R' => { s.way = s.way.rot(-amount) },
        'F' => { s.pos += s.way.mul(amount) }
        'B' => { s.pos += -s.way.mul(amount) }
        _ => panic!("Unrecognized instruction {}{}", instruction, amount)
    }
    s
}

fn step_b(mut s: State, (instruction, amount): (char, i64)) -> State {
    match instruction {
        'N' => { s.way += NORTH.mul(amount) },
        'E' => { s.way += EAST.mul(amount)},
        'S' => { s.way += SOUTH.mul(amount) },
        'W' => { s.way += WEST.mul(amount) },
        'L' => { s.way = s.way.rot(amount) },
        'R' => { s.way = s.way.rot(-amount) },
        'F' => { s.pos += s.way.mul(amount) }
        'B' => { s.pos += -s.way.mul(amount) }
        _ => panic!("Unrecognized instruction {}{}", instruction, amount)
    }
    s
}

pub fn problem12a(demo: bool) {
    let path = if demo { "inputs/input12demo.txt" } else { "inputs/input12.txt" };
    let s = State { pos: point(0, 0), way: point(1, 0)};
    let s = read_input(path).fold(s, step_a);
    println!("{:?}", s.pos.norm());
}

pub fn problem12b(demo: bool) {
    let path = if demo { "inputs/input12demo.txt" } else { "inputs/input12.txt" };
    let s = State { pos: point(0, 0), way: point(10, 1)};
    let s = read_input(path).fold(s, step_b);
    println!("{:?}", s.pos.norm());
}