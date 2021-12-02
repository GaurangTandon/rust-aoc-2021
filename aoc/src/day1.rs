use std::{
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
};

fn read_inputs() -> io_result<Vec<u32>> {
    let input_file = File::open("inputs/1.txt")?; // using try panic macro
    let file_reader = BufReader::new(input_file);
    let inputs = file_reader
        .lines()
        .map(|line| line.unwrap().parse::<u32>().unwrap())
        .collect::<Vec<u32>>();
    return Ok(inputs);
}

fn part1(inputs: &Vec<u32>) {
    let (mut answer, mut previous): (u32, u32) = (0, u32::MAX);
    for val in inputs {
        if *val > previous {
            answer += 1;
        }
        previous = *val;
    }
    println!("Answer: {}", answer);
}

fn part2(inputs: &Vec<u32>) {
    let mut answer = 0;
    // indexed loops in rust are bad as they may not get optimized by O3 compiler
    // for i in 3..inputs.len() {
    //     let previous_window = inputs[i - 3] + inputs[i - 2] + inputs[i - 1];
    //     let this_window = inputs[i - 2] + inputs[i - 1] + inputs[i - 0];
    //     if this_window > previous_window {
    //         answer += 1;
    //     }
    // }
    // hence we prefer to use a for in loop
    if inputs.len() < 3 {
        println!("ZERO");
        return;
    }
    let mut previous_window = inputs[0] + inputs[1] + inputs[2];
    let mut this_window = inputs[1] + inputs[2];
    let mut last_elm = inputs[2];
    // taking a read only slice (https://doc.rust-lang.org/std/vec/struct.Vec.html#slicing)
    for val in &inputs[3..] {
        this_window += val;
        last_elm += val;
        if this_window > previous_window {
            answer += 1;
        }
        previous_window = this_window;
        this_window = last_elm;
        last_elm = *val;
    }

    println!("Answer: {}", answer);
}

// BurntSushi's convention to not return Result in main
// https://blog.burntsushi.net/rust-error-handling/#standard-library-traits-used-for-error-handling
// In the future we need to improve this using argparse
fn main() {
    let inputs = read_inputs().expect("Input read correctly");
    // part1(&inputs);
    part2(&inputs); // take an immutable read reference when passing inputs
}
