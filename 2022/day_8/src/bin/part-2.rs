use std::fs;

use day_8::p2;


fn main() { let input = fs::read_to_string("input.txt").unwrap();
    println!("\np2 {}", p2(&input));
}
