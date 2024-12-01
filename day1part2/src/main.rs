use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let mut left_list = vec![];
    let mut right_list = vec![];
    for line in input.split("\n") {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        if numbers.len() != 2 {
            continue;
        }
        let left_num: i64 = numbers[0].parse().unwrap();
        left_list.push(left_num);
        let right_num: i64 = numbers[1].parse().unwrap();
        right_list.push(right_num);
    }
    left_list.sort();
    right_list.sort();

    let mut right_pointer = 0;

    let mut sim_score = 0;

    let mut left_score_map = HashMap::new();

    for left in &left_list {
        if let Some(cached_left_score) = left_score_map.get(left) {
            sim_score += cached_left_score;
            continue;
        }
        while right_list
            .get(right_pointer)
            .is_some_and(|right| right < left)
        {
            right_pointer += 1;
        }

        if right_pointer >= right_list.len() {
            break;
        }

        let mut mult = 0;
        while right_list
            .get(right_pointer)
            .is_some_and(|right| right == left)
        {
            mult += 1;
            right_pointer += 1;
        }

        let cur_score = left * mult;
        sim_score += cur_score;
        left_score_map.insert(left, cur_score);
    }

    println!("sim_score: {sim_score}");
}
