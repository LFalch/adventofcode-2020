use adventofcode_2020::read_lines;

use std::collections::HashMap;

struct CanHaveMap {
    inner: HashMap<String, bool>,
}

impl CanHaveMap {
    fn new() -> Self {
        CanHaveMap { inner: HashMap::new()}
    }
    fn can_have(&mut self, colour: &str, rules: &HashMap<String, Vec<(usize, String)>>) -> bool {
        if let Some(&b) = self.inner.get(colour) {
            b
        } else {
            let mut has_bag = false;
            for (_, inner_colour) in &rules[colour] {
                if inner_colour == colour {

                } else if inner_colour == SHINY_GOLD {
                    has_bag = true;
                    break;
                } else {
                    if self.can_have(inner_colour, rules) {
                        has_bag = true;
                        break;
                    }
                }
            }

            self.inner.insert(colour.to_owned(), has_bag);
            has_bag
        }
    }
}

const SHINY_GOLD: &str = "shiny gold";

fn main() {
    let mut rules: HashMap<String, Vec<(usize, String)>> = HashMap::new();

    for line in read_lines("day7.txt") {
        const BAGS_CONTAIN: &str = " bags contain";
        let first_split = line.find(BAGS_CONTAIN).unwrap();
        let colour = line[..first_split].to_owned();
        let rest = &line[first_split+BAGS_CONTAIN.len()..];
        if rest == " no other bags." {
            rules.insert(colour, Vec::new());
        } else {
            let mut bags = Vec::with_capacity(2);
            for bag_rule in rest.split(",") {
                let bag_rule = bag_rule.trim();
                let space = bag_rule.find(' ').unwrap();
                let bag = bag_rule.find(" bag").unwrap();
                let number = bag_rule[..space].parse().unwrap();
                let inner_colour = bag_rule[space+1..bag].to_owned();

                bags.push((number, inner_colour));
            }
            rules.insert(colour, bags);
        }
    }

    let bags_that_have_gold_bag = {
        let mut can_have_map = CanHaveMap::new();
        rules.keys().filter(|k| can_have_map.can_have(k, &rules)).count()
    };

    println!("{}", bags_that_have_gold_bag);

    fn count_inside(colour: &str, rules: &HashMap<String, Vec<(usize, String)>>) -> usize {
        let mut sum = 0;
        for (num, colour) in &rules[colour] {
            sum += num * (1 + count_inside(colour, rules));
        }
        sum
    }

    println!("{}", count_inside(SHINY_GOLD, &rules));
}
