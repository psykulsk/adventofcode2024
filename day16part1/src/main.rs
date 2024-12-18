use std::{
    cmp::min,
    collections::HashSet,
    fs,
    io::{stdin, stdout, Read, Write},
    isize,
    thread::sleep_ms,
};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

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

    fn rot(&self) -> isize {
        match self {
            Self::N => 0,
            Self::S => 180,
            Self::E => 90,
            Self::W => 270,
        }
    }

    fn rot_cost(&self, target: &Dir) -> isize {
        min::<isize>(
            self.rot().abs_diff(target.rot()) as isize,
            ((360 as isize).abs_diff(self.rot()) as isize) + target.rot(),
        ) / 90
            * 1000
    }
}

fn pause() {
    // let mut stdout = stdout();
    // stdout.write(b"Press Enter to continue...").unwrap();
    // stdout.flush().unwrap();
    // stdin().read(&mut [0]).unwrap();
    sleep_ms(50);
}

fn print_map(map: &Vec<Vec<char>>) {
    for row in map {
        for c in row {
            print!("{c}");
        }
        print!("\n");
    }
    print!("\n");
}

fn print_map_with_path(map: &Vec<Vec<char>>, visited: &HashSet<(usize, usize)>) {
    let mut map_clone = map.clone();
    for point in visited {
        map_clone[point.0][point.1] = 'X';
    }
    print_map(&map_clone);
    pause();
}

fn dfs_shortest_path(
    start: &(usize, usize),
    start_dir: &Dir,
    mut visited: HashSet<(usize, usize)>,
    end: &(usize, usize),
    map: &Vec<Vec<char>>,
    current_cost: usize,
) -> Option<usize> {
    //print_map_with_path(map, &visited);
    if start == end {
        return Some(current_cost);
    }
    if visited.contains(start) {
        return None;
    }
    visited.insert(*start);
    let mut min_cost = None;
    for dir in DIRECTIONS {
        let neigh = dir.next(start);
        if map[neigh.0][neigh.1] == '#' {
            continue;
        }
        if let Some(cost) = dfs_shortest_path(
            &neigh,
            &dir,
            visited.clone(),
            end,
            map,
            current_cost + (start_dir.rot_cost(&dir) as usize) + 1,
        ) {
            if let Some(current_min_cost) = min_cost {
                min_cost = Some(min(current_min_cost, cost));
            } else {
                min_cost = Some(cost);
            }
        }
    }

    min_cost
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

    print_map(&map);
    println!("start: {start:?}, end: {end:?}");
    let visited: HashSet<(usize, usize)> = HashSet::new();
    let result = dfs_shortest_path(&start.unwrap(), &Dir::E, visited, &end.unwrap(), &map, 0);
    println!("result: {result:?}");
}
