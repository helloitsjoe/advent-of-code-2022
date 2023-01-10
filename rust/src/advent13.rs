use serde_json::{
    Value,
    Value::{Array, Number},
};
use std::cmp::min;
use std::fs;

fn compare_int(a: u64, b: u64) -> Option<bool> {
    // println!("int a {:?}", a);
    // println!("int b {:?}", b);
    if a == b {
        return None;
    } else {
        return Some(a < b);
    }
}

fn compare_arr(a: Vec<Value>, b: Vec<Value>) -> Option<bool> {
    // println!("compare_arr a {:?}", a);
    // println!("compare_arr b {:?}", b);

    for i in 0..min(a.len(), b.len()) {
        let a_val = &a[i];
        let b_val = &b[i];

        if a_val != b_val {
            return compare_match(a_val, b_val);
        }
    }

    return Some(a.len() < b.len());
}

fn compare_match(val_a: &Value, val_b: &Value) -> Option<bool> {
    return match (val_a, val_b) {
        (Number(a), Number(b)) => compare_int(a.as_u64().unwrap(), b.as_u64().unwrap()),
        (Array(a), Array(b)) => compare_arr(a.to_vec(), b.to_vec()),
        (Number(_), Array(b)) => compare_arr(vec![val_a.clone()], b.to_vec()),
        (Array(a), Number(_)) => compare_arr(a.to_vec(), vec![val_b.clone()]),
        _ => {
            println!("unknown");
            return Some(false);
        }
    };
}

pub fn run() {
    let path = "./src/advent13_test_input.txt";
    let content = fs::read_to_string(path).expect("Error reading file");

    let mut right_order_indices: Vec<usize> = vec![];

    let pairs: Vec<Vec<&str>> = content
        .split("\n\n")
        .map(|pair| pair.lines().collect())
        .collect();

    for (i, pair) in pairs.iter().enumerate() {
        let line_a: Value = serde_json::from_str(pair[0]).unwrap();
        let line_b: Value = serde_json::from_str(pair[1]).unwrap();

        // println!("====== START {}", i + 1);

        if let Some(true) = compare_match(&line_a, &line_b) {
            // println!("correct");
            right_order_indices.push(i + 1);
        }
    }

    println!("{:?}", right_order_indices);
    println!("{:?}", right_order_indices.iter().sum::<usize>());
}
