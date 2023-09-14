use std::fs;

use day_7::p1;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("p1 {}", p1(&input));
}
