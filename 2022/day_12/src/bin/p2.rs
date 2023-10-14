use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("result part 2: {}", day_12::part_2::p2(&input));
}
