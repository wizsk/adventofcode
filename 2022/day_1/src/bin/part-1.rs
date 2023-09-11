use std::fs;

use day_1::p1_find_greatest_cal;

fn main() {
    let input = fs::read_to_string("input.txt").expect("file 'input.txt' not found");
    println!("most cal: {}", p1_find_greatest_cal(&input));
}
