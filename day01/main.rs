#![feature(io)]

use std::fs::File;
use std::io::Read;

fn main() {
    let mut counter = 0;
    let mut position = None;
    for (i, c) in File::open("input.txt").unwrap().chars().enumerate() {
        match c.unwrap() {
            '(' => counter += 1,
            ')' => counter -= 1,
            _ => ()
        }
        if position == None && counter == -1 {
            position = Some(i+1);
        }
    }
    println!("Part one: {}", counter);
    println!("Part two: {}", position.unwrap());
}
