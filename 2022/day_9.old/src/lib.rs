use std::collections::HashSet;

mod helpers;
mod part_2;

use crate::helpers::*;

pub fn p1(input: &str) -> usize {
    let mut been: HashSet<Point> = HashSet::new();
    // 0 -> real head
    let mut bridge = Position::new();

    for line in input.trim().lines() {
        // println!("\n== {line} ==");
        parse_ins(&mut been, &mut bridge, line);
    }
    been.len()
}



// parse instruction
fn parse_ins(_set: &mut HashSet<Point>, bridge: &mut Position, line: &str) {
    let cmds = line.split(" ").collect::<Vec<_>>();
    if cmds.len() != 2 {
        panic!("while parsing file cmds.len != 2");
    }

    let ins = cmds[0];
    let step = cmds[1].parse::<i32>().unwrap();

    move_and_calc(_set, bridge, ins, step);

    // println!("{:?}", bridge.head);
}

fn move_and_calc(_set: &mut HashSet<Point>, bridge: &mut Position, ins: &str, step: i32) {
    for _ in 0..step {
        match ins {
            "R" => {
                bridge.head.x += 1;
            }
            "L" => {
                bridge.head.x -= 1;
            }
            "U" => {
                bridge.head.y += 1;
            }
            "D" => {
                bridge.head.y -= 1;
            }
            _ => panic!("unexpected instruction {ins}"),
        }
        bridge.calculate();
        _set.insert(bridge.tail.clone());
        // println!("{:?}", bridge);
    }
}

fn calculate_tail(head: Point, tail: &mut Point) {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;
    // same row || same cullmn and next to eatch other
    println!("calc: {:?} {:?}", head, tail);
    if dx == 0 && dy == 0
        || head.x == tail.x && (dy == 1 || dy == -1)
        || head.y == tail.y && (dx == 1 || dx == -1)
    {
        // println!(":::::::::::::::::::::::::::::::pasei ache");
        return;
    }

    if head.x == tail.x {
        if dy > 1 {
            tail.y += 1;
        } else {
            tail.y -= 1;
        }
    } else if head.y == tail.y {
        if dx > 1 {
            tail.x += 1;
        } else {
            tail.x -= 1;
        }
    // } else if head.x == tail.x + 1 && (head.y == tail.y + 1 || head.y == tail.y - 1) {
    //     tail.x += 1;
    // } else if head.x == tail.x - 1 && (head.y == tail.y + 1 || head.y == tail.y - 1) {
    //     tail.x -= 1;
    //
    } else if head.x == tail.x + 1 && head.y == tail.y + 2
        || head.x == tail.x + 2 && head.y == tail.y + 1
    {
        tail.x += 1;
        tail.y += 1;
    } else if head.x == tail.x + 1 && head.y == tail.y - 2
        || head.x == tail.x + 2 && head.y == tail.y - 1
    {
        tail.x += 1;
        tail.y -= 1;
    } else if head.x == tail.x - 1 && head.y == tail.y - 2
        || head.x == tail.x - 2 && head.y == tail.y - 1
    {
        tail.x -= 1;
        tail.y -= 1;
    } else if head.x == tail.x - 1 && head.y == tail.y + 2
        || head.x == tail.x - 2 && head.y == tail.y + 1
    {
        tail.x -= 1;
        tail.y += 1;
    }
    // println!("cccc: {:?}\n\t{dx} {dy}", self);
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
";

    #[test]
    fn calc() {
        let result = p1(INPUT);
        assert_eq!(result, 13);
    }
    #[test]
    fn test_p2() {
        let inp: &str = "R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";
        // let result = p2(inp);
        // assert_eq!(result, 36);
    }
}
