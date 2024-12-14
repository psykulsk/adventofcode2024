use std::{
    cmp::{max, min},
    fs, i64,
};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

const A_PRICE: i64 = 3;
const B_PRICE: i64 = 1;

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();

    let re_a = regex::Regex::new(r"Button A: X\+(\d+), Y\+(\d+)").unwrap();
    let re_b = regex::Regex::new(r"Button B: X\+(\d+), Y\+(\d+)").unwrap();
    let re_prize = regex::Regex::new(r"Prize: X=(\d+), Y=(\d+)").unwrap();

    let mut iter = input.split("\n").into_iter();
    let mut total_price = 0;
    while let Some(line) = iter.next() {
        if line.is_empty() {
            continue;
        }
        let cap_a = re_a.captures(line).unwrap();
        let a_x: i64 = cap_a.get(1).unwrap().as_str().parse().unwrap();
        let a_y: i64 = cap_a.get(2).unwrap().as_str().parse().unwrap();
        let line = iter.next().unwrap();
        let cap_b = re_b.captures(line).unwrap();
        let b_x: i64 = cap_b.get(1).unwrap().as_str().parse().unwrap();
        let b_y: i64 = cap_b.get(2).unwrap().as_str().parse().unwrap();
        let line = iter.next().unwrap();
        let cap_prize = re_prize.captures(line).unwrap();
        let prize_x: i64 = cap_prize.get(1).unwrap().as_str().parse().unwrap();
        let prize_x = prize_x + 10000000000000;
        let prize_y: i64 = cap_prize.get(2).unwrap().as_str().parse().unwrap();
        let prize_y = prize_y + 10000000000000;

        let a_top = prize_x * b_y - prize_y * b_x;
        let a_bot = a_x * b_y - a_y * b_x;
        let a = a_top / a_bot;

        if a_top % a_bot != 0 || a < 0 {
            continue;
        }

        let b_top = prize_y - a * a_y;
        let b_bot = b_y;
        let b = b_top / b_bot;
        if b_top % b_bot != 0 || b < 0 {
            continue;
        }
        let price = A_PRICE * a + B_PRICE * b;
        println!("a={a}, b={b}, price = {price}");
        total_price += price;
    }
    println!("{total_price}");
}
