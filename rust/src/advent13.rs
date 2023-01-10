use serde_json::Value;
use std::cmp::min;
use std::fs;

fn compare_int(a: u64, b: u64) -> bool {
    if a == b || a > b {
        return false;
    } else {
        return true;
    }
}

fn compare_arr(a: Vec<Value>, b: Vec<Value>) -> bool {
    if a.len() < b.len() {
        return true;
    } else {
        return false;
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
        let line_a: Vec<Value> = serde_json::from_str(pair[0]).unwrap();
        let line_b: Vec<Value> = serde_json::from_str(pair[1]).unwrap();

        for j in 0..min(line_a.len(), line_b.len()) {
            let val_a: &Value = &line_a[j];
            let val_b: &Value = &line_b[j];

            let correct: bool = match (val_a, val_b) {
                (Value::Number(a), Value::Number(b)) => {
                    compare_int(a.as_u64().unwrap(), b.as_u64().unwrap())
                }
                (Value::Array(a), Value::Array(b)) => compare_arr(a.to_vec(), b.to_vec()),
                _ => false,
            };

            if correct == true {
                right_order_indices.push(i);
            }
        }
    }

    println!("{:?}", right_order_indices);
}
