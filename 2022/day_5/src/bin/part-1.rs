use std::fs;

use day_5::p1;


fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("p1 res {}", p1(&input))
}