use std::fs;

use day_9::calculate_m_h_w;

fn main() {
    let intput = fs::read_to_string("input.txt").unwrap();

    println!("{:?}", calculate_m_h_w(&intput));
}
