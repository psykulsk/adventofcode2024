use core::panic;
use std::{collections::HashMap, fs, isize};

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

#[derive(Hash, PartialEq, Eq, Debug)]
enum Dir {
    N,
    S,
    E,
    W,
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
    let mut sides: HashMap<(usize, Dir), Vec<usize>> = HashMap::new();

    while let Some((y, x)) = dfs_queue.pop() {
        if visited[y][x] {
            continue;
        }
        region_size += 1;
        visited[y][x] = true;
        if y == max_y - 1 {
            sides
                .entry((y, Dir::S))
                .and_modify(|sides_dim| sides_dim.push(x))
                .or_insert(vec![x]);
        };
        if y == 0 {
            sides
                .entry((y, Dir::N))
                .and_modify(|sides_dim| sides_dim.push(x))
                .or_insert(vec![x]);
        };
        if x == max_x - 1 {
            sides
                .entry((x, Dir::E))
                .and_modify(|sides_dim| sides_dim.push(y))
                .or_insert(vec![y]);
        };
        if x == 0 {
            sides
                .entry((x, Dir::W))
                .and_modify(|sides_dim| sides_dim.push(y))
                .or_insert(vec![y]);
        };
        get_neighbours((y, x), max_y, max_x)
            .into_iter()
            .for_each(|neigh| {
                if map[neigh.0][neigh.1] != region_sign {
                    let neigh_dir: Dir;
                    let side_dim: usize;
                    let other_dim: usize;
                    if neigh.0 > y {
                        neigh_dir = Dir::S;
                        side_dim = y;
                        other_dim = x;
                    } else if neigh.0 < y {
                        neigh_dir = Dir::N;
                        side_dim = y;
                        other_dim = x;
                    } else if neigh.1 > x {
                        neigh_dir = Dir::E;
                        side_dim = x;
                        other_dim = y;
                    } else if neigh.1 < x {
                        neigh_dir = Dir::W;
                        side_dim = x;
                        other_dim = y;
                    } else {
                        panic!("wrong neighbor");
                    }
                    sides
                        .entry((side_dim, neigh_dir))
                        .and_modify(|sides_dim| sides_dim.push(other_dim))
                        .or_insert(vec![other_dim]);
                } else {
                    dfs_queue.push(neigh);
                }
            });
        //println!("(y, x): ({y},{x}), sign: {region_sign}, region_size: {region_size}, fence_len: {fence_len}");
    }
    //println!("sign: {region_sign}, sides: {sides:?}");
    let nr_of_sides = sides
        .into_iter()
        .fold(0, |acc, ((side_dim, dir), other_dim)| {
            let mut local_sides = 1;
            let mut other_dim = other_dim.clone();
            other_dim.sort();
            let mut prev = other_dim[0];
            for i in 1..other_dim.len() {
                let cur = other_dim[i];
                if cur.abs_diff(prev) > 1 {
                    local_sides += 1;
                    //println!(
                    //    "found diff! region {region_sign}, dir: {dir:?}, side_dim: {side_dim}"
                    //);
                }
                prev = cur;
            }
            //println!(
            //    "region {region_sign}, dir: {dir:?}, side_dim: {side_dim}, adding local_sides: {local_sides}"
            //);
            acc + local_sides
        });
    let region_price = region_size * nr_of_sides;
    //println!("region {region_sign}, price: {region_size}*{nr_of_sides}={nr_of_sides}");
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
