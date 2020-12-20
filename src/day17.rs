use std::fs::File;
use std::io::{self, BufRead};
use std::collections::HashSet;
// use std::iter::Iterator;
use cgmath::{Vector3, Vector4};
use std::ops::Add;
use std::hash::Hash;

fn build_deltas3() -> Vec<Vector3<i64>> {
    let mut ds = Vec::with_capacity(26);
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                if !(dx == 0 && dy == 0 && dz == 0) {
                    ds.push(Vector3::new(dx, dy, dz));
                }
            }
        }
    }
    ds
}
fn build_deltas4() -> Vec<Vector4<i64>> {
    let mut ds = Vec::with_capacity(80);
    for dx in -1..=1 {
        for dy in -1..=1 {
            for dz in -1..=1 {
                for dw in -1..=1 {
                    if !(dx == 0 && dy == 0 && dz == 0 && dw == 0) {
                        ds.push(Vector4::new(dx, dy, dz, dw));
                    }
                }
            }
        }
    }
    ds
}

fn read(path: &str) -> impl Iterator<Item=(i64, i64)> {
    let file = File::open(path).unwrap();
    let mut set = HashSet::new();
    for (y, line) in io::BufReader::new(file).lines().enumerate() {
        let l = line.unwrap();
        for (x, c) in l.chars().enumerate() {
            if c == '#' {
                set.insert((x as i64, y as i64));
            }
        }
    }
    set.into_iter()
}

fn active_region<T: Add<Output=T> + Eq + Hash + Copy>(map: &HashSet<T>, deltas: &[T]) -> HashSet<T> {
    let mut region = HashSet::new();
    for &p in map {
        region.insert(p);
        for &d in deltas {
            region.insert(p + d);
        }
    }
    region
}

fn step<T: Hash + Add<Output=T> + Eq + Hash + Copy>(map: HashSet<T>, deltas: &[T]) -> HashSet<T> {
    let mut new_map = HashSet::new();
    for p in active_region(&map, deltas) {
        let mut c = 0;
        for &d in deltas {
            c += map.contains(&(p + d)) as i64;
        }
        if c == 3 || (map.contains(&p) && c == 2)  {
            new_map.insert(p);
        }
    }
    new_map
}

pub fn problem17a(demo: bool) {
    let path = if demo { "inputs/input17demo.txt" } else { "inputs/input17.txt" };
    let map = read(path).map(|(x, y)| Vector3::new(x, y, 0)).collect();
    let deltas = build_deltas3();
    let map = (0..6).fold(map, |m, _| step(m, &deltas));
    println!("{:?}", map.len());
}

pub fn problem17b(demo: bool) {
    let path = if demo { "inputs/input17demo.txt" } else { "inputs/input17.txt" };
    let map = read(path).map(|(x, y)| Vector4::new(x, y, 0, 0)).collect();
    let deltas = build_deltas4();
    let map = (0..6).fold(map, |m, _| step(m, &deltas));
    println!("{:?}", map.len());
}