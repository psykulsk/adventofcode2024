use std::{collections::HashSet, fs, isize};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

const NEIGHS_DIFF_CORDS: [(i8, i8); 4] = [(-1, 0), (1, 0), (0, 1), (0, -1)];

fn get_neighbours((y, x): (usize, usize), max_y: usize, max_x: usize) -> Vec<(usize, usize)> {
    let mut neighs = vec![];
    for (dy, dx) in NEIGHS_DIFF_CORDS {
        if let Some(new_y) = y.checked_add_signed(dy as isize).filter(|ny| *ny < max_y) {
            if let Some(new_x) = x.checked_add_signed(dx as isize).filter(|nx| *nx < max_x) {
                neighs.push((new_y, new_x));
            }
        }
    }
    neighs
}

fn get_reachable_nines(
    (start_y, start_x): (usize, usize),
    map: &Vec<Vec<u32>>,
) -> HashSet<(usize, usize)> {
    let mut reachable_nines: HashSet<(usize, usize)> = HashSet::new();
    let max_x = map.get(0).unwrap().len();
    let max_y = map.len();

    let mut dfs_queue: Vec<(usize, usize)> = vec![(start_y, start_x)];
    while let Some((y, x)) = dfs_queue.pop() {
        let current_height = map[y][x];
        get_neighbours((y, x), max_y, max_x)
            .into_iter()
            .filter(|(ny, nx)| map[*ny][*nx] == current_height + 1)
            .for_each(|neigh| {
                if map[neigh.0][neigh.1] == 9 {
                    reachable_nines.insert(neigh);
                } else {
                    dfs_queue.push(neigh);
                }
            });
    }
    reachable_nines
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut map: Vec<Vec<u32>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for line in input.split('\n') {
        if line.len() != 0 {
            visited.push(vec![false; line.len()]);
            map.push(line.chars().map(|c| c.to_digit(10).unwrap()).collect());
        }
    }

    let max_x = map.get(0).unwrap().len();
    let max_y = map.len();

    let mut score = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            let field = map[y][x];
            if field == 0 {
                score += get_reachable_nines((y, x), &map).len();
            }
        }
    }
    println!("{score}");
}
