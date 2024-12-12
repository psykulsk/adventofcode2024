use std::{fs, isize};

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

fn get_region_fence_price(
    (start_y, start_x): (usize, usize),
    map: &Vec<Vec<char>>,
    visited: &mut Vec<Vec<bool>>,
) -> usize {
    let max_x = map.get(0).unwrap().len();
    let max_y = map.len();

    let mut dfs_queue: Vec<(usize, usize)> = vec![(start_y, start_x)];
    let region_sign = map[start_y][start_x];
    let mut region_size = 0;
    let mut fence_len = 0;
    while let Some((y, x)) = dfs_queue.pop() {
        if visited[y][x] {
            continue;
        }
        region_size += 1;
        visited[y][x] = true;
        if y == max_y - 1 {
            fence_len += 1;
        };
        if y == 0 {
            fence_len += 1;
        };
        if x == max_x - 1 {
            fence_len += 1;
        };
        if x == 0 {
            fence_len += 1;
        };
        get_neighbours((y, x), max_y, max_x)
            .into_iter()
            .for_each(|neigh| {
                if map[neigh.0][neigh.1] != region_sign {
                    fence_len += 1;
                } else {
                    dfs_queue.push(neigh);
                }
            });
        println!("(y, x): ({y},{x}), sign: {region_sign}, region_size: {region_size}, fence_len: {fence_len}");
    }
    let region_price = region_size * fence_len;
    println!("region {region_sign}, price: {region_size}*{fence_len}={region_price}");
    region_price
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut map: Vec<Vec<char>> = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for line in input.split('\n') {
        if line.len() != 0 {
            visited.push(vec![false; line.len()]);
            map.push(line.chars().collect());
        }
    }

    let max_x = map.get(0).unwrap().len();
    let max_y = map.len();

    let mut total_price = 0;
    for y in 0..max_y {
        for x in 0..max_x {
            if !visited[y][x] {
                total_price += get_region_fence_price((y, x), &map, &mut visited);
            }
        }
    }
    println!("{total_price}");
}
