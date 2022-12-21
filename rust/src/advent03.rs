use std::collections::HashMap;
use std::fs;

fn convert_char_to_priority(c: char) -> i32 {
    let num = c as i32;
    if num > 96 {
        return num - 96;
    }
    return num - 64 + 26;
}

pub fn run() {
    // Priority = a-z 1-26 A-Z 27-52
    // Read file
    let total: i32 = fs::read_to_string("./src/advent03.txt")
        .expect("Error reading file")
        .split("\n")
        .map(|line| {
            let mut map = HashMap::<char, bool>::new();
            let len = line.chars().count();

            // Split each line in half
            for letter in line[0..len / 2].chars() {
                map.insert(letter, true);
            }
            for letter in line[len / 2..].chars() {
                // Find duplicate char
                if map.contains_key(&letter) {
                    let priority = convert_char_to_priority(letter);
                    println!("{}", letter);
                    println!("{}", priority);
                    return priority;
                }
            }
            return 0;
        })
        // Sum priorities of all duplicates
        .sum();

    println!("{}", total)
}
