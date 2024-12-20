use std::{
    cmp::min,
    collections::{BinaryHeap, HashMap, HashSet},
    fs,
};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

#[derive(Debug, Hash, PartialEq, PartialOrd, Eq, Clone, Copy)]
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

    fn rot(&self) -> usize {
        match self {
            Self::N => 0,
            Self::S => 180,
            Self::E => 90,
            Self::W => 270,
        }
    }

    fn rot_cost(&self, target: &Dir) -> usize {
        min::<usize>(
            self.rot().abs_diff(target.rot()),
            (360 as usize).abs_diff(self.rot()) + target.rot(),
        ) / 90
            * 1000
    }
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
}

#[derive(Debug, Eq, PartialEq, PartialOrd)]
struct VertexWithMinDistance {
    pub dist: usize,
    pub pos: (usize, usize),
    pub dir: Dir,
}

impl Ord for VertexWithMinDistance {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.dist.cmp(&self.dist)
    }
}

fn djikstra(map: &Vec<Vec<char>>, start: &(usize, usize), target: &(usize, usize)) -> usize {
    let mut vertices: BinaryHeap<VertexWithMinDistance> = BinaryHeap::new();
    vertices.push(VertexWithMinDistance {
        pos: start.clone(),
        dist: 0,
        dir: Dir::E,
    });

    let mut distances: HashMap<((usize, usize), Dir), usize> =
        HashMap::from([((start.clone(), Dir::E), 0)]);

    while let Some(vertex) = vertices.pop() {
        let vertex_min_dist = distances.get(&((vertex.pos), vertex.dir)).unwrap().clone();
        if vertex_min_dist < vertex.dist {
            continue;
        }
        for dir in DIRECTIONS {
            let next = dir.next(&vertex.pos);
            let next_dist = vertex_min_dist + vertex.dir.rot_cost(&dir) + 1;
            let next_field_value = map[next.0][next.1];
            if next_field_value == '#' {
                continue;
            }
            if next_dist < *distances.get(&((next), dir)).unwrap_or(&usize::MAX) {
                distances.insert(((next), dir), next_dist);
                vertices.push(VertexWithMinDistance {
                    dist: next_dist,
                    pos: next,
                    dir,
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
    min_target_dist
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
    let result = djikstra(&map, &start.unwrap(), &end.unwrap());
    println!("result: {result:?}");
}
