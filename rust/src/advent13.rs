use std::fs;

pub fn run() {
    let path = "./src/advent13_test_input.txt";
    let content = fs::read_to_string(path).expect("Error reading file");
    let pairs: Vec<Vec<&str>> = content
        .split("\n\n")
        .map(|pair| pair.lines().collect::<Vec<&str>>())
        .collect();

    for pair in pairs {
        println!("{:?}", pair);
    }
}
