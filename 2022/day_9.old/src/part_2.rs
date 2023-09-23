use super::helpers::{Point, Position};
use std::collections::HashSet;

pub fn p2(input: &str) -> usize {
    let mut been: HashSet<Point> = HashSet::new();
    let mut rope = vec![Point::new(); 10];
    // for i in 0..rope.len() - 1 {
    //     calculate_tail(rope[i].clone(), &mut rope[i + 1]);
    //     println!("{i}:{:?} {}:{:?}", rope[i].clone(), i + 1, &rope[i + 1]);
    //     been.insert(rope[i + 1].clone());
    // }
    // if true {
    //     return 0;
    // }

    for line in input.trim().lines() {
        let cmds = line.split(" ").collect::<Vec<_>>();
        if cmds.len() != 2 {
            panic!("while parsing file cmds.len != 2");
        }

        let ins = cmds[0];
        let steps = cmds[1].parse::<i32>().unwrap();
        // println!("\n== {line} ==");
        for _ in 0..steps {
            match ins {
                "R" => {
                    rope.first_mut().unwrap().x += 1;
                }
                "L" => {
                    rope.first_mut().unwrap().x -= 1;
                }
                "U" => {
                    rope.first_mut().unwrap().y += 1;
                }
                "D" => {
                    rope.first_mut().unwrap().y -= 1;
                }
                _ => panic!("unexpected instruction {ins}"),
            }

            for i in 0..rope.len() - 1 {
                calculate_tail(rope[i].clone(), &mut rope[i + 1]);
                if i == rope.len() - 2 {
                    println!("\t:) but what??");
                    if been.insert(rope[i + 1].clone()) {
                        println!("\tsome thing is wrong! :) but what??");
                    }
                }
            }
            // printit(&rope);
        }
    }

    been.len()
}

fn calculate_tail(head: Point, tail: &mut Point) {
    let dx = head.x - tail.x;
    let dy = head.y - tail.y;
    // same row || same cullmn and next to eatch other
    // println!("calc: {:?} {:?}", head, tail);
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
            println!("i ran lsdkffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
        } else {
            tail.y -= 1;
        }
    } else if head.y == tail.y {
        if dx > 1 {
            println!("i ran lsdkffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff");
            tail.x += 1;
        } else {
            tail.x -= 1;
        }
    } else if head.x == tail.x + 1 && (head.y == tail.y + 1 || head.y == tail.y - 1) {
        tail.x += 1;
    } else if head.x == tail.x - 1 && (head.y == tail.y + 1 || head.y == tail.y - 1) {
        tail.x -= 1;
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

fn move_and_calc(bridge: &mut Position, ins: &str, step: i32) {
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
        // println!("{:?}", bridge);
    }
}

fn printit(b: &Vec<Point>) {
    let bx = b.iter().map(|p| p.x).max().unwrap() + 10;
    let sx = b.iter().map(|p| p.x).min().unwrap() - 10;
    let by = b.iter().map(|p| p.y).max().unwrap() + 10;
    let sy = b.iter().map(|p| p.y).min().unwrap() - 10;
    let mut board: Vec<Vec<char>> =
        vec![vec!['.'; (bx - sx).abs() as usize]; (by - sy).abs() as usize];

    // let set: HashSet<&Point> = HashSet::from_iter(b.iter().clone());

    for p in b {
        // not zero valued
        let x = (p.x + sx.abs() - 1) as usize;
        let y = (p.y + sy.abs() - 1) as usize;
        println!("x{x} y{y}: {p:?}: {sx}, {by}");
        board[y][x] = '0';

        println!();
        println!();
        for b in &board {
            for g in b {
                print!("{g}");
            }
            println!();
        }
        println!();
    }
    return;

    // lets go crazyyyyy
    // with the mems
    let board = vec![vec!['.'; (bx - sx).abs() as usize]; (by - sy).abs() as usize];
    println!();
    println!();
    for b in board {
        for g in b {
            print!("{g}");
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
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
        let result = p2(inp);
        assert_eq!(result, 36);
    }
}
