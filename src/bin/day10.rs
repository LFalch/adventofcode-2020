use adventofcode_2020::read_lines;

use std::collections::{VecDeque, HashSet};

fn main() {
    let mut adapter_jolts: Vec<_> = Some(0).into_iter().chain(read_lines("day10.txt").map(|l| l.parse::<u64>().unwrap())).collect();
    adapter_jolts.sort_unstable();
    let mut diffs1 = 0;
    // Always has the difference between the phone battery joltage and last adapter (which in the problem is defined to be 3)
    let mut diffs3 = 1;
    for window in adapter_jolts.windows(2) {
        match window {
            &[a, b] => match b - a {
                1 => diffs1 += 1,
                3 => diffs3 += 1,
                _ => (),
            }
            _ => unreachable!(),
        }
    }
    println!("{} * {} => {}", diffs1, diffs3, diffs1 * diffs3);

    // TODO determine number of ordered sub-lists that maintain only a maximum difference of 3 between each element
}
