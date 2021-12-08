use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
    iter::Iterator,
};

struct FileReadIterator {
    buf_reader: BufReader<File>,
}

const INPUT_COUNT: usize = 10;
const OUTPUT_COUNT: usize = 4;

impl Iterator for FileReadIterator {
    type Item = (Vec<String>, Vec<String>);
    fn next(&mut self) -> Option<Self::Item> {
        let mut x = String::new();
        let sizeread = self.buf_reader.read_line(&mut x).unwrap();
        if sizeread == 0 {
            return None;
        }
        x.pop();
        let (input, output) = x.split_once(" | ").unwrap();
        let input_array = input
            .splitn(INPUT_COUNT, char::is_whitespace)
            .map(|x| String::from(x))
            .collect();
        let output_array = output
            .splitn(OUTPUT_COUNT, char::is_whitespace)
            .map(|x| String::from(x))
            .collect();
        Some((input_array, output_array))
    }
}

fn part1(input_reader: FileReadIterator) -> u32 {
    let mut count = 0;
    for (_, output) in input_reader {
        for entry in output {
            let len = entry.len();
            if len == 2 || len == 3 || len == 4 || len == 7 {
                count += 1;
            }
        }
    }

    count
}

const DIGIT_COUNT: usize = 10;

fn part2(input_reader: FileReadIterator) -> u32 {
    input_reader.fold(0, |sum, (input_arr, output_arr)| {
        // digit to its mask mapping
        let mut mbp = [0; DIGIT_COUNT];
        // mask to digit mapping
        let mut mask_to_digit = HashMap::<u32, u32>::new();

        let mut six_dig_masks = 0;
        let mut five_digs = Vec::<u32>::new();
        let mut six_digs = Vec::<u32>::new();

        for input in input_arr {
            let mask_val = input.chars().fold(0, |valid_mask, ch| {
                valid_mask | 1 << ((ch as u32) - ('a' as u32))
            });
            let len = input.len();

            let idx = if len == 2 {
                1
            } else if len == 3 {
                7
            } else if len == 7 {
                8
            } else if len == 4 {
                4
            } else {
                DIGIT_COUNT
            };

            if idx < DIGIT_COUNT {
                mbp[idx] = mask_val;
            } else if len == 5 {
                five_digs.push(mask_val);
            } else if len == 6 {
                six_dig_masks ^= mask_val;
                six_digs.push(mask_val);
            }
        }

        let a = mbp[1] ^ mbp[7];
        let cf = mbp[1];
        let bd = mbp[4] ^ mbp[1];
        let eg = mbp[8] ^ bd ^ cf ^ a;
        let bfg = six_dig_masks ^ a;

        let mut d = 0;
        for m in five_digs.iter() {
            if (m & bfg) == bfg {
                mask_to_digit.insert(*m, 5);
                mbp[5] = *m;
                d = (*m) ^ a ^ bfg;
                break;
            }
        }
        assert!(d > 0);

        let b = d ^ bd;

        let mut c = 0;
        for m in five_digs.iter() {
            if (m & eg) == eg {
                mask_to_digit.insert(*m, 2);
                mbp[2] = *m;
                c = (*m) ^ a ^ eg ^ d;
                break;
            }
        }
        assert!(c > 0);
        let f = cf ^ c;

        let threem = a ^ c ^ d ^ f;

        let mut g = 0;
        for m in five_digs.iter() {
            if (m & threem) == threem {
                mask_to_digit.insert(*m, 3);
                mbp[3] = *m;
                g = (*m) ^ threem;
                break;
            }
        }
        assert!(g > 0);

        let e = eg ^ g;

        mask_to_digit.insert(a ^ b ^ c ^ e ^ f ^ g, 0);
        mask_to_digit.insert(c ^ f, 1);
        mask_to_digit.insert(a ^ c ^ d ^ e ^ g, 2);
        mask_to_digit.insert(a ^ c ^ d ^ f ^ g, 3);
        mask_to_digit.insert(b ^ c ^ d ^ f, 4);
        mask_to_digit.insert(a ^ b ^ d ^ f ^ g, 5);
        mask_to_digit.insert(a ^ b ^ d ^ e ^ g ^ f, 6);
        mask_to_digit.insert(a ^ c ^ f, 7);
        mask_to_digit.insert(mbp[8], 8);
        mask_to_digit.insert(mbp[8] ^ e, 9);

        let mut num = 0;
        for output in output_arr {
            let mask_val = output.chars().fold(0, |valid_mask, ch| {
                valid_mask | 1 << ((ch as u32) - ('a' as u32))
            });
            num = 10 * num + mask_to_digit.get(&mask_val).unwrap();
        }

        sum + num
    })
}

fn get_reader(day: u32) -> io_result<FileReadIterator> {
    let input_file = File::open(format!("inputs/{}.txt", day))?;
    let file_reader = BufReader::new(input_file);
    Ok(FileReadIterator {
        buf_reader: file_reader,
    })
}

fn main() {
    let mut args = env::args();
    args.next();
    let day = args.next().expect("Give day argument");
    let day_integer = day.parse::<u32>().unwrap();

    let input_iterator = get_reader(day_integer).expect("Input read correctly");

    // let answer = part1(input_iterator);
    let answer = part2(input_iterator);
    println!("Answer: {}", answer)
}
