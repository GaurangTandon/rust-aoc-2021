use std::{
    collections::HashSet,
    fs::File,
    io::{BufReader, Read, Result as io_result},
};

struct FileReadIterator {
    buffer: [u8; 1],
    buf_reader: BufReader<File>,
}

enum Action {
    U,
    R,
    D,
    L,
}

impl Iterator for FileReadIterator {
    type Item = Action; // 0-3, starting up and clockwise
    fn next(&mut self) -> Option<Action> {
        self.buf_reader.read_exact(&mut self.buffer).unwrap();
        let one_char = self.buffer[0] as char;

        if char::is_whitespace(one_char) {
            return None;
        }

        return Some(match one_char {
            '^' => Action::U,
            '>' => Action::R,
            'v' => Action::D,
            '<' => Action::L,
            _ => panic!("Bad input"),
        });
    }
}

/// This function gets a file reader over the current day
/// Rust follows snake case
fn get_reader(day: u32) -> io_result<FileReadIterator> {
    let input_file = File::open(format!("inputs/{}.txt", day))?;
    let file_reader = BufReader::new(input_file);
    return Ok(FileReadIterator {
        buf_reader: file_reader,
        buffer: [0; 1],
    });
}

fn part1(input_reader: FileReadIterator) -> u32 {
    let mut hs = HashSet::new();
    let mut position = (0, 0);
    for token in input_reader {
        match token {
            Action::U => {
                position.0 += 1;
            }
            Action::D => {
                position.0 -= 1;
            }
            Action::R => {
                position.1 += 1;
            }
            Action::L => {
                position.1 -= 1;
            }
        }
        hs.insert(position);
    }

    return hs.len() as u32;
}

fn part2(input_reader: FileReadIterator) -> u32 {
    let start_pos = (0, 0);
    let mut hs = HashSet::new();
    hs.insert(start_pos);

    let mut positions = [start_pos, start_pos];
    let mut curr = 0;
    for token in input_reader {
        match token {
            Action::U => {
                positions[curr].0 += 1;
            }
            Action::D => {
                positions[curr].0 -= 1;
            }
            Action::R => {
                positions[curr].1 += 1;
            }
            Action::L => {
                positions[curr].1 -= 1;
            }
        }
        hs.insert(positions[curr]);
        curr = 1 - curr;
    }

    return hs.len() as u32;
}

fn main() {
    let input_iterator = get_reader(3).expect("Input read correctly");
    // let answer = part1(input_iterator);
    let answer = part2(input_iterator);
    println!("Answer: {}", answer)
}
