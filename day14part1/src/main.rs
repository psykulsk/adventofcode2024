use std::process::exit;
use std::{collections::HashMap, fs, i64, process::Command, thread::sleep_ms};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

//const HEIGHT: usize = 7;
//const WIDTH: usize = 11;
const HEIGHT: usize = 103;
const WIDTH: usize = 101;
const STEPS: usize = 100;

fn empty_map() -> Vec<Vec<char>> {
    let mut map = vec![];
    for _ in 0..HEIGHT {
        map.push(vec!['.'; WIDTH]);
    }
    map
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

fn is_map_christmas_tree(map: &Vec<Vec<char>>, step: usize) -> bool {
    for y in 0..HEIGHT {
        for i in 0..WIDTH - 10 {
            let mut only_ones = true;
            for j in i..i + 10 {
                if map[y][j] != '1' {
                    only_ones = false;
                }
            }
            if only_ones {
                println!("step:{step}");
                print_map(map);
                exit(0);
            }
        }
    }
    return false;
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();

    let re = regex::Regex::new(r"p=([-]?\d+),([-]?\d+) v=([-]?\d+),([-]?\d+)").unwrap();

    let mut iter = input.split("\n").into_iter();
    //let mut q1 = 0;
    //let mut q2 = 0;
    //let mut q3 = 0;
    //let mut q4 = 0;
    let mut robots: Vec<_> = vec![];
    while let Some(line) = iter.next() {
        if line.is_empty() {
            continue;
        }
        let cap = re.captures(line).unwrap();

        let p_x: i32 = cap.get(1).unwrap().as_str().parse().unwrap();
        let p_y: i32 = cap.get(2).unwrap().as_str().parse().unwrap();
        let v_x: i32 = cap.get(3).unwrap().as_str().parse().unwrap();
        let v_y: i32 = cap.get(4).unwrap().as_str().parse().unwrap();

        robots.push(((p_x, p_y), (v_x, v_y)));

        //println!("({after_x},{after_y})");
    }

    for step in 0..STEPS * 100000000000 {
        let mut map = empty_map();
        for ((p_x, p_y), (v_x, v_y)) in &robots {
            let mut after_x = p_x + v_x.signum() * (v_x.abs() * (step as i32)) % (WIDTH as i32);

            if after_x >= (WIDTH as i32) {
                after_x -= WIDTH as i32;
            } else if after_x < 0 {
                after_x += WIDTH as i32;
            }
            let mut after_y = p_y + v_y.signum() * (v_y.abs() * (step as i32)) % (HEIGHT as i32);
            if after_y >= HEIGHT as i32 {
                after_y -= HEIGHT as i32;
            } else if after_y < 0 {
                after_y += HEIGHT as i32;
            }
            map[after_y as usize][after_x as usize] = '1';

            //if after_x > (WIDTH as i32) / 2 {
            //    if after_y > (HEIGHT as i32) / 2 {
            //        q3 += 1;
            //    } else if after_y < (HEIGHT as i32) / 2 {
            //        q2 += 1;
            //    }
            //} else if after_x < (WIDTH as i32) / 2 {
            //    if after_y > (HEIGHT as i32) / 2 {
            //        q4 += 1;
            //    } else if after_y < (HEIGHT as i32) / 2 {
            //        q1 += 1;
            //    }
            //}
        }
        //println!("step:{step}  -----------------------");
        //print_map(&map);
        //pause();
        is_map_christmas_tree(&map, step);
    }
}
