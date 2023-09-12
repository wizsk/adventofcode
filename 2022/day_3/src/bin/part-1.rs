use std::fs;

use day_3::p1_sum;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("p1 out: {}", p1_sum(&input));
}
