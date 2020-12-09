use adventofcode_2020::read_lines;

use std::collections::HashMap;

const REQUIRED_FIELDS: &[&str] = &[
    "byr", // (Birth Year)
    "iyr", // (Issue Year)
    "eyr", // (Expiration Year)
    "hgt", // (Height)
    "hcl", // (Hair Color)
    "ecl", // (Eye Color)
    "pid", // (Passport ID)
    // Let's ignore this one ;)
    // "cid", // (Country ID)
];

const EYE_COLOURS: &[&str] = &[
    "amb",
    "blu",
    "brn",
    "gry",
    "grn",
    "hzl",
    "oth",
];

fn bool(b: bool) -> Option<()> {
    if b { Some(()) } else { None }
}

fn in_range<I: Ord>(low: I, high: I, actual: I) -> Option<()> {
    bool(low <= actual && actual <= high)
} 

fn height(s: &str) -> Option<()> {
    let unit_i = s.find(|c: char| !c.is_numeric())?;
    let hgt = s[..unit_i].parse().ok()?;
    match &s[unit_i..] {
        "cm" => in_range(150, 193, hgt),
        "in" => in_range(59, 76, hgt),
        _ => None,
    }
}

fn validate_passport(passport: &HashMap<String, String>) -> Option<()> {
    in_range(1920, 2002, passport.get("byr")?.parse().ok()?)?;
    in_range(2010, 2020, passport.get("iyr")?.parse().ok()?)?;
    in_range(2020, 2030, passport.get("eyr")?.parse().ok()?)?;
    height(passport.get("hgt")?)?;
    bool(EYE_COLOURS.contains(&passport.get("ecl")?))?;
    let hcl = passport.get("hcl")?;
    bool(hcl.len() == 7);
    let mut hcl = hcl.chars();
    bool(hcl.next() == Some('#'))?;
    for c in hcl {
        in_range('0', '9', c).or(in_range('a', 'f', c))?;
    }
    let pid = passport.get("pid")?;
    bool(pid.len() == 9)?;
    bool(pid.chars().all(<char>::is_numeric))
}

fn main() {
    let mut passports = vec![HashMap::new()];

    for line in read_lines("day4.txt") {
        if line.is_empty() {
            passports.push(HashMap::new());
        } else {
            let passport = passports.last_mut().unwrap();
            for entry in line.split(' ') {
                let colon = entry.find(':').unwrap();
                passport.insert(entry[..colon].to_owned(), entry[colon+1..].to_owned());
            }
        }
    }

    let valids = passports
        .iter()
        .filter(|pp| REQUIRED_FIELDS.iter().all(|&field| pp.contains_key(field)))
        .count();

    println!("Valids: {}", valids);

    let very_valids = passports
        .iter()
        .filter_map(validate_passport)
        .count();

    println!("Valids: {}", very_valids);
}
