use std::{
    cmp::min,
    collections::{BTreeSet, HashMap, HashSet},
    fs,
};

const INPUT_FILE: &str = "./testinput.txt";
//const INPUT_FILE: &str = "./input.txt";

#[derive(Debug, Hash, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
enum Dir {
    N,
    E,
    S,
    W,
}

const DIRECTIONS: [Dir; 4] = [Dir::N, Dir::E, Dir::S, Dir::W];

impl Dir {
    fn next(&self, start: &(usize, usize)) -> (usize, usize) {
        match self {
            Self::N => (start.0 - 1, start.1),
            Self::S => (start.0 + 1, start.1),
            Self::E => (start.0, start.1 + 1),
            Self::W => (start.0, start.1 - 1),
        }
    }

    fn get_left_and_right(&self) -> [Dir; 2] {
        match self {
            Self::N => [Dir::W, Dir::E],
            Self::S => [Dir::E, Dir::W],
            Self::W => [Dir::S, Dir::N],
            Self::E => [Dir::N, Dir::S],
        }
    }
}

fn djikstra(
    map: &Vec<Vec<char>>,
    start: &(usize, usize),
    target: &(usize, usize),
) -> (
    HashMap<((usize, usize), Dir), Vec<((usize, usize), Dir)>>,
    Vec<((usize, usize), Dir)>,
) {
    let mut vertices: BTreeSet<(usize, (usize, usize), Dir)> = BTreeSet::new();
    vertices.insert((0, *start, Dir::E));

    let mut distances: HashMap<((usize, usize), Dir), usize> =
        HashMap::from([((*start, Dir::E), 0)]);

    let mut prev: HashMap<((usize, usize), Dir), Vec<((usize, usize), Dir)>> = HashMap::new();

    while let Some(vertex) = vertices.pop_first() {
        let current_vertex_position = vertex.1;
        let current_vertex_direction = vertex.2;
        let current_vertex_min_cost = distances
            .get(&(current_vertex_position, current_vertex_direction))
            .unwrap()
            .clone();
        let mut possible_dirs = current_vertex_direction.get_left_and_right().to_vec();
        possible_dirs.push(current_vertex_direction);

        for next_move_dir in possible_dirs {
            let mut cost = 1;
            if next_move_dir != current_vertex_direction {
                cost += 1000;
            }
            let next_position = next_move_dir.next(&current_vertex_position);
            if map[next_position.0][next_position.1] == '#' {
                continue;
            }
            let current_next_position_cost = distances
                .get(&(next_position, next_move_dir))
                .unwrap_or(&usize::MAX)
                .clone();
            let new_cost = current_vertex_min_cost + cost;
            if new_cost < current_next_position_cost {
                distances.insert(((next_position), next_move_dir), new_cost);
                vertices.remove(&(current_next_position_cost, next_position, next_move_dir));
                vertices.insert((new_cost, next_position, next_move_dir));
                prev.insert(
                    (next_position, next_move_dir),
                    vec![(current_vertex_position, current_vertex_direction)],
                );
            } else if new_cost == current_next_position_cost {
                prev.entry((next_position, next_move_dir))
                    .and_modify(|prev_positions| {
                        prev_positions.push((current_vertex_position, current_vertex_direction));
                    });
            }
        }
    }
    let mut min_target_dist: usize = usize::MAX;
    for dir in DIRECTIONS {
        min_target_dist = *min(
            distances.get(&(*target, dir)).unwrap_or(&usize::MAX),
            &min_target_dist,
        );
    }

    let mut best_path_target_starting_dirs = Vec::new();
    for dir in DIRECTIONS {
        let dist = distances.get(&(*target, dir)).unwrap_or(&usize::MAX);
        if min_target_dist == *dist {
            best_path_target_starting_dirs.push((*target, dir));
        }
    }
    (prev, best_path_target_starting_dirs)
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();

    let mut map: Vec<Vec<char>> = vec![];
    let mut iter = input.split("\n").into_iter();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        map.push(line.chars().collect());
    }

    let mut start = None;
    let mut end = None;
    for (y, row) in map.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            if *field == 'S' {
                start = Some((y, x));
            } else if *field == 'E' {
                end = Some((y, x));
            }
        }
    }

    println!("start: {start:?}, end: {end:?}");
    let (prev, best_path_target_start_points) = djikstra(&map, &start.unwrap(), &end.unwrap());
    let mut queue: Vec<((usize, usize), Dir)> = best_path_target_start_points.clone();

    let mut best_path_tiles = HashSet::new();
    while let Some(node) = queue.pop() {
        queue.extend(prev.get(&node).unwrap_or(&vec![]));
        best_path_tiles.insert(node.0);
    }
    //println!("prev: {prev:?}");
    println!("best_path_tiles: {best_path_tiles:?}");
    let result = best_path_tiles.len();
    println!("result: {result:?}");
}
