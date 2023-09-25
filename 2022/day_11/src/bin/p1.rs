use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("p1: {}", day_11::part_1::p1(&input));
}
