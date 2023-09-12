use std::fs;

use day_2::p2;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("score is {}", p2(&input));
}
