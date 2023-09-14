use std::fs;

use day_6::p1;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("p1 {}", p1(&input));
}
