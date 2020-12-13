use adventofcode_2020::read_lines;

use std::collections::HashSet;

fn main() {
    let mut sum_union = 0;
    let mut sum_intersection = 0;

    let mut union_of_answers = HashSet::new();
    let mut intersection_of_answers: Option<HashSet<char>> = None;

    for line in read_lines("day6.txt").chain(Some("".to_owned())) {
        if line.is_empty() {
            sum_union += union_of_answers.len();
            sum_intersection += intersection_of_answers.map(|s| s.len()).unwrap_or(0);
            union_of_answers.clear();
            intersection_of_answers = None;
        } else {
            let mut answers = HashSet::new();
            for c in line.chars() {
                union_of_answers.insert(c);
                answers.insert(c);
            }
            if let Some(ref mut intersection_of_answers) = intersection_of_answers {
                *intersection_of_answers = intersection_of_answers.intersection(&answers).copied().collect();
            } else {
                intersection_of_answers = Some(answers);
            }
        }
    }
    println!("{}", sum_union);
    println!("{}", sum_intersection);
}
