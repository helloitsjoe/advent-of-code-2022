use std::cmp::min;
use std::fs;

fn compare_lines(a: &str, b: &str) -> bool {
    let mut result: bool = false;

    for i in 0..min(a.len(), b.len()) {
        let a_char: char = a.chars().nth(i).unwrap();
        let b_char: char = b.chars().nth(i).unwrap();

        if a_char.is_digit(10) && b_char.is_digit(10) {
            if a_char != b_char {
                result = a_char < b_char;
            }
        }

        if a_char == '[' && b_char.is_digit(10) {
            let new_a_char = a.chars().nth(i + 1).unwrap();
            if new_a_char < b_char {
                return true;
            } else if new_a_char < b_char {
                return false;
            } else if new_a_char == b_char {
                if a.chars().nth(i + 2).unwrap() == ']' {
                    return true;
                }
            }
        }

        if b_char == '[' && a_char.is_digit(10) {
            let new_b_char = b.chars().nth(i + 1).unwrap();
            if a_char < new_b_char {
                return true;
            } else if new_b_char < a_char {
                return false;
            } else if new_b_char == a_char {
                if b.chars().nth(i + 2).unwrap() != ']' {
                    return true;
                }
            }
        }

        if a_char == ']' && b_char != ']' {
            return true;
        }
    }

    return result;
}

pub fn run() {
    let path = "./src/advent13.txt";
    let content = fs::read_to_string(path).expect("Error reading file");

    let mut right_order_indices: Vec<usize> = vec![];

    let pairs: Vec<Vec<&str>> = content
        .split("\n\n")
        .map(|pair| pair.lines().collect())
        .collect();

    for (i, pair) in pairs.iter().enumerate() {
        let line_a = pair[0];
        let line_b = pair[1];

        println!("====== START {}", i + 1);
        println!("A {:?}", line_a);
        println!("B {:?}", line_b);

        if compare_lines(line_a, line_b) == true {
            println!("correct");
            right_order_indices.push(i + 1);
        }
    }

    println!("{:?}", right_order_indices);
    println!("{:?}", right_order_indices.iter().sum::<usize>());
}
