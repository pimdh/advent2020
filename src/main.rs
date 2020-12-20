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
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;
mod day19;
mod day20;
mod day21;
mod day22;

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
        "14a" => day14::problem14a(is_demo),
        "14b" => day14::problem14b(is_demo),
        "15a" => day15::problem15a(is_demo),
        "15b" => day15::problem15b(is_demo),
        "16a" => day16::problem16a(is_demo),
        "16b" => day16::problem16b(is_demo),
        "17a" => day17::problem17a(is_demo),
        "17b" => day17::problem17b(is_demo),
        "18a" => day18::problem18a(is_demo),
        "18b" => day18::problem18b(is_demo),
        "19a" => day19::problem19a(is_demo),
        "19b" => day19::problem19b(is_demo),
        "20a" => day20::problem20a(is_demo),
        "20b" => day20::problem20b(is_demo),
        "21a" => day21::problem21a(is_demo),
        "21b" => day21::problem21b(is_demo),
        "22a" => day22::problem22a(is_demo),
        "22b" => day22::problem22b(is_demo),
        &_ => println!("Problem not found")
    }
}
