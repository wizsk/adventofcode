pub fn p1(input: &str) -> String {
    let stack = input.split("\n\n").nth(0).unwrap();
    let instructions = input.split("\n\n").nth(1).unwrap().trim();
    let stack_size = (stack.lines().nth(0).unwrap().len() + 1) / 4;

    let mut stack_vec: Vec<Vec<char>> = vec![vec![]; stack_size];

    stack
        .lines()
        .filter(|line| !line.trim().is_empty() || line.contains("["))
        .rev()
        .for_each(|line| {
            let mut index = 0;
            for (i, c) in line.chars().enumerate() {
                if (i as i32 - 1) % 4 == 0 {
                    if !c.is_numeric() && c != ' ' {
                        stack_vec[index].push(c);
                    }
                    index += 1;
                }
            }
        });

    // move 3 from 1 to 3
    // instructions.trim().lines().map(|line| line.split(" ").filter(|itm| itm.))
    instructions.trim().lines().for_each(|line| {
        let ins = line
            .split(" ")
            .filter(|itm| match itm.parse::<u32>() {
                Ok(_) => true,
                Err(_) => false,
            })
            .collect::<Vec<_>>();
        // println!("{:?}", ins);

        let mut box_count: u32 = 0;
        let mut source_stack: usize = 0;
        for (i, step) in ins.iter().enumerate() {
            match i {
                // 0 =>
                0 => {
                    // pop
                    box_count = step.parse().unwrap();
                }
                1 => {
                    // where
                    source_stack = step.parse::<usize>().unwrap() - 1;
                }
                2 => {
                    let current_stack: usize = step.parse::<usize>().unwrap() - 1;
                    for _ in 0..box_count {
                        let current_box = stack_vec[source_stack].pop().unwrap();
                        stack_vec[current_stack].push(current_box);
                    }
                }
                _ => {
                    panic!("instuctions are more than 3");
                }
            }
        }
    });

    // println!("{:?}", stack_vec);
    // println!("stack size: {stack_size}\nline: '{}'", stack.lines().nth(1).unwrap());

    let mut res = String::new();

    stack_vec.iter().for_each(|s| {
        res.push(s.last().unwrap().clone());
    });

    return res;
}

pub fn p2(input: &str) -> String {
    let stack = input.split("\n\n").nth(0).unwrap();
    let instructions = input.split("\n\n").nth(1).unwrap().trim();
    let stack_size = (stack.lines().nth(0).unwrap().len() + 1) / 4;

    let mut stack_vec: Vec<Vec<char>> = vec![vec![]; stack_size];

    stack
        .lines()
        .filter(|line| !line.trim().is_empty() || line.contains("["))
        .rev()
        .for_each(|line| {
            let mut index = 0;
            for (i, c) in line.chars().enumerate() {
                if (i as i32 - 1) % 4 == 0 {
                    if !c.is_numeric() && c != ' ' {
                        stack_vec[index].push(c);
                    }
                    index += 1;
                }
            }
        });

    // move 3 from 1 to 3
    // instructions.trim().lines().map(|line| line.split(" ").filter(|itm| itm.))
    instructions.trim().lines().for_each(|line| {
        let ins = line
            .split(" ")
            .filter(|itm| match itm.parse::<u32>() {
                Ok(_) => true,
                Err(_) => false,
            })
            .collect::<Vec<_>>();
        // println!("{:?}", ins);

        let mut box_count: u32 = 0;
        let mut source_stack: usize = 0;
        for (i, step) in ins.iter().enumerate() {
            match i {
                // 0 =>
                0 => {
                    // pop
                    box_count = step.parse::<u32>().unwrap();
                }
                1 => {
                    // where
                    source_stack = step.parse::<usize>().unwrap() - 1;
                }
                2 => {
                    let destenation_stack: usize = step.parse::<usize>().unwrap() - 1;
                    let mut pile = (0..box_count)
                        .map(|_| stack_vec[source_stack].pop().unwrap())
                        .collect::<Vec<_>>();

                    (0..box_count).for_each(|_| {
                        stack_vec[destenation_stack].push(pile.pop().unwrap());
                    });
                }
                _ => {
                    panic!("instuctions are more than 3");
                }
            }
        }
    });

    // println!("{:?}", stack_vec);
    // println!("stack size: {stack_size}\nline: '{}'", stack.lines().nth(1).unwrap());

    let mut res = String::new();

    stack_vec.iter().for_each(|s| {
        res.push(s.last().unwrap().clone());
    });

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
";

    #[test]
    fn test_p1() {
        let result = p1(INPUT);
        assert_eq!(result, "CMZ");
    }
    #[test]
    fn test_p2() {
        let result = p2(INPUT);
        assert_eq!(result, "MCD");
    }
}
