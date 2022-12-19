use std::collections::HashMap;
use std::fs;

fn calculate_score(a: &str, b: &str, values: &mut HashMap<&str, i32>) -> i32 {
    let val_a = *values.get(a).unwrap();
    let val_b = *values.get(b).unwrap();

    let score = if val_a == val_b {
        val_b + 3
    } else if val_b == val_a + 1 || val_b == val_a - 2 {
        val_b + 6
    } else {
        val_b
    };

    return score;
}

pub fn run() {
    // Create value map
    let mut values = HashMap::<&str, i32>::new();
    values.insert("A", 1);
    values.insert("B", 2);
    values.insert("C", 3);
    values.insert("X", 1);
    values.insert("Y", 2);
    values.insert("Z", 3);

    // Read file, break on newlines
    let file = fs::read_to_string("./src/advent02.txt").expect("Could not read file");
    let games = file
        .trim()
        .split("\n")
        .map(|s| s.split(" ").collect::<Vec<&str>>())
        .map(|v| return calculate_score(v[0], v[1], &mut values))
        // Sum values
        .sum::<i32>();

    println!("{:?}", games)
}
