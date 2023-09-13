use std::fs;

use day_4::p2;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("p2 res {}", p2(&input))
}
