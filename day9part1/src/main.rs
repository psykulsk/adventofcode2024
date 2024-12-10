use std::fs;

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

fn main() {
    let input: Vec<i64> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .trim_end()
        .chars()
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut expanded: Vec<i64> = Vec::new();
    let mut free_space_indices: Vec<i64> = Vec::new();

    input
        .into_iter()
        .fold((false, 0, 0), |acc: (bool, i64, i64), block| {
            let (is_free_space, mut global_pos, mut mem_block_index) = acc;
            if is_free_space {
                (0..block).into_iter().for_each(|_| {
                    expanded.push(-1);
                    free_space_indices.push(global_pos);
                    global_pos += 1;
                });
            } else {
                (0..block).into_iter().for_each(|_| {
                    expanded.push(mem_block_index);
                    global_pos += 1;
                });
                mem_block_index += 1;
            }
            (!is_free_space, global_pos, mem_block_index)
        });
    let mut defragmented = expanded.clone();

    let mut free_space_indices: Vec<i64> = free_space_indices.into_iter().rev().collect();

    for i in (0..expanded.len()).rev() {
        let mem_block = expanded[i];
        if mem_block == -1 {
            continue;
        }
        if let Some(free) = free_space_indices.last() {
            if (*free as usize) < i {
                defragmented[*free as usize] = mem_block;
                defragmented[i] = -1;
                free_space_indices.pop();
            }
        }
    }
    let mut checksum = 0;
    //checksum
    for (pos, block) in defragmented.into_iter().enumerate() {
        if block != -1 {
            checksum += pos * (block as usize);
        }
    }
    println!("checksum: {checksum}");
}
