use std::fs;

use day_10::part_1::p1;

fn main() {
    let i = fs::read_to_string("input.txt").unwrap();
    println!("part 1 {}", p1(&i));
}
