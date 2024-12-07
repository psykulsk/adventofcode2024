use std::fs;

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

fn is_possibly_true(target: u64, current_result: u64, numbers: &[u64]) -> bool {
    //println!("{target} {current_result} {numbers:?}");
    if current_result == target && numbers.is_empty() {
        return true;
    } else if current_result > target || numbers.is_empty() {
        return false;
    }
    // part2
    let mut new_number = current_result.to_string();
    new_number.push_str(&numbers[0].to_string());
    let num: u64 = new_number.parse().unwrap();
    is_possibly_true(target, num, &numbers[1..])
        || is_possibly_true(target, current_result * numbers[0], &numbers[1..])
        || is_possibly_true(target, current_result + numbers[0], &numbers[1..])
}

fn main() {
    let input = fs::read_to_string(INPUT_FILE).unwrap();
    let re = regex::Regex::new(r"(:?(?<result>[0-9]+):)|(:? (?<num>[0-9]+))").unwrap();
    let mut sum = 0;
    for line in input.split('\n') {
        if line.len() == 0 {
            continue;
        }
        //println!("---------------------");
        let mut result: Option<u64> = None;
        let mut numbers: Vec<u64> = Vec::new();
        re.captures_iter(line).for_each(|cap| {
            if let Some(res) = cap.name("result") {
                result = Some(res.as_str().parse().unwrap());
            }
            if let Some(num) = cap.name("num") {
                numbers.push(num.as_str().parse().unwrap());
            }
        });
        let result = result.unwrap();
        if is_possibly_true(result, numbers[0], &numbers[1..]) {
            //println!("true for: {result}: {numbers:?}");
            sum += result;
        }
    }
    println!("result: {sum}");
}
