use adventofcode_2020::read_lines;

fn main() {
    let mut seats: Vec<_> = read_lines("day5.txt")
        .map(|s| (
            u16::from_str_radix(&s[..7].replace('F', "0").replace('B', "1"), 2).unwrap(),
            u16::from_str_radix(&s[7..].replace('L', "0").replace('R', "1"), 2).unwrap(),
        ))
        .map(|(row, col)| row * 8 + col)
        .collect();

    seats.sort_unstable();
    println!("max: {}", seats.last().unwrap());

    let mut seats = seats.into_iter();
    let mut last_seat = seats.next().unwrap();
    for seat in seats {
        if seat != last_seat + 1 {
            println!("seat missing: {}", last_seat + 1);
        }
        last_seat = seat;
    }
}
