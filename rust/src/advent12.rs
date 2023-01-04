use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn new(x: usize, y: usize) -> Point {
        Point { x, y }
    }
}

struct PointDelta {
    x: isize,
    y: isize,
    dir: char,
}

impl PointDelta {
    fn new(x: isize, y: isize, dir: char) -> PointDelta {
        PointDelta { x, y, dir }
    }
}

fn find_possible_next_moves(rows: &Vec<&str>, curr: &Point, prev: &Point) -> Vec<char> {
    // Check N, E, S, W for any letter 1 above, equal, or any below
    // Ignore letters >1 above current
    // Ignore path already taken
    let mut output = Vec::new();
    let deltas = vec![
        PointDelta::new(0, -1, 'N'),
        PointDelta::new(1, 0, 'E'),
        PointDelta::new(0, 1, 'S'),
        PointDelta::new(-1, 0, 'W'),
    ];
    // Get bytes for curr char
    let mut curr_char = rows[curr.y].as_bytes()[curr.x];
    // Convert S (start) to a
    if curr_char == 83 {
        curr_char = 97;
    }

    for tup in deltas {
        let new = Point {
            y: (curr.y as isize + tup.y) as usize,
            x: (curr.x as isize + tup.x) as usize,
        };

        // if let Some(new) = new_opt {
        // Ignore the point we came from
        if prev.x == new.x && prev.y == new.y {
            continue;
        }

        // Check that new point is no more than 1 letter above current
        // use rows.get() for safe access that returns an Option
        if let Some(row) = rows.get(new.y) {
            if let Some(_) = row.as_bytes().get(new.x) {
                if rows[new.y].as_bytes()[new.x] <= (1 + curr_char) {
                    output.push(tup.dir);
                }
            }
        }
    }

    return output;
}

fn find_start(lines: &Vec<&str>) -> Option<Point> {
    for (y, &line) in lines.iter().enumerate() {
        for (x, letter) in line.chars().enumerate() {
            if letter == 'S' {
                return Some(Point { x, y });
            }
        }
    }

    return None;
}

fn is_end(pt: &Point, lines: &Vec<&str>) -> bool {
    return lines[pt.y].as_bytes()[pt.x] == 69; // 'E'
}

fn get_next_from_dir(prev: &Point, dir: char) -> Point {
    return match dir {
        'N' => Point::new(prev.x, prev.y - 1),
        'E' => Point::new(prev.x + 1, prev.y),
        'S' => Point::new(prev.x, prev.y + 1),
        'W' => Point::new(prev.x - 1, prev.y),
        _ => panic!("Ah!"),
    };
}

fn find_all(next_moves: &Vec<char>, prev: &Point, lines: &Vec<&str>) {
    if is_end(prev, lines) == true {
        println!("End! {:?}", prev);
        return;
    }
    for &dir in next_moves {
        let curr = &get_next_from_dir(&prev, dir);
        let next_next = find_possible_next_moves(lines, curr, prev);
        find_all(&next_next, curr, lines);
        println!("{:?}", next_next);
    }
}

pub fn run() {
    // Start at S, end at E
    // move up, down, left, right
    // Can only move 1 letter greater than current letter

    // Do one pass, determine the shortest path to every square
    // Store in a map? { "x:y": { num: 3, moves: [0, 2, 1] } }
    // Find ANY route from S to E
    // Avoid revisiting an already seen point
    let content = fs::read_to_string("./src/advent12_test_input.txt").expect("Cannot read file");
    let lines: Vec<&str> = content.lines().collect();
    let start = match find_start(&lines) {
        Some(p) => p,
        None => panic!("Gah!"),
    };

    find_all(
        &find_possible_next_moves(&lines, &start, &start),
        &start,
        &lines,
    );
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

#[test]
fn test_possible_next_end() {
    let lines = vec!["aaa", "aaa", "aaa"];
    let curr = Point { x: 2, y: 2 };
    let prev = Point { x: 1, y: 2 };
    assert_eq!(find_possible_next_moves(&lines, &curr, &prev), ['N']);
}
