use adventofcode_2020::read_lines;

fn parse(s: &str) -> (usize, usize, char, &str) {
    let i = s.find(':').unwrap();
    let (rule, pass) = s.split_at(i);
    let line = s.find('-').unwrap();
    let space = s.find(' ').unwrap();
    let i = rule[..line].parse().unwrap();
    let j = rule[line+1..space].parse().unwrap();

    (i, j, s[space+1..].chars().next().unwrap(), &pass[2..])
}

fn check(s: &str) -> bool {
    let (i, j, ch, pass) = parse(s);
    let count = pass.chars().filter(|&c| c == ch).count();
    (i..=j).contains(&count)
}

fn check2(s: &str) -> bool {
    let (i, j, ch, pass) = parse(s);

    let mut chs = pass.chars();
    (chs.nth(i-1).unwrap() == ch) ^
        (chs.nth(j-i-1).unwrap() == ch)
}

#[test]
fn test() {
    assert!(check2("1-3 a: abcde"));
    assert!(!check2("1-3 b: cdefg"));
    assert!(!check2("2-9 c: ccccccccc"));
}

fn main() {
    let count = read_lines("day2.txt").filter(|s| check(s)).count();
    println!("{}", count);
    let count = read_lines("day2.txt").filter(|s| check2(s)).count();
    println!("{}", count);
}
