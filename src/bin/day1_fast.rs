use adventofcode_2020::read_lines;

use std::cmp::Ordering::*;
use std::collections::HashSet;

use self_compare::SliceCompareExt;

fn main() {
    let mut numbers: Vec<_> = read_lines("day1.txt")
        .map(|s| s.trim().parse::<u128>().unwrap())
        .collect();

    // O(n lg(n))
    numbers.sort();

    let numbers = numbers;

    // O(n) (n lg(n) is o(n) so it's still O(n lg(n)) in total)
    {
        let mut i = 0;
        let mut j = numbers.len() - 1;

        let (a, b) = loop {
            let sum = numbers[i] + numbers[j];
            match sum.cmp(&2020) {
                Equal => break (numbers[i], numbers[j]),
                Greater => j -= 1,
                Less => i += 1,
            }
        };
        println!("{}*{} = {}", a, b, a*b);
    }

    // O(n²) (which is o(n lg n))
    {
        let set: HashSet<_> = numbers
            .iter()
            .cloned()
            .collect();

        for (a, b) in numbers.self_comparer() {
            let sum = a + b;
            if let Some(to_find) = 2020u128.checked_sub(sum) {
                if set.contains(&to_find) {
                    println!("{}*{}*{} = {}", a, b, to_find, a*b*to_find);
                    break
                }
            }
        }
    }
}
