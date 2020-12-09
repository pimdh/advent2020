use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{VecDeque};
use std::iter::Iterator;

fn read_input(demo: bool) -> impl Iterator<Item=i64> {
    let input_name = if demo { "inputs/input9demo.txt" } else {"inputs/input9.txt" };
    let file = File::open(input_name).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap().parse().unwrap())
}

#[allow(clippy::needless_range_loop)]
fn contains_sum(nums: &VecDeque<i64>, x: i64) -> bool {
    for (i, a) in nums.iter().enumerate() {
        for j in i+1..nums.len() {
            let b = nums[j];
            if a + b == x {
                return true;
            }
        }
    }
    false
}

fn find_invalid_number(demo: bool) -> i64 {
    let mut numbers = VecDeque::new();
    let capacity = if demo { 5 } else { 25 };
    for x in read_input(demo) {
        if numbers.len() == capacity && !contains_sum(&numbers, x) {
            return x;
        }
        if numbers.len() == capacity {
            numbers.pop_front();
        }
        numbers.push_back(x);
    }
    panic!("Not found");
}

pub fn problem9a(demo: bool) {
    println!("{}", find_invalid_number(demo));
}

pub fn problem9b(demo: bool) {
    let sum = find_invalid_number(demo);
    let numbers: Vec<_> = read_input(demo).collect();
    let cumsum: Vec<_> = numbers.iter().scan(0, |state, &x| {
        *state += x;
        Some(*state)
    }).collect();

    for (i, a) in cumsum.iter().enumerate() {
        for j in (i+1)..cumsum.len() {
            let b = cumsum[j];
            if b - a == sum {
                let slice = &numbers[i+1..j+1];
                let max = slice.iter().max().unwrap();
                let min = slice.iter().min().unwrap();
                println!("{:?} {} {} {}", slice, max, min, max + min);
                return;
            }
        }
    }
}