use std::collections::HashMap;

fn iterate(initial: &[u64], num: u64) -> u64 {
    let mut hist = HashMap::new();
    let t_inital = initial.len() as u64;
    for (i, v) in initial.iter().enumerate() {
        let t = (i as u64) + 1;
        if t < t_inital {
            hist.insert(*v, t);
        }
    }
    let mut v = *initial.last().unwrap();
    for t in (t_inital+1)..=num {
        let v_prev = v;
        if let Some(t_prev) = hist.get(&v) {
            v = t - t_prev - 1;
        } else {
            v = 0;
        }
        hist.insert(v_prev, t-1);
    }
    v
}

pub fn problem15a(_demo: bool) {
    let initial = vec![0, 1, 5, 10, 3, 12, 19];
    println!("{}", iterate(&initial, 2020));
}

pub fn problem15b(_demo: bool) {
    let initial = vec![0, 1, 5, 10, 3, 12, 19];
    println!("{}", iterate(&initial, 30000000));
}