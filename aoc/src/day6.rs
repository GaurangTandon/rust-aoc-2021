use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
    iter::Iterator,
};

struct FileReadIterator {
    buf_reader: BufReader<File>,
    values: Vec<u32>,
    index: usize,
    has_read: bool,
}

impl Iterator for FileReadIterator {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_read == false {
            let mut x = String::new();
            self.buf_reader.read_line(&mut x).unwrap();
            x.pop();
            self.values = x.split(",").map(|x| x.parse::<u32>().unwrap()).collect();
            self.has_read = true;
        }
        if self.index < self.values.len() {
            self.index += 1;
            return Some(self.values[self.index - 1]);
        }
        return None;
    }
}

fn part1(input_reader: FileReadIterator) -> u64 {
    const DAYS: usize = 256;
    const FISH_MAX: usize = 8;
    let mut dp = [[0 as u64; DAYS + FISH_MAX + 2]; FISH_MAX + 1];
    // dp[i value fish][jth day] = dp[6][j + i + 1] + dp[8][j + i + 1]
    // dp[i][DAYS + 1] = 0
    for day in (0..=DAYS).rev() {
        for fish in 0..=FISH_MAX {
            let creation_day = day + fish + 1;
            if creation_day > DAYS {
                dp[fish][day] = 1;
            } else {
                dp[fish][day] = dp[6][creation_day] + dp[8][creation_day];
            }
        }
    }

    let mut answer = 0;
    for fish in input_reader {
        answer += dp[fish as usize][0];
    }
    return answer;
}

fn get_reader(day: u32) -> io_result<FileReadIterator> {
    let input_file = File::open(format!("inputs/{}.txt", day))?;
    let file_reader = BufReader::new(input_file);
    return Ok(FileReadIterator {
        buf_reader: file_reader,
        index: 0,
        values: Vec::new(),
        has_read: false,
    });
}

fn main() {
    let mut args = env::args();
    args.next();
    let day = args.next().expect("Give day argument");
    let day_integer = day.parse::<u32>().unwrap();

    let input_iterator = get_reader(day_integer).expect("Input read correctly");

    let answer = part1(input_iterator);
    println!("Answer: {}", answer)
}
