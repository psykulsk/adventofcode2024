use std::{collections::HashMap, fs};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

fn dfs(memo: &mut HashMap<(usize, usize), usize>, num: usize, steps: usize) -> usize {
    if steps == 0 {
        return 1;
    }

    if let Some(memo_result) = memo.get(&(num, steps)) {
        return *memo_result;
    }

    let result;
    if num == 0 {
        result = dfs(memo, 1, steps - 1);
    } else if num.to_string().len() % 2 == 0 {
        let num_string = num.to_string();
        let left_num = num_string[0..num_string.len() / 2].parse().unwrap();
        let right_num = num_string[num_string.len() / 2..].parse().unwrap();
        result = dfs(memo, left_num, steps - 1) + dfs(memo, right_num, steps - 1);
    } else {
        result = dfs(memo, num * 2024, steps - 1);
    }
    memo.insert((num, steps), result);
    result
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let split: Vec<_> = input.trim().split(" ").collect();
    let nums: Vec<usize> = split.into_iter().map(|num| num.parse().unwrap()).collect();
    let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
    let mut result = 0;
    for num in nums {
        result += dfs(&mut memo, num, 75);
    }
    println!("result: {result}");
}
