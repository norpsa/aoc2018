extern crate regex;

use regex::Regex;

const INPUT_FILE: &str = include_str!("../input.txt");

struct Area {
    id: i32,
    x: i32,
    y: i32,
    w: i32,
    h: i32
}

fn main() {
    let mut map = [[0u8; 1000]; 1000];
    let mut structs = Vec::new();
    for line in INPUT_FILE.lines() {
        let input_regex = Regex::new(r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let cap = input_regex.captures(line).unwrap();
        structs.push(Area { 
            id: cap[1].parse::<i32>().unwrap(),
            x: cap[2].parse::<i32>().unwrap(),
            y: cap[3].parse::<i32>().unwrap(),
            w: cap[4].parse::<i32>().unwrap(),
            h: cap[5].parse::<i32>().unwrap() });
    }

    for area in &structs {
        for i in 0..area.w {
            for j in 0..area.h {
                map[(area.x+i) as usize][(area.y+j) as usize] += 1;
            }
        }
    }

    let mut result_area = 0;
    for i in 0..999 {
        for j in 0..999 {
            if map[i][j] > 1 {
                result_area += 1;
            }
        }
    }

    println!("PART 1 {}", result_area);

    for area in &structs {
        let mut own_area = true;
        for i in 0..area.w {
            for j in 0..area.h {
                if map[(area.x+i) as usize][(area.y+j) as usize] > 1 {
                    own_area = false;
                }
            }
        }
        if own_area {
            println!("PART 2 {}", area.id);
            break;
        }

    }

} 
