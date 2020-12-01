use adventofcode_2020::read_lines;
use self_compare::SliceCompareExt;

use std::cmp::Ordering::*;

fn main() {
    let mut numbers: Vec<_> = read_lines("day1.txt")
        .map(|s| s.trim().parse::<u128>().unwrap())
        .collect();


    // O(n lg(n)) solution
    numbers.sort();

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

    // First solutions that were easy to write
    // O(n²)

    numbers.compare_self(|a, b| {
        if a + b == 2020 {
            println!("{}*{} = {}", a, b, a*b);
        }
    });

    // O(n³)
    for i in 0..numbers.len() {
        for j in i+1..numbers.len()  {
            for k in j+1..numbers.len() {
                let (a, b, c) = (numbers[i], numbers[j], numbers[k]);
                if a + b + c == 2020 {
                    println!("{}*{}*{} = {}", a, b, c, a*b*c);
                    return
                }
            }
        }
    }
}
