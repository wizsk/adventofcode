pub fn calculate_m_h_w(input: &str) -> (usize, usize) {
    // can be - (negative) while calculating
    let mut hight: i32 = 0;
    let mut width: i32 = 0;

    for line in input.trim().lines() {
        let cmd = line.split(" ").collect::<Vec<_>>();
        if cmd.len() != 2 {
            panic!("while parsing file cmd.len != 2");
        }

        let ins = cmd[0];
        let step = cmd[1].parse::<i32>().unwrap();

        match ins {
            "U" => hight += step,
            "D" => hight -= step,
            "R" => width += step,
            "L" => width -= step,
            _ => {}
        }
    }

    (hight as usize, width as usize)
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
        let result = calculate_m_h_w(INPUT);
        assert_eq!(result, (2, 2));
    }
}
