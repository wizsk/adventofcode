use std::fs;

use day_6::p2;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("p2 {}", p2(&input));
}