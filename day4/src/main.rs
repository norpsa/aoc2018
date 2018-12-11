extern crate regex;

use regex::Regex;
use std::collections::HashMap;

const INPUT_FILE: &str = include_str!("../input.txt");

fn main() {
    let mut entrys = Vec::new();
    for line in INPUT_FILE.lines() {
        entrys.push(line);
    }
    entrys.sort();

    let mut begin_sleep = 0;
    let mut end_sleep;
    let mut current_id = 0;
    let mut map = HashMap::new();
    for entry in entrys {
        let input_regex = Regex::new(r"#(\d+)").unwrap();
        let minute_regex = Regex::new(r":(\d+)").unwrap();
        
        if input_regex.is_match(entry) {
            let cap = input_regex.captures(entry).unwrap();
            current_id = cap[1].parse::<i32>().unwrap();
        } else {
            if entry.contains("sleep") {
                begin_sleep = minute_regex.captures(entry).unwrap()[1].parse::<i32>().unwrap();
            } else {
                end_sleep = minute_regex.captures(entry).unwrap()[1].parse::<i32>().unwrap();
                let array = map.entry(current_id).or_insert([0; 60]);
                for i in begin_sleep..end_sleep {
                    array[i as usize] += 1;
                }
            }
            
        }
    }
    let mut sum_map = HashMap::new();

    let mut current_max = 0;
    let mut current_max_id = 0;
    for (id, array) in &map {
        let sum = array.iter().fold(0,|a, &b| a + b);
        sum_map.insert(id, sum);

        if current_max < sum {
            current_max = sum;
            current_max_id = *id;
        }
    }

    let array = map.get(&current_max_id).expect("should exist");
    let mut current_max_minute = 0;
    let mut current_max_minute_sum = 0;
    for i in 0..60 {
        if array[i as usize] > current_max_minute_sum {
            current_max_minute_sum = array[i as usize];
            current_max_minute = i;
        }
    }

    println!("{}", current_max_minute*current_max_id);

    let mut current_most_frequent_minute = 0;
    let mut current_frequence = 0;
    let mut current_id = 0;
    for (id, array) in &map {
        for i in 0..60 {
            if array[i as usize] > current_frequence {
                current_frequence = array[i as usize];
                current_most_frequent_minute = i;
                current_id = *id;
            }
        }
    }

    println!("{}", current_most_frequent_minute*current_id);
   
}
