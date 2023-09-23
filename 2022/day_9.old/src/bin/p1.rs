use std::fs;

use day_9::p1;

fn main() {
    // let intput = fs::read_to_string("input.txt").unwrap();
    let intput = match fs::read_to_string("input.txt") {
    };

    println!("{:?}", p1(&intput));
}
