use regex::Regex;
use std::fs;

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

const NUM1_MATCH: &str = "num1";
const NUM2_MATCH: &str = "num2";
const DONT_MATCH: &str = "dont";
const DO_MATCH: &str = "do";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let re =
        Regex::new(r"(?:mul\((?<num1>[0-9]+),(?<num2>[0-9]+)\))|(?<dont>don't\(\))|(?<do>do\(\))")
            .unwrap();
    let (result, _) = re
        .captures_iter(&input)
        .fold((0, true), |(acc, enabled), cap| {
            if cap.name(DONT_MATCH).is_some() {
                (acc, false)
            } else if cap.name(DO_MATCH).is_some() {
                (acc, true)
            } else if enabled {
                let num1: i64 = cap.name(NUM1_MATCH).unwrap().as_str().parse().unwrap();
                let num2: i64 = cap.name(NUM2_MATCH).unwrap().as_str().parse().unwrap();
                (acc + num1 * num2, enabled)
            } else {
                (acc, enabled)
            }
        });

    println!("result: {result}");
}
