use std::{cmp::min, fs, i64};

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

        let mut lowest_price: Option<i64> = None;
        let mut a_presses = 0;
        loop {
            let x_left = prize_x - a_presses * a_x;
            let y_left = prize_y - a_presses * a_y;
            let b_x_mod = x_left % b_x;
            let b_y_mod = y_left % b_y;
            if b_y_mod == 0 && b_x_mod == 0 {
                let b_presses = x_left / b_x;
                let price = a_presses * A_PRICE + b_presses * B_PRICE;
                lowest_price = Some(min(price, lowest_price.unwrap_or(i64::MAX)))
            } else if x_left < 0 || y_left < 0 {
                break;
            }
            a_presses += 1;
        }
        total_price += lowest_price.unwrap_or(0);
    }
    println!("{total_price}");
}
