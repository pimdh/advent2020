use std::fs::File;
use std::io;
use std::iter::Iterator;
use io::Read;
use std::collections::{BTreeMap, BTreeSet};

fn read(path: &str) -> String {
    let mut file = File::open(path).unwrap();
    let mut s = String::new();
    file.read_to_string(&mut s).unwrap();
    s
}

use nom::{
    IResult,
    bytes::complete::tag,
    character::complete::{char, alpha1, digit1},
    combinator::{map, recognize},
    multi::{separated_list1},
    sequence::{separated_pair},
    branch::{alt},
    sequence::{tuple, terminated},
};


fn bag_parser(input: &str) -> IResult<&str, &str> {
    terminated(
        recognize(separated_pair(alpha1, char(' '), alpha1)),
        alt((tag(" bags"), tag(" bag"))))
    (input)
}

type ParseResult<'a> = Vec<(&'a str, Vec<(u32, &'a str)>)>;

fn parser<'a>(input: &'a str) -> IResult<&'a str, ParseResult<'a>> {
    separated_list1(
        tag("\n"),
        tuple((
            terminated(bag_parser, tag(" contain ")),
            terminated(
                alt((
                    map(tag("no other bags"), |_| Vec::new()),
                    separated_list1(
                        tag(", "),
                        tuple((
                            map(terminated(digit1, char(' ')), |n: &str| n.parse::<u32>().unwrap()),
                            bag_parser,
                        ))))),
                char('.'))
        )))
    (input)
}


fn propagate_union_of_elements<T: Ord + Copy>(mut sets: BTreeMap<T, BTreeSet<T>>) -> BTreeMap<T, BTreeSet<T>> {
    let colors: Vec<_> = sets.keys().cloned().collect();
    loop {
        let mut modified = false;
        for color in &colors {
            let mut color_set = sets[color].clone();
            let mut color_modified = false;
            for sub_color in &sets[color] {
                let sub_set = &sets[sub_color];
                if !sets[color].is_superset(sub_set) {
                    modified = true;
                    color_modified = true;
                    color_set = color_set.union(sub_set).cloned().collect();
                }
            }
            if color_modified {
                sets.insert(*color, color_set);
            }
        }
        if !modified {
            break;
        }
    }
    sets
}


pub fn problem7a(demo: bool) {
    let path = if demo { "inputs/input7demo.txt" } else { "inputs/input7.txt" };
    let s = read(path);
    let parse_result = parser(s.as_str()).unwrap().1;

    let mut sets = BTreeMap::new();
    for (color, contents) in parse_result {
        let mut set = BTreeSet::new();
        for (_, sub_color) in contents {
            set.insert(sub_color);
        }
        sets.insert(color, set);
    }

    sets = propagate_union_of_elements(sets);
    let res = sets.values().filter(|s| s.contains("shiny gold")).count();
    println!("{:?}", res);
}

fn step(sets: &BTreeMap<&str, Vec<(u32, &str)>>, start: &str) -> u32 {
    sets[start].iter().map(|(n, c)| n * (1 + step(sets, c))).sum()
}

pub fn problem7b(demo: bool) {
    let path = if demo { "inputs/input7demo.txt" } else { "inputs/input7.txt" };
    let s = read(path);
    let parse_result = parser(s.as_str()).unwrap().1;
    let sets: BTreeMap<_, _> = parse_result.into_iter().collect();
    println!("{}", step(&sets, "shiny gold"));
}