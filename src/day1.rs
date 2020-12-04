use std::fs::File;
use std::io::{self, BufRead};
use std::collections::BTreeSet;
use std::iter::Iterator;

fn read_input() -> impl Iterator<Item=i32> {
    let input_name = "inputs/input1a.txt";
    let file = File::open(input_name).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap().parse().unwrap())
}


pub fn problem1a() {
    let set: BTreeSet<_> = read_input().collect();
    for x in &set {
        if set.contains(&(2020-x)) {
            println!("{}", x *  (2020-x));
            return;
        }
    }
}

pub fn problem1b() {
    let set: BTreeSet<_> = read_input().collect();
    for x in &set {
        for y in &set {
            let z = 2020 - x - y;
            if set.contains(&z) {
                return println!("{}", x * y * z);
            }
        }
    }
}