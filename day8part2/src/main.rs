use std::{
    char,
    collections::{HashMap, HashSet},
    fs,
};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

fn bounds_check((y, x): (i32, i32), max_y: i32, max_x: i32) -> bool {
    y >= 0 && y < max_y && x >= 0 && x < max_x
}

fn new_point(start: (i32, i32), base_diff: (i32, i32), mult: i32) -> (i32, i32) {
    (
        start.0 + (mult * base_diff.0),
        start.1 + (mult * base_diff.1),
    )
}

fn generate_all_antinodes(
    start: (i32, i32),
    base_diff: (i32, i32),
    max_y: i32,
    max_x: i32,
    antinodes: &mut HashSet<(i32, i32)>,
) {
    let mut mult = 0;
    while bounds_check(new_point(start, base_diff, mult), max_y, max_x) {
        antinodes.insert(new_point(start, base_diff, mult));
        mult += 1;
    }
    let mut mult = 0;
    while bounds_check(new_point(start, base_diff, mult), max_y, max_x) {
        antinodes.insert(new_point(start, base_diff, mult));
        mult -= 1;
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

    let width: i32 = matrix.get(0).unwrap().len().try_into().unwrap();
    let height: i32 = matrix.len().try_into().unwrap();

    let mut antennas_per_freq: HashMap<char, Vec<(i32, i32)>> = HashMap::new();

    for y in 0..height {
        for x in 0..width {
            let freq = matrix[y as usize][x as usize];
            if freq != '.' {
                antennas_per_freq
                    .entry(freq)
                    .and_modify(|antennas| {
                        antennas.push((y.try_into().unwrap(), x.try_into().unwrap()));
                    })
                    .or_insert(vec![(y.try_into().unwrap(), x.try_into().unwrap())]);
            }
        }
    }

    let mut antinodes: HashSet<(i32, i32)> = HashSet::new();

    for (_, antennas) in antennas_per_freq.into_iter() {
        for i in 0..antennas.len() {
            let current = antennas[i];
            let rest = &antennas[i + 1..];

            for other_antenna in rest {
                let diff = (other_antenna.0 - current.0, other_antenna.1 - current.1);

                generate_all_antinodes(current, diff, height, width, &mut antinodes);
            }
        }
    }
    let result = antinodes.len();
    println!("result: {result}");
}
