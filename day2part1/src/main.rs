use std::fs;

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

const MIN_DIFF: i64 = 1;
const MAX_DIFF: i64 = 3;

fn is_safe(nums: &[i64]) -> bool {
    let first_diff = nums[0] - nums[1];
    if first_diff.abs() < MIN_DIFF || first_diff.abs() > MAX_DIFF {
        return false;
    }
    let mut prev_num = nums.get(0).unwrap();
    for num in &nums[1..] {
        let diff = prev_num - num;
        if diff.signum() != first_diff.signum() || diff.abs() < MIN_DIFF || diff.abs() > MAX_DIFF {
            return false;
        }
        prev_num = num;
    }
    true
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut safe_count = 0;
    for line in input.split("\n") {
        let numbers: Vec<&str> = line.split_whitespace().collect();
        println!("numbers: {numbers:?}");
        let nums: Vec<i64> = numbers
            .into_iter()
            .map(|num| num.parse().unwrap())
            .collect();
        if nums.len() > 0 && is_safe(&nums) {
            safe_count += 1;
        }
    }
    println!("result: {safe_count}");
}
