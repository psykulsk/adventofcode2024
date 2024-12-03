use regex::Regex;
use std::fs;

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let re = Regex::new(r"mul\(([0-9]+),([0-9]+)\)").unwrap();
    let result = re.captures_iter(&input).fold(0, |acc, cap| {
        let (_, vals): (&str, [&str; 2]) = cap.extract();
        let num1: i64 = vals.get(0).unwrap().parse().unwrap();
        let num2: i64 = vals.get(1).unwrap().parse().unwrap();
        acc + num1 * num2
    });

    println!("result: {result}");
}
