use std::fs;

use day_10::part_2::p2;

fn main() {
    let i = fs::read_to_string("input.txt").unwrap();
    println!("part 1 {}", p2(&i));
}
