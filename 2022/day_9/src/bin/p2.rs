use std::fs;

use day_9::part_2;

fn main() {
    let intput = fs::read_to_string("input.txt").unwrap();

    println!("p2 {:?}", part_2(&intput));
}


