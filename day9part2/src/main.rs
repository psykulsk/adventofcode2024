use std::{fs, iter::Peekable};

//const INPUT_FILE: &str = "./testinput.txt";
const INPUT_FILE: &str = "./input.txt";

fn forward_until_file_end(
    file_id: i64,
    initial_file_start: usize,
    memory: &Vec<i64>,
    iter: &mut Peekable<impl Iterator<Item = usize>>,
) -> (usize, usize) {
    let mut len = 1;
    let mut file_start = initial_file_start;
    while let Some(next) = iter.peek() {
        if memory[*next] != file_id {
            break;
        } else {
            len += 1;
            file_start = *next;
            iter.next();
        }
    }
    (file_start, len)
}

fn try_to_fit_in_free_space(
    free_space_blocks: &mut Vec<(usize, usize)>,
    defragmented: &mut Vec<i64>,
    mem_block: i64,
    start_of_file: usize,
    file_size: usize,
) {
    for free_space_block_i in 0..free_space_blocks.len() {
        let (free_space_block_pos, free_space_block_size) = free_space_blocks[free_space_block_i];
        if free_space_block_pos < start_of_file && free_space_block_size >= file_size {
            //println!("start_of_file:{start_of_file}, file_size:{file_size}, file_id:{mem_block}; free_space_block_pos:{free_space_block_pos}, free_space_block_size:{free_space_block_size}");
            for i in 0..file_size {
                defragmented[free_space_block_pos + i] = mem_block;
                defragmented[start_of_file + i] = -1;
            }
            if free_space_block_size > file_size {
                free_space_blocks[free_space_block_i] = (
                    free_space_block_pos + file_size,
                    free_space_block_size - file_size,
                );
            } else {
                free_space_blocks.remove(free_space_block_i);
            }
            break;
        }
    }
}

fn main() {
    let input: Vec<i64> = fs::read_to_string(INPUT_FILE)
        .unwrap()
        .trim_end()
        .chars()
        .into_iter()
        .map(|c| c.to_digit(10).unwrap() as i64)
        .collect();

    let mut expanded: Vec<i64> = Vec::new();
    let mut free_space_indices: Vec<usize> = Vec::new();

    let mut free_space_blocks: Vec<(usize, usize)> = Vec::new();
    input
        .into_iter()
        .fold((false, 0, 0), |acc: (bool, usize, i64), block| {
            let (is_free_space, mut global_pos, mut mem_block_index) = acc;
            if is_free_space {
                free_space_blocks.push((global_pos, block as usize));
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

    let mut iter = (0..expanded.len()).rev().peekable();

    while let Some(next) = iter.next() {
        let mem_block = defragmented[next];
        if mem_block == -1 {
            continue;
        } else {
            //println!("expanded:     {expanded:?}");
            //println!("defragmented: {defragmented:?}");
            let (start_of_file, file_size) =
                forward_until_file_end(mem_block, next, &defragmented, &mut iter);
            try_to_fit_in_free_space(
                &mut free_space_blocks,
                &mut defragmented,
                mem_block,
                start_of_file,
                file_size,
            );
        }
    }

    //println!("expanded:     {expanded:?}");
    //println!("defragmented: {defragmented:?}");
    let mut checksum = 0;
    //checksum
    for (pos, block) in defragmented.into_iter().enumerate() {
        if block != -1 {
            checksum += pos * (block as usize);
        }
    }
    println!("checksum: {checksum}");
}
