use adventofcode_2020::read_lines;

use std::collections::{VecDeque, HashSet};

const PREAMBLE_LENGTH: usize = 25;

fn main() {
    let mut xmas_data = read_lines("day9.txt").map(|l| l.parse::<u64>().unwrap());
    let mut prev = VecDeque::with_capacity(PREAMBLE_LENGTH);
    for _ in 0..PREAMBLE_LENGTH {
        prev.push_back(xmas_data.next().unwrap());
    }
    let mut prev_set: HashSet<_> = prev.iter().copied().collect();

    let mut invalid = 0;

    for num in xmas_data {
        {
            let mut has_2sum = false;
            for &n in &prev_set {
                if let Some(remaining) = num.checked_sub(n) {
                    match prev_set.get(&remaining) {
                        Some(_) => {
                            has_2sum = true;
                            break;
                        },
                        None => (),
                    }
                }
            }
            if !has_2sum {
                invalid = num;
                break
            }
        }
        let to_remove = prev.pop_front().unwrap();
        prev.push_back(num);
        prev_set.remove(&to_remove);
        prev_set.insert(num);
    }
    println!("{}", invalid);

    let mut contiguous = VecDeque::new();
    let mut contiguous_sum = 0;

    for num in read_lines("day9.txt").map(|l| l.parse::<u64>().unwrap()) {
        contiguous.push_back(num);
        contiguous_sum += num;

        while contiguous_sum > invalid {
            let front = contiguous.pop_front().unwrap();
            contiguous_sum -= front;
        }

        if contiguous_sum == invalid {
            break
        }
    }

    let min = *contiguous.iter().min().unwrap();
    let max = *contiguous.iter().max().unwrap();

    println!("{} + {} => {}", min, max, min + max);
}
