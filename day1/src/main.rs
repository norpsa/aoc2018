use std::collections::HashSet;

const INPUT_FILE: &str = include_str!("../input.txt");

fn main() {
    let result1 = solve1();
    println!("PART 1: {}", result1);

    let result2 = solve2();
    println!("PART 2: {}", result2);
}

fn solve1() -> i32 {
    let mut sum = 0;

    for line in INPUT_FILE.lines() {
        sum += line.parse::<i32>().unwrap();
    }

    return sum;
}

fn solve2() -> i32 {

    let mut results: HashSet<i32> = HashSet::new();
    let mut sum = 0;

    loop {
        for line in INPUT_FILE.lines() {
            sum += line.parse::<i32>().unwrap();
            if results.contains(&sum) {
                return sum;
            }
            results.insert(sum);
        }
    }
}