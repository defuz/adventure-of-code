#![feature(io)]
use std::fs::File;
use std::io::Read;
use std::collections::HashSet;

fn first_year() -> usize {
    let mut x = 0;
    let mut y = 0;
    let mut houses = HashSet::new();
    for c in File::open("input.txt").unwrap().chars() {
        match c.unwrap() {
            '<' => x -= 1,
            '>' => x += 1,
            '^' => y -= 1,
            'v' => y += 1,
            _ => ()
        }
        houses.insert((x, y));
    }
    houses.len()
}

fn second_year() -> usize {
    let mut santa : (i16, i16) = (0, 0);
    let mut robot : (i16, i16) = (0, 0);
    let mut houses = HashSet::new();
    for (i, c) in File::open("input.txt").unwrap().chars().enumerate() {
        let (x, y) = if i % 2 == 0 {&mut santa} else {&mut robot};
        match c.unwrap() {
            '<' => *x -= 1,
            '>' => *x += 1,
            '^' => *y -= 1,
            'v' => *y += 1,
            _ => ()
        }
        houses.insert((*x, *y));
    }
    houses.len()
}

fn main() {
    println!("Part one: {}", first_year());
    println!("Part two: {}", second_year());
}
