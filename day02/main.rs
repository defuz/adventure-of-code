use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let mut total_square = 0;
    let mut total_ribbon = 0;
    for line in BufReader::new(File::open("input.txt").unwrap()).lines().map(|r| r.unwrap()) {
        let mut v: Vec<u32> = line.split('x').map(|s| s.parse().unwrap()).collect();
        v.sort();
        total_square += 3 * v[0] * v[1] + 2 * v[1] * v[2] + 2 * v[2] * v[0];
        total_ribbon += 2 * v[0] + 2 * v[1] + v[0] * v[1] * v[2];
    }
    println!("Part one: {}", total_square);
    println!("Part two: {}", total_ribbon);
}
