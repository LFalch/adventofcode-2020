use adventofcode_2020::read_lines;
use self_compare::SliceCompareExt;

fn main() {
    let numbers: Vec<_> = read_lines("day1.txt")
        .map(|s| s.trim().parse::<u128>().unwrap())
        .collect();

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
