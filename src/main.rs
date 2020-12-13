#[macro_use]
extern crate lazy_static;

mod day1; 
mod day2; 
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;
mod day12;
mod day13;

fn main() {
    let arg = std::env::args().nth(1).expect("no pattern given");
    let is_demo = std::env::args().nth(2).filter(|x| x == "demo").is_some();
    match arg.as_str() {
        "1a" => day1::problem1a(),
        "1b" => day1::problem1b(),
        "2a" => day2::problem2a(is_demo),
        "2b" => day2::problem2b(is_demo),
        "3a" => day3::problem3a(is_demo),
        "3b" => day3::problem3b(is_demo),
        "4a" => day4::problem4a(is_demo),
        "4b" => day4::problem4b(is_demo),
        "5a" => day5::problem5a(is_demo),
        "5b" => day5::problem5b(is_demo),
        "6a" => day6::problem6a(is_demo),
        "6b" => day6::problem6b(is_demo),
        "7a" => day7::problem7a(is_demo),
        "7b" => day7::problem7b(is_demo),
        "8a" => day8::problem8a(is_demo),
        "8b" => day8::problem8b(is_demo),
        "9a" => day9::problem9a(is_demo),
        "9b" => day9::problem9b(is_demo),
        "10a" => day10::problem10a(is_demo),
        "10b" => day10::problem10b(is_demo),
        "11a" => day11::problem11a(is_demo),
        "11b" => day11::problem11b(is_demo),
        "12a" => day12::problem12a(is_demo),
        "12b" => day12::problem12b(is_demo),
        "13a" => day13::problem13a(is_demo),
        "13b" => day13::problem13b(is_demo),
        &_ => println!("Problem not found")
    }
}
