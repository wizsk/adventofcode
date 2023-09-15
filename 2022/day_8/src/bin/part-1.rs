use std::fs;

use day_8::p1;


fn main() { let input = fs::read_to_string("input.txt").unwrap();
    println!("\np1 {}", p1(&input));
}
