use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

fn find_possible_next_moves(rows: &Vec<&str>, curr: &Point, prev: &Point) -> Vec<char> {
    // Check N, E, S, W for any letter 1 above, equal, or any below
    // Ignore letters >1 above current
    // Ignore path already taken
    let mut output = Vec::new();
    // Get bytes for curr char
    let curr_char = rows[curr.y].as_bytes()[curr.x];
    for i in 0..4 {
        let new = match i {
            0 => Point {
                y: curr.y - 1,
                x: curr.x,
            },
            1 => Point {
                y: curr.y,
                x: curr.x + 1,
            },
            2 => Point {
                y: curr.y + 1,
                x: curr.x,
            },
            3 => Point {
                y: curr.y,
                x: curr.x - 1,
            },
            _ => panic!("No"),
        };

        if prev.x == new.x && prev.y == new.y {
            continue;
        }

        if rows[new.y].as_bytes()[new.x] <= (1 + curr_char) {
            output.push(i);
        }
    }
    return output
        .iter()
        .map(|num| match num {
            0 => 'N',
            1 => 'E',
            2 => 'S',
            3 => 'W',
            _ => 'X',
        })
        .collect::<Vec<char>>();
}

pub fn run() {
    // Start at S, end at E
    // move up, down, left, right
    // Can only move 1 letter greater than current letter

    // Do one pass, determine the shortest path to every square
    // Store in a map? { "x:y": { num: 3, moves: [0, 2, 1] } }
    // Find ANY route from S to E
    let content = fs::read_to_string("./src/advent12_test_input.txt").expect("Cannot read file");
    let lines: Vec<&str> = content.lines().collect();
    println!("{:?}", lines);
}

#[test]
fn test_possible_next() {
    let lines = vec!["aaa", "caa", "aba"];
    let curr = Point { x: 1, y: 1 };
    let prev = Point { x: 1, y: 0 };
    assert_eq!(find_possible_next_moves(&lines, &curr, &prev), ['E', 'S']);
}

#[test]
fn test_possible_next_origin() {
    let lines = vec!["aaa", "caa", "aba"];
    let origin = Point { x: 0, y: 0 };
    assert_eq!(find_possible_next_moves(&lines, &origin, &origin), ['E']);
}
