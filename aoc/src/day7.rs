use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Result as io_result},
    iter::Iterator,
};

struct FileReadIterator {
    buf_reader: BufReader<File>,
    has_read: bool,
}

impl Iterator for FileReadIterator {
    type Item = Vec<u32>;
    fn next(&mut self) -> Option<Self::Item> {
        if self.has_read == false {
            let mut x = String::new();
            self.buf_reader.read_line(&mut x).unwrap();
            x.pop();
            return Some(x.split(",").map(|x| x.parse::<u32>().unwrap()).collect());
        }
        return None;
    }
}

fn part1(mut input_reader: FileReadIterator) -> u32 {
    let mut values = input_reader.next().unwrap();
    values.sort();
    let mut sum = 0;
    for value in &values {
        sum += *value - values[0];
    }
    let mut answer_cost = u32::MAX;

    for index in 0..values.len() {
        if sum < answer_cost {
            answer_cost = sum;
        }
        if index < values.len() - 1 {
            let delta = values[index + 1] - values[index];
            let peoplebehind = (index + 1) as u32;
            let peopleahead = (values.len() - index - 1) as u32;

            sum += delta * peoplebehind;
            sum -= delta * peopleahead;
        }
    }

    return answer_cost;
}

fn ap(x: u32) -> u32 {
    return x * (x + 1) / 2;
}

fn part2(mut input_reader: FileReadIterator) -> u32 {
    let mut values = input_reader.next().unwrap();
    values.sort();

    let mut sum = 0;
    let len = values.len() as u32;
    for value in &values {
        sum += value;
    }
    sum /= len;

    let candidates = [sum, sum + 1];

    let mut answer_cost = u32::MAX;

    for cand in candidates {
        let mut cost = 0;
        for value in &values {
            cost += ap(if *value > cand { value - cand } else { cand - value });
        }
        answer_cost = u32::min(answer_cost, cost);
    }

    return answer_cost;
}

fn get_reader(day: u32) -> io_result<FileReadIterator> {
    let input_file = File::open(format!("inputs/{}.txt", day))?;
    let file_reader = BufReader::new(input_file);
    return Ok(FileReadIterator {
        buf_reader: file_reader,
        has_read: false,
    });
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
