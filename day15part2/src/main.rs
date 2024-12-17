use core::panic;
use std::{
    collections::HashSet,
    fs,
    io::{stdin, stdout, Read, Write},
};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

enum Command {
    N,
    E,
    S,
    W,
}

impl From<&char> for Command {
    fn from(value: &char) -> Self {
        match value {
            '^' => Self::N,
            '>' => Self::E,
            'v' => Self::S,
            '<' => Self::W,
            _ => panic!("invalid command"),
        }
    }
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

impl Command {
    fn exec(&self, (start_y, start_x): (usize, usize)) -> (usize, usize) {
        match self {
            Command::N => (start_y - 1, start_x),
            Command::E => (start_y, start_x + 1),
            Command::S => (start_y + 1, start_x),
            Command::W => (start_y, start_x - 1),
        }
    }
}

fn get_box_fields(map: &Vec<Vec<char>>, pos: (usize, usize)) -> Option<Vec<(usize, usize)>> {
    let field_val = map[pos.0][pos.1];
    match field_val {
        '[' => Some(vec![pos, (pos.0, pos.1 + 1)]),
        ']' => Some(vec![(pos.0, pos.1 - 1), pos]),
        _ => None,
    }
}

fn gather_shift(
    map: &Vec<Vec<char>>,
    start: &(usize, usize),
    command: &Command,
) -> Result<HashSet<(usize, usize)>, ()> {
    println!("gather_shift start: {start:?}");
    let field = map[start.0][start.1];
    let mut fields_to_shift = HashSet::new();
    if field == '#' {
        return Err(());
    } else if field == '.' {
        return Ok(fields_to_shift);
    }
    if let Some(box_fields) = get_box_fields(map, *start) {
        fields_to_shift.extend(box_fields.clone());
        match command {
            Command::S | Command::N => {
                for field in &box_fields {
                    fields_to_shift.extend(gather_shift(map, &command.exec(*field), command)?);
                }
            }
            Command::E => {
                fields_to_shift.extend(gather_shift(map, &command.exec(box_fields[1]), command)?);
            }
            Command::W => {
                fields_to_shift.extend(gather_shift(map, &command.exec(box_fields[0]), command)?);
            }
        };
    }
    Ok(fields_to_shift)
}

struct Game {
    pub map: Vec<Vec<char>>,
    pub robot: (usize, usize),
}

impl Game {
    fn run(&mut self, command: &Command) {
        let next_pos = command.exec(self.robot);
        let gather_result = gather_shift(&self.map, &next_pos, command);
        if gather_result.is_err() {
            return;
        }
        let mut fields_to_shift = vec![self.robot];
        fields_to_shift.extend(gather_result.unwrap());
        match command {
            Command::S => fields_to_shift.sort_by(|a, b| a.0.cmp(&b.0)),
            Command::N => fields_to_shift.sort_by(|a, b| b.0.cmp(&a.0)),
            Command::E => fields_to_shift.sort_by(|a, b| a.1.cmp(&b.1)),
            Command::W => fields_to_shift.sort_by(|a, b| b.1.cmp(&a.1)),
        }
        //println!("fields_to_shift: {fields_to_shift:?}");
        // shift
        while let Some(to_shift) = fields_to_shift.pop() {
            let to_shift_val = self.map[to_shift.0][to_shift.1];
            let shift_target = command.exec(to_shift);
            self.map[to_shift.0][to_shift.1] = '.';
            self.map[shift_target.0][shift_target.1] = to_shift_val;
        }
        self.robot = next_pos;
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

fn sum_gps_coords(map: &Vec<Vec<char>>) -> usize {
    let mut result = 0;
    for (y, row) in map.iter().enumerate() {
        for (x, c) in row.iter().enumerate() {
            if *c == '[' {
                result += y * 100 + x;
            }
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();

    let mut map: Vec<Vec<char>> = vec![];
    let mut iter = input.split("\n").into_iter();
    while let Some(line) = iter.next() {
        if line.is_empty() {
            break;
        }
        map.push(
            line.chars()
                .map(|c| match c {
                    'O' => vec!['[', ']'],
                    '.' => vec!['.', '.'],
                    '#' => vec!['#', '#'],
                    '@' => vec!['@', '.'],
                    _ => panic!("unexpected char"),
                })
                .flatten()
                .collect(),
        );
    }

    let mut robot = None;
    for (y, row) in map.iter().enumerate() {
        for (x, field) in row.iter().enumerate() {
            if *field == '@' {
                robot = Some((y, x));
                break;
            }
        }
    }

    let mut game = Game {
        map: map.clone(),
        robot: robot.unwrap(),
    };

    let commands: Vec<char> = iter
        .map(&str::chars)
        .flatten()
        .filter(|c| !c.is_whitespace())
        .collect();

    for command in &commands {
        //print_map(&game.map);
        //println!("---------");
        //pause();
        game.run(&Command::from(command));
    }
    print_map(&map);
    println!("---------");
    print_map(&game.map);
    println!("---------");
    let result = sum_gps_coords(&game.map);
    println!("result: {result}");
}
