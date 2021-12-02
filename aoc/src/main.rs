use std::{
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
};

enum Action {
    F,
    D,
    U,
}

fn read_inputs() -> io_result<Vec<(Action, u32)>> {
    let input_file = File::open("inputs/2.txt")?; // using try panic macro
    let file_reader = BufReader::new(input_file);
    let inputs = file_reader
        .lines()
        .map(|line| {
            let res1 = line.unwrap();
            let mut res = res1.split(" ");
            let (a, b) = (res.next().unwrap(), res.next().unwrap());
            return {
                (
                    {
                        if a == "forward" {
                            Action::F
                        } else {
                            if a == "down" {
                                Action::D
                            } else {
                                Action::U
                            }
                        }
                    },
                    b.parse().unwrap(),
                )
            };
        })
        .collect::<Vec<(Action, u32)>>();
    return Ok(inputs);
}

fn part1(inputs: &Vec<(Action, u32)>) -> u32 {
    let (mut horizontal, mut depth): (u32, u32) = (0, 0);

    for (action, movement) in inputs {
        match action {
            Action::F => {
                horizontal += movement;
            }
            Action::D => {
                depth += movement;
            }
            Action::U => {
                depth -= movement;
            }
        }
    }

    return horizontal * depth;
}

fn part2(inputs: &Vec<(Action, u32)>) -> u32 {
    let (mut horizontal, mut depth, mut aim): (u32, u32, u32) = (0, 0, 0);

    for (action, movement) in inputs {
        match action {
            Action::F => {
                horizontal += movement;
                depth += aim * movement;
            }
            Action::D => {
                aim += movement;
            }
            Action::U => {
                aim -= movement;
            }
        }
    }

    return horizontal * depth;
}

// BurntSushi's convention to not return Result in main
// https://blog.burntsushi.net/rust-error-handling/#standard-library-traits-used-for-error-handling
// In the future we need to improve this using argparse
fn main() {
    let inputs = read_inputs().expect("Input read correctly");
    // let answer = part1(&inputs);
    let answer = part2(&inputs);
    println!("Answer: {}", answer)
}
