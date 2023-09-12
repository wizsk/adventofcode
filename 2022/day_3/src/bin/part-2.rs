use std::fs;

use day_3::p2_group_sum;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("p2 out: {}", p2_group_sum(&input));
}
