use std::fs;

use day_1::p2_find_top_3;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("top 3 {}", p2_find_top_3(&input));
}
