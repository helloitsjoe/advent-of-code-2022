use std::collections::HashMap;
use std::fs;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
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

type Visited = HashMap<Point, u32>;

fn sanitize_char(pt: &Point, rows: &Vec<&str>) -> u8 {
    let curr_char = rows[pt.y].as_bytes()[pt.x];

    if curr_char == 83 {
        // Convert S (start) to a
        return 97;
    } else if curr_char == 69 {
        // Convert E (end) to z
        return 122;
    }

    return curr_char;
}

fn find_possible_next_moves(
    rows: &Vec<&str>,
    curr: &Point,
    prev: &Point,
    // TODO: Maybe refactor to do map/distance check outside this function
    map: &mut Visited,
    distance: u32,
) -> Vec<char> {
    // Check N, E, S, W for any letter 1 above, equal, or any below
    // Ignore letters >1 above current
    // Check map to see if point has been visited.
    // If so only add it as an option if distance is <= current node + 1
    let mut output = Vec::new();
    let deltas = vec![
        PointDelta::new(0, -1, 'N'),
        PointDelta::new(1, 0, 'E'),
        PointDelta::new(0, 1, 'S'),
        PointDelta::new(-1, 0, 'W'),
    ];

    let curr_char = sanitize_char(curr, rows);

    for tup in deltas {
        let new = Point {
            y: (curr.y as isize + tup.y) as usize,
            x: (curr.x as isize + tup.x) as usize,
        };

        // Ignore the point we came from
        if prev.x == new.x && prev.y == new.y {
            continue;
        }

        // Ignore points we've visited unless distance is less
        if map.contains_key(&new) && map[&new] < distance {
            continue;
        }

        // Check that new point is no more than 1 letter above current
        // use rows.get() for safe access that returns an Option
        if let Some(row) = rows.get(new.y) {
            if let Some(_) = row.as_bytes().get(new.x) {
                let new_char = sanitize_char(&new, rows);
                if new_char <= (1 + curr_char) {
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

fn find_route(
    next_moves: &Vec<char>,
    prev: &Point,
    lines: &Vec<&str>,
    map: &mut Visited,
    distance: u32,
) -> Option<Point> {
    if is_end(prev, lines) == true {
        println!("End! {:?}", prev);
        println!("Moves: {:?}", map[prev]);
        // for (k, v) in map {
        //     println!("{:?} {}", k, v);
        // }
        // return *prev;
        return Some(prev.clone());
    } else if next_moves.len() == 0 {
        return None;
    }
    let mut routes = vec![];
    for &dir in next_moves {
        let curr = &get_next_from_dir(&prev, dir);
        let distance = distance + 1;
        // TODO: Use a set and overwrite with lowest distance?
        map.insert(curr.clone(), distance);
        let next_next = &find_possible_next_moves(lines, curr, prev, map, distance);
        println!("{:?} {:?}", curr, next_next);
        if let Some(route) = find_route(next_next, curr, lines, map, distance) {
            routes.push(route);
        }
    }
    // println!("{:?}", routes);
    return None;
}

pub fn run() {
    // Start at S, end at E
    // move up, down, left, right
    // Can only move 1 letter greater than current letter

    // Do one pass, determine the shortest path to every square
    // Store in a map? { "x:y": { num: 3, moves: [0, 2, 1] } }
    // Find ANY route from S to E
    // Avoid revisiting an already seen point
    // let path = "./src/advent12_test_input.txt";
    let path = "./src/advent12.txt";
    let content = fs::read_to_string(path).expect("Cannot read file");
    let lines: Vec<&str> = content.lines().collect();
    let start = match find_start(&lines) {
        Some(p) => p,
        None => panic!("Gah!"),
    };

    let mut map: Visited = HashMap::new();
    let next_moves = find_possible_next_moves(&lines, &start, &start, &mut map, 0);
    find_route(&next_moves, &start, &lines, &mut map, 0);
}

#[test]
fn test_possible_next() {
    let lines = vec!["aaa", "caa", "aba"];
    let curr = Point { x: 1, y: 1 };
    let prev = Point { x: 1, y: 0 };
    let mut map = Visited::new();
    let next_moves = find_possible_next_moves(&lines, &curr, &prev, &mut map, 0);
    assert_eq!(next_moves, ['E', 'S']);
}

#[test]
fn test_possible_next_origin() {
    let lines = vec!["aaa", "caa", "aba"];
    let origin = Point { x: 0, y: 0 };
    let mut map = Visited::new();
    let next_moves = find_possible_next_moves(&lines, &origin, &origin, &mut map, 0);
    assert_eq!(next_moves, ['E']);
}

#[test]
fn test_possible_next_last_point() {
    let lines = vec!["aaa", "aaa", "aaa"];
    let curr = Point { x: 2, y: 2 };
    let prev = Point { x: 1, y: 2 };
    let mut map = Visited::new();
    let next_moves = find_possible_next_moves(&lines, &curr, &prev, &mut map, 0);
    assert_eq!(next_moves, ['N']);
}

#[test]
fn test_possible_next_start() {
    // Start should be treated as 'a'
    let lines = vec!["caa", "Sca", "aaa"];
    let curr = Point { x: 0, y: 1 };
    let mut map = Visited::new();
    let next_moves = find_possible_next_moves(&lines, &curr, &curr, &mut map, 0);
    assert_eq!(next_moves, ['S']);
}

#[test]
fn test_possible_next_end() {
    // End should be treated as 'z'
    let lines = vec!["aya", "wxE", "aya"];
    let curr = Point { x: 1, y: 1 };
    let prev = Point { x: 0, y: 1 };
    let mut map = Visited::new();
    let next_moves = find_possible_next_moves(&lines, &curr, &prev, &mut map, 0);
    assert_eq!(next_moves, ['N', 'S']);
}

// #[test]
// fn test_possible_next_visited() {
//     let lines = vec!["aaa", "aaa", "aaa"];
//     let curr = Point { x: 1, y: 1 };
//     let prev = Point { x: 0, y: 1 };
//     assert_eq!(find_possible_next_moves(&lines, &curr, &prev), ['N', 'S']);
// }
