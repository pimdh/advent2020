use std::fs::{read_to_string};
use std::iter::Iterator;
use std::collections::BTreeMap;
use std::str::FromStr;

use nom::{
    IResult,
    bytes::complete::{tag, take_until},
    character::complete::{digit1},
    combinator::{map, map_res},
    sequence::{tuple, terminated, separated_pair, delimited},
    multi::{separated_list1},
};

fn digit_parse<T: FromStr>(s: &str) -> IResult<&str, T> {
    map_res(digit1, str::parse::<T>)(s)
}

type DefEntry = (String, Vec<(u64, u64)>);
type ParseResult = (Vec<DefEntry>, Vec<u64>, Vec<Vec<u64>>);

fn parse(s: &str) -> ParseResult {
    let res: IResult<_, _> = tuple((
        separated_list1(
            tag("\n"),
            tuple((
                terminated(map(take_until(":"), |x: &str| x.to_owned()), tag(": ")),
                separated_list1(tag(" or "), separated_pair(digit_parse, tag("-"), digit_parse))
            ))
        ),
        delimited(
            tag("\n\nyour ticket:\n"),
            separated_list1(tag(","), digit_parse),
            tag("\n\nnearby tickets:\n"),
        ),
        separated_list1(tag("\n"), separated_list1(tag(","), digit_parse))
    ))(s);
    res.unwrap().1
}

pub fn in_ranges<T: Ord>(ranges: &[(T, T)], x: T) -> bool {
    for (start, end) in ranges {
        if x >= *start && x <= *end {
            return true
        }
    }
    false
}

fn ticket_error_rate(defs: &[DefEntry], ticket: &[u64]) -> u64 {
    let mut error_rate = 0;
    for &val in ticket {
        let mut val_valid = false;
        for (_, ranges) in defs {
            if in_ranges(ranges, val) {
                val_valid = true;
                break;
            }
        }
        if !val_valid {
            error_rate += val;
        }
    }
    error_rate
}

pub fn problem16a(demo: bool) {
    let path = if demo { "inputs/input16demo.txt" } else { "inputs/input16.txt" };
    let s = read_to_string(path).unwrap();
    let (defs, _, nearby_tickets) = parse(s.as_str());
    let error_rate: u64 = nearby_tickets.iter().map(|t| ticket_error_rate(&defs, t)).sum();
    println!("{}", error_rate);
}

pub fn problem16b(demo: bool) {
    let path = if demo { "inputs/input16demob.txt" } else { "inputs/input16.txt" };
    let s = read_to_string(path).unwrap();
    let (defs, your_ticket, nearby_tickets) = parse(s.as_str());
    let valid_tickets: Vec<_> = nearby_tickets.iter().filter(|t| ticket_error_rate(&defs, t) == 0).collect();

    let mut options = Vec::new();
    for i in 0..defs.len() {
        let mut field_options = vec![true; defs.len()];
        for ticket in &valid_tickets {
            let val = ticket[i];
            for (j, (_, ranges)) in defs.iter().enumerate() {
                if !in_ranges(ranges, val) {
                    field_options[j] = false;
                }
            }
        }
        options.push(field_options);
    }

    // Identify fields that can be uniquely determined, then disallow other fields to have determination
    let mut identified: BTreeMap<usize, usize> = BTreeMap::new();
    let n = options.len();
    while identified.len() < n {
        // println!("{:?}", options);
        let counts: Vec<u64> = options.iter().map(|field_opts| field_opts.iter().map(|&o| o as u64).sum()).collect();
        // println!("{:?}", counts);
        let (field, _) = counts.iter().enumerate().find(|(f, &c)| !identified.contains_key(f) && c == 1).unwrap();
        let (def_idx, _) = options[field].iter().enumerate().find(|(_, &o)| o).unwrap();
        identified.insert(field, def_idx);
        // println!("{:?} {} {} {:?}", counts, field, def_idx, identified);
        for (i, opt) in options.iter_mut().enumerate() {
            if i != field {
                opt[def_idx] = false;
            }
        }
    }

    let mut c = 1;
    for (&field, &def_idx) in &identified {
        let def_name = &defs[def_idx].0;
        println!("{}\t{}", field, def_name);
        if def_name.split(' ').next().unwrap() == "departure" {
            c *= your_ticket[field];
        }
    }
    println!("{}", c);
}