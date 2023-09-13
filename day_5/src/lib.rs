pub fn p1(input: &str) -> String {
    let stack = input.split("\n\n").nth(0).unwrap();
    let instructions = input.split("\n\n").nth(1).unwrap().trim();
    let stack_size = (stack.lines().nth(0).unwrap().len() + 1) / 4;

    let mut stack_vec: Vec<Vec<char>> = vec![vec![]; stack_size];

    let _ = stack
        .lines()
        .filter(|line| !line.trim().is_empty() || line.contains("["))
        .rev()
        .map(|line| {
            println!("line: '{line}'");

            let mut index = 0;
            for (i, c) in line.chars().enumerate() {
                if (i as i32 - 1) % 4 == 0 {
                    if !c.is_numeric() && c != ' ' {
                        stack_vec[index].push(c);
                    }
                    index += 1;
                }
            }
            1
        })
        .sum::<u64>();

    // let _ = instructions.trim().lines().map(|line| {
    //     line.split(" ").filter(|itm| {
    //         match itm.parse::<u32>() {
    //         Ok(num) => ,
    //         Err(_) => {},
    //     }})
    // })

    // println!("{:?}", stack_vec);
    // println!("stack size: {stack_size}\nline: '{}'", stack.lines().nth(1).unwrap());

    "fo".to_string()
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
    fn it_works() {
        let result = p1(INPUT);
        assert_eq!(result, "CMZ");
    }
}
