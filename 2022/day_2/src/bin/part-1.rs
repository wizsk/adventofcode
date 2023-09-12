use std::fs;

use day_2::p1;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("score is {}", p1(&input));
}
