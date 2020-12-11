use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::Iterator;
use std::fmt; 
use Field::{Floor, Occupied, Empty};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Field {
    Floor,
    Occupied,
    Empty,
}

impl Field {
    fn parse(ch: char) -> Field {
    match ch {
        '.' => Floor,
        'L' => Empty,
        '#' => Occupied,
        _ => panic!("Character {} could not be parsed.", ch)
    }}

    fn to_char(&self) -> char {
        match self {
            Floor => '.',
            Empty => 'L',
            Occupied => '#',
        }

    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    height: usize,
    width: usize,
    data: Vec<Field>,
}

impl Map {
    fn get(&self, i: isize, j: isize) -> Field {
        self.data[(i as usize) * self.width + (j as usize)]
    }

    fn is_occupied(&self, i: usize, j: usize, di: isize, dj: isize, scan: bool) -> bool {
        let mut i = i as isize;
        let mut j = j as isize;
        loop {
            i += di;
            j += dj;
            if i < 0 || i >= self.height as isize || j < 0 || j >= self.width as isize {
                return false;
            } else if self.get(i, j) == Occupied {
                return true;
            } else if !scan || self.get(i, j) == Empty {
                return false;
            }
        }
    }

    fn read(path: &str) -> Map {
        let file = File::open(path).unwrap();
        let lines = BufReader::new(file).lines();
        let mut num_lines = 0;
        let data: Vec<_> = lines.flat_map(|l| {
            num_lines += 1;
            l.unwrap().chars().map(Field::parse).collect::<Vec<_>>()
        }).collect();
        Map {
            height: num_lines,
            width: data.len() / num_lines,
            data
        }
    }

    fn occupancy_count(&self, scan: bool) -> Vec<i32> {
        (0..self.data.len()).map(|n| {
            let (i, j) = ((n / self.width), (n % self.width));
            let mut c = 0;
            for di in -1..=1 {
                for dj in -1..=1 {
                    if !(di == 0 && dj == 0)  {
                        c += self.is_occupied(i, j, di, dj, scan) as i32;
                    }
                }
            }
            c
        }).collect()
    }
}

#[allow(clippy::write_with_newline)]
impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.height {
            let line = &self.data[i * self.width..(i+1)*self.width];
            writeln!(f, "{}", line.iter().map(Field::to_char).collect::<String>()).unwrap();
        }
        Ok(())
    }
}

fn step_a(map: &Map) -> Map {
    let data = map.data.iter().zip(map.occupancy_count(false).iter()).map(|(&f, &c)| {
        if f == Empty && c == 0 { Occupied }
        else if f == Occupied && c >= 4 { Empty }
        else { f }
    }).collect();
    Map { height: map.height, width: map.width, data}
}

fn step_b(map: &Map) -> Map {
    let data = map.data.iter().zip(map.occupancy_count(true).iter()).map(|(&f, &c)| {
        if f == Empty && c == 0 { Occupied }
        else if f == Occupied && c >= 5 { Empty }
        else { f }
    }).collect();
    Map { height: map.height, width: map.width, data}
}

fn fix<T: Eq, F: Fn(&T) -> T>(f: F, mut x: T) -> T {
    loop {
        let y = f(&x);
        if y == x {
            return y;
        }
        x = y;
    }
}

pub fn problem11a(demo: bool) {
    let path = if demo { "inputs/input11demo.txt" } else { "inputs/input11.txt" };
    let map = Map::read(path);
    let res = fix(step_a, map).data.iter().filter(|&&f| f == Occupied).count();
    println!("{}", res);
}

pub fn problem11b(demo: bool) {
    let path = if demo { "inputs/input11demo.txt" } else { "inputs/input11.txt" };
    let map = Map::read(path);
    let res = fix(step_b, map).data.iter().filter(|&&f| f == Occupied).count();
    println!("{}", res);
}