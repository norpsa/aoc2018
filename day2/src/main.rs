const INPUT_FILE: &str = include_str!("../input.txt");
use std::collections::HashMap;

fn main() {

    let result1 = solve1();
    println!("PART 1: {}", result1);

    let result2 = solve2();
    println!("PART 2: {}", result2);
    
}

fn solve1() -> i32 {
    let mut sum_contains_2 = 0;
    let mut sum_contains_3 = 0;

    for line in INPUT_FILE.lines() {

        let mut letter_map = HashMap::new();

        for letter in line.chars() {
            *letter_map.entry(letter).or_insert(0) += 1;
        }

        let mut contains_2 = false;
        let mut contains_3 = false;

        for (_key, val) in letter_map {
            if val == 2 {
                contains_2 = true;
            } else if val == 3 {
                contains_3 = true;
            }
        }

        if contains_2 {
            sum_contains_2 += 1;
        }

        if contains_3 {
            sum_contains_3 += 1;
        }
    }

    return sum_contains_2*sum_contains_3;
}

fn solve2() -> String {
    for (i, word1) in INPUT_FILE.lines().enumerate() {
        for word2 in INPUT_FILE.lines().skip(i + 1) {
            
            let word_len = word1.len();
            let mut s = String::new();

            for j in 0..word_len {
                if word1.chars().nth(j).expect("Invalid length") == word2.chars().nth(j).expect("Invalid length") {
                    s.push(word1.chars().nth(j).expect("Invalid length"));
                }
            }
            
            if s.len() == (word_len - 1) {
                return s;
            }
        }
    }

    return String::new();
}
