use std::{
    fs::File,
    io::{BufReader, Read, Result as io_result},
};

const SIZE: usize = 12;

struct FileReadIterator {
    buffer: [u8; SIZE + 1],
    buf_reader: BufReader<File>,
}

impl Iterator for FileReadIterator {
    type Item = u32; // 0-3, starting up and clockwise
    fn next(&mut self) -> Option<Self::Item> {
        let mut failed = false;
        // TODO: any better way to handle the EOF?
        self.buf_reader
            .read_exact(&mut self.buffer)
            .unwrap_or_else(|_| failed = true);

        if failed {
            return None;
        }

        // TODO: Is there no efficient way to pass the buffer directly to from_str_radix?
        // Because creating this string object is expensive
        let take = std::str::from_utf8(&self.buffer[0..SIZE])
            .unwrap()
            .to_string();
        return Some(u32::from_str_radix(&take, 2).unwrap());
    }
}

/// This function gets a file reader over the current day
/// Rust follows snake case
fn get_reader(day: u32) -> io_result<FileReadIterator> {
    let input_file = File::open(format!("inputs/{}.txt", day))?;
    let file_reader = BufReader::new(input_file);
    return Ok(FileReadIterator {
        buf_reader: file_reader,
        buffer: [0; SIZE + 1],
    });
}

fn part1(input_reader: FileReadIterator) -> u32 {
    let mut count = [0; SIZE];
    let mut total = 0;

    for token in input_reader {
        for bit in 0..SIZE {
            count[bit] += if (token & (1 << bit)) > 0 { 1 } else { 0 };
        }
        total += 1;
    }

    let (mut low, mut high) = (0, 0);
    for bit in 0..SIZE {
        let one = count[bit];
        let zero = total - one;
        if one > zero {
            high += 1 << bit;
        } else {
            low += 1 << bit;
        }
    }
    return low * high;
}

fn part2(input_reader: FileReadIterator) -> u32 {
    let mut tokens = input_reader.collect::<Vec<u32>>();
    tokens.sort();

    let mut answer = [0, 0];

    for case in 0..2 {
        let mut number: u32 = 0;
        let (mut b, mut e): (i32, i32) = (0, tokens.len() as i32 - 1);
        for bit in (0..SIZE).rev() {
            // find the first index at which a 1 occurs
            // using binary search
            let (b_, e_, mut ans): (i32, i32, i32) = (b, e, e + 1);
            if b == e {
                number = tokens[b as usize];
                break;
            }
            while b <= e {
                let mid = (b + e) / 2;
                if (tokens[mid as usize] & (1 << bit)) > 0 {
                    ans = mid;
                    e = mid - 1;
                } else {
                    b = mid + 1;
                }
            }
            let count = [ans - 1 - b_ + 1, e_ - ans + 1];
            let keep_one =
                (case == 0 && count[1] >= count[0]) || (case == 1 && count[1] < count[0]);
            if keep_one {
                b = ans;
                e = e_;
                number += 1 << bit;
            } else {
                b = b_;
                e = ans - 1;
            }
        }
        answer[case] = number;
    }

    println!("{} {}", answer[0], answer[1]);
    return answer[0] * answer[1];
}

fn main() {
    // TODO: take day argument from command line
    let input_iterator = get_reader(3).expect("Input read correctly");
    // let answer = part1(input_iterator);
    let answer = part2(input_iterator);
    println!("Answer: {}", answer)
}
