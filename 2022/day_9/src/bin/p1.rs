use std::fs;

use day_9::part_1;

fn main() {
    let intput = fs::read_to_string("input.txt").unwrap();

    println!("{:?}", part_1(&intput));
}
