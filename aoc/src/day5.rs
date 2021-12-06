use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
    iter::Iterator,
};

struct FileReadIterator {
    buf_reader: BufReader<File>,
}

type Point = (usize, usize);
type Line = (Point, Point);

impl Iterator for FileReadIterator {
    type Item = Line;
    fn next(&mut self) -> Option<Self::Item> {
        let mut x = String::new();
        self.buf_reader.read_line(&mut x).unwrap();
        if x.is_empty() {
            return None;
        }
        x.pop();
        let line: Vec<Vec<usize>> = x
            .split(" -> ")
            .map(|x| x.split(",").map(|y| y.parse::<usize>().unwrap()).collect())
            .collect();
        assert!(line.len() == 2);
        assert!(line[0].len() == 2);
        assert!(line[1].len() == 2);
        return Some(((line[0][0], line[0][1]), (line[1][0], line[1][1])));
    }
}

fn get_reader(day: u32) -> io_result<FileReadIterator> {
    let input_file = File::open(format!("inputs/{}.txt", day))?;
    let file_reader = BufReader::new(input_file);
    return Ok(FileReadIterator {
        buf_reader: file_reader,
    });
}

const GRID_SIZE: usize = 1000;

fn part1(input_reader: FileReadIterator) -> u32 {
    let mut horizontal = vec![[0 as i16; GRID_SIZE + 1]; GRID_SIZE + 1];
    let mut vertical = vec![[0 as i16; GRID_SIZE + 1]; GRID_SIZE + 1];
    let mut symmdiag = vec![[0 as i16; GRID_SIZE + 1]; GRID_SIZE + 1];
    let mut antidiag = vec![[0 as i16; GRID_SIZE + 1]; GRID_SIZE + 1];
    let mut counts = vec![[0 as u16; GRID_SIZE]; GRID_SIZE];

    for ((mut sx, mut sy), (mut ex, mut ey)) in input_reader {
        let is_horizontal = sy == ey;
        let is_vertical = sx == ex;

        if is_horizontal {
            if sx > ex {
                std::mem::swap(&mut sx, &mut ex);
            }
            horizontal[sy][sx] += 1;
            horizontal[sy][ex + 1] -= 1;
        } else if is_vertical {
            if sy > ey {
                std::mem::swap(&mut sy, &mut ey);
            }
            vertical[sy][sx] += 1;
            vertical[ey + 1][sx] -= 1;
        } else {
            if sx > ex {
                std::mem::swap(&mut sx, &mut ex);
                std::mem::swap(&mut sy, &mut ey);
            }
            if sy < ey {
                symmdiag[sy][sx] += 1;
                symmdiag[ey + 1][ex + 1] += 1;
            } else {
                antidiag[sy][sx] += 1;
                if ey > 0 {
                    antidiag[ey - 1][ex + 1] += 1;
                }
            }
        }
    }

    for i in 0..GRID_SIZE {
        let mut count = 0;
        for j in 0..GRID_SIZE {
            count += horizontal[i][j];
            counts[i][j] = count as u16;
        }
    }
    for j in 0..GRID_SIZE {
        let mut count = 0;
        for i in 0..GRID_SIZE {
            count += vertical[i][j];
            counts[i][j] += count as u16;
        }
    }
    for j in 0..GRID_SIZE {
        let mut count = 0;
        let mut row: i32 = 0;
        let mut col = j;

        while row < GRID_SIZE as i32 && col < GRID_SIZE {
            count += symmdiag[row as usize][col];
            counts[row as usize][col] += count as u16;
            row += 1;
            col += 1;
        }
        row = GRID_SIZE as i32 - 1;
        col = j;
        count = 0;
        while row >= 0 && col < GRID_SIZE {
            count += antidiag[row as usize][col];
            counts[row as usize][col] += count as u16;
            row -= 1;
            col += 1;
        }
    }
    for j in 0..GRID_SIZE {
        let mut count = 0;
        let mut row = j as i32;
        let mut col = 0;

        while row < GRID_SIZE as i32 && col < GRID_SIZE {
            count += symmdiag[row as usize][col];
            counts[row as usize][col] += count as u16;
            row += 1;
            col += 1;
        }
        row = j as i32;
        col = GRID_SIZE - 1;
        count = 0;
        while row >= 0 && col < GRID_SIZE {
            count += antidiag[row as usize][col];
            counts[row as usize][col] += count as u16;
            row -= 1;
            col += 1;
        }
    }

    let mut answer = 0;
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            if counts[i][j] > 1 {
                answer += 1;
            }
        }
    }

    return answer;
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
