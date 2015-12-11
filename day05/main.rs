use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    // PART 1
    let good_words = BufReader::new(File::open("input.txt").unwrap())
                     .lines().map(Result::unwrap).filter(|word| {
        let mut vowels = 0;
        let mut prev = '\n';
        let mut has_double = false;

        for c in word.chars() {
            match (prev, c) {
                ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y') =>
                    return false,
                _ => {}
            }
            if "aeiou".contains(c) {
                vowels += 1
            }
            if prev == c {
                has_double = true;
            }
            prev = c;
        }

        (vowels >= 3) && has_double
    }).count();
    println!("Part one: {}", good_words);
    // PART 2
    let good_words = BufReader::new(File::open("input.txt").unwrap())
                     .lines().map(Result::unwrap).filter(|word| {
        let c = word.chars().collect::<Vec<_>>();
        let mut rule1 = false;
        let mut rule2 = false;
        for i in 0..c.len()-2 {
            if c[i] == c[i+2] {
                rule1 = true;
            }
            for j in i+2..c.len()-1 {
                if c[i] == c[j] && c[i+1] == c[j+1] {
                    rule2 = true
                }
            }
        }
        rule1 && rule2
    }).count();
    println!("Part two: {}", good_words);
}
