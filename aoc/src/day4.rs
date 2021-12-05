use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
    iter::Iterator,
};

// We don't need this macro when using match arms
// But when using direct equality operator, we do need it
#[derive(PartialEq)]
enum FileReadState {
    Actions,
    Grids,
}

struct FileReadIterator {
    buf_reader: BufReader<File>,
    state: FileReadState,
}

const MAX_NUM: usize = 100;
const GRID_DIM: usize = 5;

impl Iterator for FileReadIterator {
    type Item = Vec<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            FileReadState::Actions => {
                let mut x = String::new();
                self.buf_reader.read_line(&mut x).unwrap();
                self.state = FileReadState::Grids;
                // TODO: some better way to not have to read teh trailing newline in the first
                // place?
                x.pop();
                return Some(x.split(",").map(|x| x.parse::<u32>().unwrap()).collect());
            }
            FileReadState::Grids => {
                let mut grid = Vec::<u32>::with_capacity(GRID_DIM * GRID_DIM);
                let mut x = String::new();
                self.buf_reader.read_line(&mut x).unwrap(); // ignore one line in the input
                for _ in 0..GRID_DIM {
                    // TODO: some better way to build this grid?
                    let mut x = String::new();
                    let sizeread = self.buf_reader.read_line(&mut x).unwrap();
                    if sizeread == 0 {
                        return None;
                    }
                    x.pop();

                    // TODO: split_whitespace combines consecutives whitespaces into one
                    // but for general delimiter, read on: https://stackoverflow.com/q/70223794
                    grid.extend(x.split_whitespace().map(|x| x.parse::<u32>().unwrap()));
                }
                return Some(grid);
            }
        }
    }
}

fn get_reader(day: u32) -> io_result<FileReadIterator> {
    let input_file = File::open(format!("inputs/{}.txt", day))?;
    let file_reader = BufReader::new(input_file);
    return Ok(FileReadIterator {
        buf_reader: file_reader,
        state: FileReadState::Actions,
    });
}

fn precompute(input_reader: &mut FileReadIterator) -> [i32; MAX_NUM] {
    let nums = input_reader.next().unwrap();
    let mut first_appear: [i32; MAX_NUM] = [-1; MAX_NUM];
    for (index, num) in nums.iter().enumerate() {
        if first_appear[*num as usize] == -1 {
            first_appear[*num as usize] = index as i32;
        }
    }
    return first_appear;
}

enum Strategy {
    Earliest,
    Latest,
}

fn day4(mut input_reader: FileReadIterator, strategy: Strategy) -> u32 {
    let first_appear = precompute(&mut input_reader);
    let (mut first_finish, mut finish_answer) = (u32::MAX, u32::MAX);
    match strategy {
        Strategy::Latest => {
            first_finish = 0;
        }
        _ => {}
    }

    // TODO: use rust codegen to make these values constant
    // https://stackoverflow.com/questions/59121887
    let col_masks =
        (0..GRID_DIM).map(|start| ((start..GRID_DIM * GRID_DIM).step_by(GRID_DIM).collect()));
    let row_masks = (0..GRID_DIM * GRID_DIM)
        .step_by(GRID_DIM)
        .map(|start| (start..start + GRID_DIM).step_by(1).collect());
    let masks: Vec<Vec<usize>> = col_masks.chain(row_masks).collect();

    for grid in input_reader {
        let mut earliest = u32::MAX;
        for mask in &masks {
            let mut mx: i32 = 0;
            for pos in mask {
                let val = first_appear[grid[*pos] as usize];
                if val == -1 {
                    mx = val;
                    break;
                }
                mx = i32::max(mx, val);
            }
            if mx == -1 {
                continue;
            }
            if (mx as u32) < earliest {
                earliest = mx as u32;
            }
        }
        match strategy {
            Strategy::Earliest => {
                if earliest > first_finish {
                    continue;
                }
            }
            Strategy::Latest => {
                if earliest < first_finish {
                    continue;
                }
            }
        }
        let (mut sum, mut thatnum) = (0, 0);
        for num in grid {
            let val = first_appear[num as usize] as u32;
            if val > earliest {
                sum += num;
            } else if val == earliest {
                thatnum = num;
            }
        }

        if first_finish == earliest {
            finish_answer = u32::max(finish_answer, thatnum * sum);
        } else {
            finish_answer = thatnum * sum;
            first_finish = earliest;
        }
    }

    return finish_answer;
}

fn main() {
    let mut args = env::args();
    args.next();
    let day = args.next().expect("Give day argument");
    let day_integer = day.parse::<u32>().unwrap();

    let input_iterator = get_reader(day_integer).expect("Input read correctly");

    let answer = day4(input_iterator, Strategy::Latest);
    println!("Answer: {}", answer)
}
