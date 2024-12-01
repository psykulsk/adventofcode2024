use std::{fs, iter::zip, num};

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
    let mut diff = 0;
    for (left, right) in zip(left_list, right_list) {
        diff += left.abs_diff(right);
    }
    println!("diff: {diff}");
}
