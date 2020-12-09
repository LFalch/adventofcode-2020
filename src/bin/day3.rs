use adventofcode_2020::read_lines;

#[derive(Debug, Clone, Copy)]
enum Field {
    Tree,
    Open,
}

impl Field {
    const fn is_tree(self) -> bool {
        match self {
            Field::Tree => true,
            _ => false,
        }
    }
}

struct Map {
    inner: Vec<Vec<Field>>,
}

impl Map {
    fn probe(&self, x: usize, y: usize) -> Field {
        self.inner.get(y).map(|row| row[x % row.len()]).unwrap_or(Field::Open)
    }
}

fn char_to_field(c: char) -> Field {
    match c {
        '.' => Field::Open,
        '#' => Field::Tree,
        _ => unimplemented!(),
    }
}

fn main() {
    let inner = read_lines("day3.txt")
        .map(|s| s.chars().map(char_to_field).collect())
        .collect();
    let map = Map {inner};

    let len = map.inner.len();

    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    let product: usize = slopes.into_iter().map(|(a, b)| {
        let slope = (1..=len/b).map(|i| (a*i, b*i));
        let trees = slope.filter(|&(x, y)| map.probe(x, y).is_tree()).count();
        println!("{}, {} -> {}", a, b, trees);
        trees
    }).product();

    println!("Product: {}", product);
}
