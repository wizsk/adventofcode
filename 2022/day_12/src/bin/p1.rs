use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("result: {}", day_12::part_1::p1(&input));
}
