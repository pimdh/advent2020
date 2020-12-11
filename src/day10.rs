use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;

fn read_input(demo: bool) -> impl Iterator<Item=i64> {
    let path = if demo { "inputs/input10demo.txt" } else { "inputs/input10.txt" };
    let file = File::open(path).unwrap();
    io::BufReader::new(file).lines().map(|l| l.unwrap().parse().unwrap())
}

pub fn problem10a(demo: bool) {
    let mut input: Vec<_> = read_input(demo).collect();
    input.sort_unstable();
    let delta: Vec<_> = input.iter().scan(0, |state, &x| {
        let y = x - *state;
        *state = x;
        Some(y)
    }).collect();
    let num_1 = delta.iter().filter(|&&x| x == 1).count();
    let num_3 = delta.iter().filter(|&&x| x == 3).count() + 1;
    println!("{} {} {:?}", num_1, num_3, num_1 * num_3);
}

pub fn problem10b(demo: bool) {
    let mut input: Vec<_> = read_input(demo).collect();
    input.push(0);
    input.sort_unstable();
    let mut counts = vec![1u64];
    for (i, v) in input.iter().enumerate().skip(1) {
        let mut c = 0;
        for d in 1..4 {
            if i >= d && v-input[i-d] <= 3 {
                c += counts[i-d];
            }
        }
        counts.push(c);
    }
    println!("{:?}", &counts[counts.len()-1]);
}