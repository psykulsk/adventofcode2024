use std::{
    collections::{HashMap, HashSet},
    fs,
};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut in_rules_section = true;
    let mut rules: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut result = 0;
    for line in input.split('\n') {
        if line.is_empty() {
            in_rules_section = false;
            continue;
        }

        if in_rules_section {
            let split: Vec<&str> = line.split("|").collect();
            let left_num: u32 = split.get(0).unwrap().parse().unwrap();
            let right_num: u32 = split.get(1).unwrap().parse().unwrap();
            rules
                .entry(left_num)
                .and_modify(|e| {
                    e.insert(right_num);
                })
                .or_insert(HashSet::from([right_num]));
        } else {
            let numbers: Vec<u32> = line.split(",").map(|n| n.parse().unwrap()).collect();
            let mut numbers_to_left = vec![];
            let mut rule_broken = false;
            for num in &numbers {
                let broken_rule = numbers_to_left.iter().find_map(|num_to_left| {
                    let numbers_required_to_be_after_num_option = rules.get(&num);
                    if let Some(numbers_required_to_be_after_num) =
                        numbers_required_to_be_after_num_option
                    {
                        numbers_required_to_be_after_num.get(num_to_left)
                    } else {
                        None
                    }
                });
                if let Some(number_breaking_rule) = broken_rule {
                    //println!("in line: {line}; number_breaking_rule: {number_breaking_rule} was to left of num: {num}");
                    rule_broken = true;
                }
                numbers_to_left.push(*num);
            }
            if !rule_broken {
                let mid_number = numbers.get(numbers.len() / 2).unwrap();
                //println!("correct line: {line}; mid number: {mid_number}");
                result += mid_number;
            }
        }
    }

    println!("result: {result}");
}
