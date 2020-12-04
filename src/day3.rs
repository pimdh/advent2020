use std::fs::File;
use std::io::{self, BufRead};
use std::iter::Iterator;
use std::fmt; 
use Field::{Tree, Open};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    Open,
    Tree
}

#[derive(Debug)]
struct Map {
    height: usize,
    width: usize,
    data: Vec<Field>,
}



impl Map {
    fn get(&self, i: usize, j: usize) -> Field {
        self.data[i * self.width + j % self.width]
    }

    fn read(path: &str) -> Map {
        let file = File::open(path).unwrap();

        let mut data = Vec::new();
        let mut num_lines = 0;

        for line in io::BufReader::new(file).lines() {
            for ch in line.unwrap().chars() {
                data.push(match ch {
                    '.' => Open,
                    '#' => Tree,
                    _ => panic!("Character {} could not be parsed.", ch)
                })
            }
            
            num_lines += 1
        }

        Map {
            height: num_lines,
            width: data.len() / num_lines,
            data
        }
    }
}

#[allow(clippy::write_with_newline)]
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.width {
            for j in 1..self.height {
                let ch = match self.get(i, j) {
                    Open => ".",
                    Tree => "#",
                };
                write!(f, "{}", ch).unwrap();
            }
            write!(f, "\n").unwrap();
        }
        Ok(())
    }
}


fn count_hits(map: &Map, di: usize, dj: usize) -> u32 {
    let mut trees_hit = 0;
    let mut i = 0;
    let mut j = 0;
    while i < map.height {
        trees_hit += match map.get(i, j) { Tree => 1, Open => 0 };
        i += di;
        j += dj;
    }
    trees_hit
}

pub fn problem3a(demo: bool) {
    let path = if demo { "inputs/input3demo.txt" } else { "inputs/input3a.txt" };
    let map = Map::read(path);
    println!("{}", count_hits(&map, 1, 3));
}

pub fn problem3b(demo: bool) {
    let path = if demo { "inputs/input3demo.txt" } else { "inputs/input3a.txt" };
    let map = Map::read(path);
    let rules = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let res: u32 = rules.iter().map(|(dj, di)| count_hits(&map, *di, *dj)).product();
    println!("{}", res);
}