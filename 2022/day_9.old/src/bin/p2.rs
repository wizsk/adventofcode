use std::fs;

use day_9::p2;

fn main() {
    let intput = fs::read_to_string("input.txt").unwrap();

    println!("p2 {:?}", p2(&intput));
}


