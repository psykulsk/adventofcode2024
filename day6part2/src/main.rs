use std::{char, collections::HashSet, fs};

//const INPUT_FILE: &str = "./testinput.txt";
//const INPUT_FILE: &str = "./testinput2.txt";
const INPUT_FILE: &str = "./input.txt";

const GUARD: char = '^';
const OBSTACLE: char = '#';

#[derive(Clone, Debug, Copy, Hash, Eq, PartialEq)]
enum Dir {
    N,
    S,
    W,
    E,
}

impl Dir {
    fn turn_right(&self) -> Dir {
        match self {
            Dir::N => Dir::E,
            Dir::S => Dir::W,
            Dir::E => Dir::S,
            Dir::W => Dir::N,
        }
    }
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
    let (next_y, next_x) = match dir {
        Dir::N => (y - 1, *x),
        Dir::S => (y + 1, *x),
        Dir::E => (*y, x + 1),
        Dir::W => (*y, x - 1),
    };
    if !bounds_check((next_y, next_x), height, width) {
        return None;
    }

    if map[next_y as usize][next_x as usize] == OBSTACLE {
        return Some(((*y, *x), dir.turn_right()));
    } else {
        return Some(((next_y, next_x), dir.clone()));
    }
}

fn check_if_putting_obstacle_in_front_creates_loop(
    current_pos: &Position,
    map: &Map,
    guard_path_pos: &HashSet<(i32, i32)>,
    guard_path: &HashSet<Position>,
    obstacle_positions: &mut HashSet<(i32, i32)>,
) {
    // if the next move is in the same direction,
    // then try putting an obstacle there instead and check whether
    // we run into any of the previously visited
    let mut guard_path_extended = guard_path.clone();
    if let Some(next) = step(&map, current_pos) {
        if next.1 == current_pos.1 {
            let obstacle_position = next.0;
            if !guard_path_pos.contains(&next.0) {
                // turn right
                let mut loop_test_pos = (current_pos.0, current_pos.1.turn_right());
                guard_path_extended.insert(loop_test_pos);
                while let Some(pos) = step(&map, &loop_test_pos) {
                    if guard_path_extended.contains(&pos) {
                        obstacle_positions.insert(obstacle_position);
                        return;
                    }
                    guard_path_extended.insert(pos);
                    loop_test_pos = pos;
                }
            }
        }
    }
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

    let mut obstacle_positions: HashSet<(i32, i32)> = HashSet::new();
    let mut guard_path_pos: HashSet<(i32, i32)> = HashSet::new();
    let mut guard_path: HashSet<Position> = HashSet::new();
    let mut current_position = guard_position.unwrap();
    guard_path.insert(current_position);
    guard_path_pos.insert(current_position.0);
    while let Some(pos) = step(&map, &current_position) {
        guard_path_pos.insert(pos.0);
        guard_path.insert(pos);
        check_if_putting_obstacle_in_front_creates_loop(
            &pos,
            &map,
            &guard_path_pos,
            &guard_path,
            &mut obstacle_positions,
        );
        current_position = pos;
    }

    let count = obstacle_positions.len();
    println!("result: {count}");
}
