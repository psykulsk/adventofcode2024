use std::fs;

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

struct Game {
    pub map: Vec<Vec<char>>,
    pub robot: (usize, usize),
}

impl Game {
    fn run(&mut self, command: &Command) {
        let mut next_pos = command.exec(self.robot);
        let mut fields_to_shift = vec![self.robot];
        while self.map[next_pos.0][next_pos.1] == 'O' {
            fields_to_shift.push(next_pos);
            next_pos = command.exec(next_pos);
        }
        let next_field = self.map[next_pos.0][next_pos.1];
        if next_field == '#' {
            return;
        }
        assert_eq!(next_field, '.');
        // shift
        let mut next_shift_pos = next_pos;
        while let Some(to_shift) = fields_to_shift.pop() {
            let to_shift_val = self.map[to_shift.0][to_shift.1];
            self.map[next_shift_pos.0][next_shift_pos.1] = to_shift_val;
            next_shift_pos = to_shift;
        }
        self.map[self.robot.0][self.robot.1] = '.';
        self.robot = command.exec(self.robot);
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
            if *c == 'O' {
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
        map.push(line.chars().collect());
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
        game.run(&Command::from(command));
    }
    print_map(&map);
    println!("---------");
    print_map(&game.map);
    println!("---------");
    let result = sum_gps_coords(&game.map);
    println!("result: {result}");
}
