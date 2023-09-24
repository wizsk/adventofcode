pub fn p1(input: &str) -> usize {
    let mut cycles: usize = 0;
    let mut x: i32 = 1;
    let mut singnal_sum: usize = 0;

    for line in input.trim().lines() {
        let ins = line.split_whitespace().collect::<Vec<_>>();
        match ins[0] {
            "noop" => {
                cycles += 1;

                if cycles < 20 {
                    continue;
                } else if cycles == 20 || (cycles - 20) % 40 == 0 {
                    // do suff
                    println!("c {cycles} x {x}");
                    singnal_sum += cycles * x as usize;
                }
            }

            "addx" => {
                for _ in 0..2 {
                    cycles += 1;

                    if cycles < 20 {
                        continue;
                    } else if cycles == 20 || (cycles - 20) % 40 == 0 {
                        // do suff
                        println!("c {cycles} x {x}");
                        singnal_sum += cycles * x as usize;
                    }
                }

                // cycles += 1;
                x += ins[1].parse::<i32>().unwrap();
            }
            _ => panic!("unrecognized ins"),
        }
    }

    return singnal_sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test() {
        let input = "addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop";
        let res = p1(input);
        assert_eq!(res, 13140);
    }
}
