use std::{char, fs};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

const MAS: &str = "MAS";
const SAM: &str = "SAM";
const WINDOW_DIM: usize = 3;

fn xmas_search(matrix: &[Vec<char>], (top_left_y, top_left_x): (usize, usize)) -> bool {
    let diag1: String = (0..WINDOW_DIM)
        .map(|i| matrix[top_left_y + i][top_left_x + i])
        .collect();
    let diag2: String = (0..WINDOW_DIM)
        .map(|i| matrix[top_left_y + WINDOW_DIM - 1 - i][top_left_x + i])
        .collect();
    println!("diag1: {diag1}, diag2: {diag2}");

    (diag1.eq(MAS) || diag1.eq(SAM)) && (diag2.eq(MAS) || diag2.eq(SAM))
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut matrix: Vec<Vec<char>> = Vec::new();
    for line in input.split('\n') {
        if line.len() != 0 {
            matrix.push(line.chars().collect());
        }
    }

    let width = matrix.get(0).unwrap().len();
    let height = matrix.len();

    let mut matches = 0;
    for y in 0..(height - WINDOW_DIM + 1) {
        for x in 0..(width - WINDOW_DIM + 1) {
            if xmas_search(&matrix, (y, x)) {
                matches += 1;
            }
        }
    }

    println!("result: {matches}");
}
