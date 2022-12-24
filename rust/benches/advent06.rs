use criterion::{black_box, criterion_group, criterion_main, Criterion};

// Couldn't figure out how to import this from src ¯\_(ツ)_/¯
use std::collections::{HashMap, HashSet};

const UNIQUE_LENGTH: usize = 14;

pub fn run_naive(_: i32) -> usize {
    let content = std::fs::read_to_string("./src/advent06.txt").expect("Error reading file");
    let chars: Vec<char> = content.chars().collect();

    let mut a = 0;
    let mut b = UNIQUE_LENGTH;

    while b < chars.len() {
        let mut hs = HashSet::<char>::new();
        for i in a..b {
            hs.insert(chars[i]);
        }
        // println!("{:?}", hs);

        if hs.len() == UNIQUE_LENGTH {
            break;
        }

        a += 1;
        b += 1;
    }

    // println!("{}", b);
    return b;
}

pub fn run(_: u32) -> usize {
    let content = std::fs::read_to_string("./src/advent06.txt").expect("Error reading file");
    let chars: Vec<char> = content.chars().collect();

    let mut a = 0;
    let mut b = UNIQUE_LENGTH;
    let mut map = HashMap::<char, i32>::new();

    for i in a..b {
        let k = chars[i];
        map.entry(k).or_insert(0);
        map.insert(k, 1 + map[&k]);
    }

    while b < chars.len() {
        let k_b = chars[b];
        map.entry(k_b).or_insert(0);
        map.insert(k_b, 1 + map[&k_b]);

        let k_a = chars[a];
        if map[&k_a] > 1 {
            map.insert(k_a, map[&k_a] - 1);
        } else {
            map.remove(&k_a);
        }

        // println!("{:?}", map);

        if map.len() == UNIQUE_LENGTH {
            break;
        }

        a += 1;
        b += 1;
    }

    return b;
}

fn criterion_benchmark(c: &mut Criterion) {
    // c.bench_function("advent 06 naive", |b| b.iter(|| run_naive(black_box(1))));
    c.bench_function("advent 06", |b| b.iter(|| run(black_box(1))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
