use std::fs;

fn split_on<'a>(line: &'a str, split_on: &'a str) -> &'a str {
    return line.split(split_on).collect::<Vec<_>>()[1];
}

fn operate(old: u64, operation: &str) -> u64 {
    let pieces = operation.split(" ").collect::<Vec<_>>();
    let a = old;
    let b = if pieces[2] == "old" {
        old
    } else {
        pieces[2].parse::<u64>().expect("Not an int")
    };

    return match pieces[1] {
        "+" => a + b,
        "-" => a - b,
        "*" => a * b,
        "/" => a / b,
        _ => panic!("uncovered case"),
    };
}

fn throw_to(input: u64, divisible_by: u64, true_case: u64, false_case: u64) -> usize {
    return if input % divisible_by == 0 {
        true_case as usize
    } else {
        false_case as usize
    };
}

#[derive(Debug)]
struct Monkey<'a> {
    starting_items: Vec<u64>,
    operation: &'a str,
    test: u64,
    true_case: u64,
    false_case: u64,
}

pub fn run() {
    let file = fs::read_to_string("./src/advent11.txt").expect("Error reading file");
    let mut monkeys: Vec<Monkey> = file
        .split("\n\n")
        .map(|monkey| {
            let lines = monkey.lines().collect::<Vec<&str>>();

            let starting_items = split_on(lines[1], "Starting items: ")
                .split(", ")
                .map(|num| num.parse().expect("Was not a number"))
                .collect::<Vec<u64>>();
            let operation = split_on(lines[2], "Operation: new = ");
            let test = split_on(lines[3], "Test: divisible by ")
                .parse::<u64>()
                .expect("Not an int");
            let true_case = split_on(lines[4], "If true: throw to monkey ")
                .parse::<u64>()
                .expect("Not an int");
            let false_case = split_on(lines[5], "If false: throw to monkey ")
                .parse::<u64>()
                .expect("Not an int");

            return Monkey {
                starting_items,
                operation,
                test,
                true_case,
                false_case,
            };
        })
        .collect();

    // for monkey in &monkeys {
    //     println!("{:?}", monkey.starting_items);
    // }

    // Each monkey runs through all items before moving onto the next monkey
    // For each starting item:
    // 1. Apply operation
    // 2. Divide by 3
    // 3. Run test to see where it's thrown next
    let mut monkey_business = vec![0; monkeys.len()];

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            // TODO: How to do this with a var?
            // let monkey = &monkeys[i];

            // Add monkey activity
            monkey_business[i] += monkeys[i].starting_items.len();

            for j in 0..monkeys[i].starting_items.len() {
                let num = monkeys[i].starting_items[j];
                let operated = operate(num, monkeys[i].operation);
                let operated = operated / 3;
                let new_monkey = throw_to(
                    operated,
                    monkeys[i].test,
                    monkeys[i].true_case,
                    monkeys[i].false_case,
                );
                monkeys[new_monkey].starting_items.push(operated);
            }
            monkeys[i].starting_items = Vec::new();
        }
    }

    for monkey in monkeys {
        println!("{:?}", monkey.starting_items);
    }

    // Get two most active monkeys:
    // Count total number of times monkeys inspect items over 20 rounds
    // Get top two and multiply together
    monkey_business.sort();
    monkey_business.reverse();

    // for business in monkey_business {
    //     println!("{:?}", business);
    // }

    println!("{:?}", monkey_business[0] * monkey_business[1]);
}

#[test]
fn test_throw_to() {
    assert_eq!(throw_to(4, 2, 2, 1), 2);
    assert_eq!(throw_to(4, 3, 2, 1), 1);
}

#[test]
fn test_operate() {
    assert_eq!(operate(5, "old + 3"), 8);
    assert_eq!(operate(5, "old * 3"), 15);
    assert_eq!(operate(5, "old * old"), 25);
}
