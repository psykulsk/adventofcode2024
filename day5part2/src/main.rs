use std::{
    collections::{HashMap, HashSet},
    fs,
};

const INPUT_FILE: &str = "./testinput.txt";
//const INPUT_FILE: &str = "./input.txt";

fn build_topo_sort_order(
    rules: HashMap<u32, Vec<u32>>,
    mut all_nodes: HashSet<u32>,
) -> HashMap<u32, u32> {
    let mut order: HashMap<u32, u32> = HashMap::new();
    let mut in_edges_count: HashMap<u32, u32> = HashMap::new();
    rules.iter().for_each(|(_, child)| {
        child.iter().for_each(|n| {
            in_edges_count
                .entry(*n)
                .and_modify(|count| {
                    *count += 1;
                })
                .or_insert(1);
            all_nodes.remove(n);
        })
    });

    let mut nodes: Vec<u32> = all_nodes.into_iter().collect();
    let mut current_order = 0;
    println!("starting nodes: {nodes:?}");
    while !nodes.is_empty() {
        let mut next_nodes: Vec<u32> = vec![];
        nodes.iter().for_each(|n| {
            order.insert(*n, current_order);
            let children = rules.get(n);
            if let Some(children) = children {
                for child in children {
                    in_edges_count.entry(*child).and_modify(|count| {
                        *count -= 1;
                    });
                    if *in_edges_count.get(child).unwrap() == 0 {
                        next_nodes.push(*child);
                    }
                }
            }
        });
        nodes = next_nodes.clone();
        current_order += 1;
    }
    order
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut graph: HashMap<u32, Vec<u32>> = HashMap::new();
    let mut all_nodes: HashSet<u32> = HashSet::new();
    let mut iter = input.split('\n');
    let mut next = iter.next();

    while next.is_some_and(|l| !l.is_empty()) {
        let line = next.unwrap();
        let split: Vec<&str> = line.split("|").collect();
        let left_num: u32 = split.get(0).unwrap().parse().unwrap();
        let right_num: u32 = split.get(1).unwrap().parse().unwrap();
        all_nodes.insert(left_num);
        graph
            .entry(left_num)
            .and_modify(|e| {
                e.push(right_num);
            })
            .or_insert(vec![right_num]);
        next = iter.next();
    }
    println!("graph: {graph:?}");
    println!("all_nodes: {all_nodes:?}");

    let order = build_topo_sort_order(graph, all_nodes);
    println!("order: {order:?}");

    let mut result = 0;
    let mut next = iter.next();
    while next.is_some_and(|l| !l.is_empty()) {
        let line = next.unwrap();
        next = iter.next();
        let mut numbers: Vec<u32> = line.split(",").map(|i| i.parse().unwrap()).collect();
        if !numbers.is_sorted_by_key(|n| order.get(n).unwrap()) {
            println!("before: {numbers:?}");
            numbers.sort_by_key(|n| order.get(n).unwrap());
            println!("after: {numbers:?}");
            result += numbers.get(numbers.len() / 2).unwrap();
        }
    }

    println!("result: {result}");
}
