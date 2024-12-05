use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    fs,
};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

fn compare_with_rules(rules: &HashMap<u32, HashSet<u32>>, first: &u32, second: &u32) -> bool {
    let res = rules.get(first);
    if let Some(children) = res {
        children.contains(second)
    } else {
        false
    }
}

fn ordering_with_rules(rules: &HashMap<u32, HashSet<u32>>, first: &u32, second: &u32) -> Ordering {
    if compare_with_rules(rules, first, second) {
        Ordering::Less
    } else {
        Ordering::Greater
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut graph: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut iter = input.split('\n');
    let mut next = iter.next();

    while next.is_some_and(|l| !l.is_empty()) {
        let line = next.unwrap();
        let split: Vec<&str> = line.split("|").collect();
        let left_num: u32 = split.get(0).unwrap().parse().unwrap();
        let right_num: u32 = split.get(1).unwrap().parse().unwrap();
        graph
            .entry(left_num)
            .and_modify(|e| {
                e.insert(right_num);
            })
            .or_insert(HashSet::from([right_num]));
        next = iter.next();
    }
    println!("graph: {graph:?}");

    let mut result = 0;
    let mut next = iter.next();
    while next.is_some_and(|l| !l.is_empty()) {
        let line = next.unwrap();
        next = iter.next();
        let mut numbers: Vec<u32> = line.split(",").map(|i| i.parse().unwrap()).collect();
        if !numbers.is_sorted_by(|a, b| compare_with_rules(&graph, a, b)) {
            println!("before: {numbers:?}");
            numbers.sort_by(|a, b| ordering_with_rules(&graph, a, b));
            println!("after: {numbers:?}");
            result += numbers.get(numbers.len() / 2).unwrap();
        }
    }

    println!("result: {result}");
}
