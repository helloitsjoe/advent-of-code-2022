use std::collections::HashMap;
use std::fs;

// Part two:
// Badge is the only item carried by all three elves
// Find unique letter in chunks of three lines

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

pub fn run_part_two() {
    let lines = fs::read_to_string("./src/advent03.txt").expect("Error reading file");

    let mut map = HashMap::<char, bool>::new();
    let mut map_two = HashMap::<char, bool>::new();
    let mut index = 0;
    let mut total = 0;
    let lines = lines.split("\n").collect::<Vec<&str>>();
    for (i, line) in lines.iter().enumerate() {
        if index == 0 {
            for letter in line.chars() {
                map.insert(letter, true);
            }
        }

        if index == 1 {
            for letter in line.chars() {
                if map.contains_key(&letter) {
                    map_two.insert(letter, true);
                }
            }
        }

        if index == 2 {
            for letter in line.chars() {
                if map.contains_key(&letter) && map_two.contains_key(&letter) {
                    let value = convert_char_to_priority(letter);
                    total += value;
                    map.clear();
                    map_two.clear();
                }
            }
        }

        if (i + 1) % 3 == 0 {
            index = 0;
        } else {
            index += 1;
        }
    }

    println!("{}", total)
}
