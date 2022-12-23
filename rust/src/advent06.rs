use std::collections::HashSet;

const UNIQUE_LENGTH: usize = 14;

pub fn run() {
    let chars = std::fs::read_to_string("./src/advent06.txt").expect("Error reading file");
    let by = chars.as_bytes();

    let mut a = 0;
    let mut b = UNIQUE_LENGTH;

    while b < chars.len() {
        let mut hs = HashSet::<u8>::new();
        for ch in a..b {
            hs.insert(by[ch]);
        }
        // println!("{:?}", hs);

        if hs.len() == UNIQUE_LENGTH {
            break;
        }

        a += 1;
        b += 1;
    }

    println!("{}", b);
}
