use crate::utils::read_file;

fn split_on(c: char, s: &str) -> (&str, &str) {
    let mut split = s.split(c);
    let head = split.next().unwrap_or("No head!");
    let tail = split.next().unwrap_or("No tail!");
    return (head, tail);
}

fn to_i32(range: (&str, &str)) -> (i32, i32) {
    // Input is all valid i32
    return (range.0.parse().unwrap(), range.1.parse().unwrap());
}

// fn part_one_check(a_range: (i32, i32), b_range: (i32, i32)) -> bool {
//     let (a_head, a_tail) = a_range;
//     let (b_head, b_tail) = b_range;

//     // Completely overlapping
//     if a_head >= b_head && a_tail <= b_tail {
//         return true;
//     }

//     if b_head >= a_head && b_tail <= a_tail {
//         return true;
//     }

//     return false;
// }

fn part_two_check(a_range: (i32, i32), b_range: (i32, i32)) -> bool {
    let (a_head, a_tail) = a_range;
    let (b_head, b_tail) = b_range;

    // Partially overlapping
    if a_head >= b_head && a_head <= b_tail {
        return true;
    }

    if b_head >= a_head && b_head <= a_tail {
        return true;
    }

    return false;
}

fn fully_contains(a: &str, b: &str) -> bool {
    // println!("{}, {}", a, b);
    let a_range = split_on('-', a);
    let b_range = split_on('-', b);

    // return part_one_check(to_i32(a_range), to_i32(b_range));
    return part_two_check(to_i32(a_range), to_i32(b_range));
}

// In how many assignment pairs does one range fully contain the other?
pub fn run() {
    let file = read_file("./src/advent04.txt");

    let num_fully_contained: i32 = file
        .trim()
        .split("\n")
        .map(|line| {
            let (head, tail) = split_on(',', line);
            if fully_contains(head, tail) {
                return 1;
            } else {
                return 0;
            }
        })
        .sum();

    println!("{}", num_fully_contained);
}
