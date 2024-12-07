use std::{char, fs};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

const GUARD: char = '^';
const OBSTACLE: char = '#';

#[derive(Clone, Debug, Copy)]
enum Dir {
    N,
    S,
    W,
    E,
}

type Position = ((i32, i32), Dir);
type Map = Vec<Vec<char>>;

fn bounds_check((y, x): (i32, i32), max_y: i32, max_x: i32) -> bool {
    y >= 0 && y < max_y && x >= 0 && x < max_x
}

fn step(map: &Map, pos: &Position) -> Option<Position> {
    let width: i32 = map.get(0).unwrap().len().try_into().unwrap();
    let height: i32 = map.len().try_into().unwrap();
    let ((y, x), dir) = pos;
    let ((next_y, next_x), next_dir_right_turn) = match dir {
        Dir::N => ((y - 1, *x), Dir::E),
        Dir::S => ((y + 1, *x), Dir::W),
        Dir::E => ((*y, x + 1), Dir::S),
        Dir::W => ((*y, x - 1), Dir::N),
    };
    if !bounds_check((next_y, next_x), height, width) {
        return None;
    }

    if map[next_y as usize][next_x as usize] == OBSTACLE {
        return Some(((*y, *x), next_dir_right_turn));
    } else {
        return Some(((next_y, next_x), dir.clone()));
    }
}

fn was_visited_and_mark((y, x): &(i32, i32), visited: &mut Vec<Vec<bool>>) -> bool {
    let value = visited[usize::try_from(*y).unwrap()][usize::try_from(*x).unwrap()];
    visited[usize::try_from(*y).unwrap()][usize::try_from(*x).unwrap()] = true;
    value
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let mut map: Map = Vec::new();
    let mut visited: Vec<Vec<bool>> = Vec::new();
    for line in input.split('\n') {
        if line.len() != 0 {
            visited.push(vec![false; line.len()]);
            map.push(line.chars().collect());
        }
    }

    let width = map.get(0).unwrap().len();
    let height = map.len();

    let mut guard_position: Option<Position> = None;
    for y in 0..height {
        for x in 0..width {
            if map[y][x] == GUARD {
                guard_position = Some(((y.try_into().unwrap(), x.try_into().unwrap()), Dir::N));
            }
        }
    }

    let mut count = 1;

    while let Some(pos) = guard_position {
        if let Some(new_pos) = step(&map, &pos) {
            if !was_visited_and_mark(&new_pos.0, &mut visited) {
                count += 1;
            }
            guard_position = Some(new_pos);
        } else {
            break;
        }
    }

    println!("result: {count:?}");
}
