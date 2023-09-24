use std::collections::HashSet;

pub fn part_1(input: &str) -> usize {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert((0, 0));

    let mut head = (0, 0);
    let mut tail = (0, 0);

    for line in input.trim().lines() {
        let arg = line.split(" ").collect::<Vec<_>>();
        let cmd = arg[0];
        let mv = arg[1]
            .parse::<i32>()
            .expect("part_1: while parsing argumnet");
        for _ in 0..mv {
            match cmd {
                "R" => {
                    head.0 += 1;
                }
                "L" => {
                    head.0 -= 1;
                }
                "U" => {
                    head.1 += 1;
                }
                "D" => {
                    head.1 -= 1;
                }
                _ => panic!("unexpected instruction {cmd}"),
            }

            let dx: i32 = head.0 - tail.0;
            let dy: i32 = head.1 - tail.1;

            if dx.abs() > 1 || dy.abs() > 1 {
                // let _x = ternary![dx > 0, 1, -1];
                // let _y = ternary![dy > 0, 1, -1];
                if dx == 0 {
                    tail.1 += dy / 2;
                } else if dy == 0 {
                    tail.0 += dx / 2;
                } else {
                    tail.0 += if dx > 0 { 1 } else { -1 };
                    tail.1 += if dy > 0 { 1 } else { -1 };
                }
            }
            set.insert(tail);
        }
    }

    set.len()
}

pub fn part_2(input: &str) -> usize {
    let mut set: HashSet<(i32, i32)> = HashSet::new();
    set.insert((0, 0));
    let mut rope = vec![(0, 0); 10];

    for line in input.trim().lines() {
        let arg = line.split(" ").collect::<Vec<_>>();
        let cmd = arg[0];
        let mv = arg[1]
            .parse::<i32>()
            .expect("part_1: while parsing argumnet");
        for _ in 0..mv {
            let head = &mut rope[0];
            match cmd {
                "R" => {
                    head.0 += 1;
                }
                "L" => {
                    head.0 -= 1;
                }
                "U" => {
                    head.1 += 1;
                }
                "D" => {
                    head.1 -= 1;
                }
                _ => panic!("unexpected instruction {cmd}"),
            }

            // for i in
            for i in 0..rope.len() - 1 {
                let head = &rope[i];
                let tail = &rope[i + 1];
                let dx: i32 = head.0 - tail.0;
                let dy: i32 = head.1 - tail.1;

                if dx.abs() > 1 || dy.abs() > 1 {
                    // let _x = ternary![dx > 0, 1, -1];
                    // let _y = ternary![dy > 0, 1, -1];
                    if dx == 0 {
                        rope[i + 1].1 += dy / 2;
                    } else if dy == 0 {
                        rope[i + 1].0 += dx / 2;
                    } else {
                        rope[i + 1].0 += if dx > 0 { 1 } else { -1 };
                        rope[i + 1].1 += if dy > 0 { 1 } else { -1 };
                    }
                }
            }
            set.insert(rope.last().unwrap().clone());
        }
    }

    set.len()
}
