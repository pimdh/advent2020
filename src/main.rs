#[macro_use]
extern crate lazy_static;

mod day1; 
mod day2; 
mod day3;
mod day4;

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
        &_ => println!("Problem not found")
    }
}
