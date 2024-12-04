use std::{char, fs, iter::zip};

use regex::Regex;

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

const XMAS: &str = "XMAS";
const SAMX: &str = "SAMX";

fn count_matches(input: &str, to_match: &[Regex]) -> usize {
    let mut output = 0;

    println!("-------------------------");
    for x in to_match {
        output += x.captures_iter(input).count();
    }
    output
}

fn print_matrix(matrix: Vec<Vec<char>>) {
    println!("--------------");
    for line in &matrix {
        let joined = String::from_iter(line);
        println!("{joined}");
    }
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.split('\n') {
        if line.len() != 0 {
            matrix.push(line.chars().collect());
        }
    }

    let to_match = vec![Regex::new(XMAS).unwrap(), Regex::new(SAMX).unwrap()];

    let width = matrix.get(0).unwrap().len();
    let height = matrix.len();

    let mut matches = 0;
    for line in &matrix {
        let line_joined = String::from_iter(line);
        matches += count_matches(&line_joined, &to_match);
    }

    for i in 0..width {
        let column: String = (0..matrix.len()).map(|j| matrix[j][i]).collect();
        matches += count_matches(&column, &to_match);
    }

    for i in 0..height {
        let diag_down_right_bottom_half: String = zip(i..height, 0..(width - i))
            .map(|(y, x)| matrix[y][x])
            .collect();
        matches += count_matches(&diag_down_right_bottom_half, &to_match);
    }

    for i in 1..width {
        let diag_down_right_top_half: String = zip(0..(height - 1), i..width)
            .map(|(y, x)| matrix[y][x])
            .collect();
        matches += count_matches(&diag_down_right_top_half, &to_match);
    }

    for i in (0..height).rev() {
        let diag_up_right_top_half: String = zip((0..(height - i)).rev(), 0..(width - i))
            .map(|(y, x)| matrix[y][x])
            .collect();
        matches += count_matches(&diag_up_right_top_half, &to_match);
    }

    for i in 1..width {
        let diag_up_right_bot_half: String = zip((i..height).rev(), i..width)
            .map(|(y, x)| matrix[y][x])
            .collect();
        matches += count_matches(&diag_up_right_bot_half, &to_match);
    }

    println!("result: {matches}");
}
