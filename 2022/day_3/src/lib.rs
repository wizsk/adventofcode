use std::vec;

pub fn p1_sum(inpt: &str) -> u32 {
    let sum: u32 = inpt
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let (s1, s2) = line.split_at(line.len() / 2);
            let c = s1.chars().find(|c| s2.contains(*c)).unwrap();
            val_from_char(&c).unwrap()
        })
        .sum::<u32>();

    return sum;
}

pub fn p2_group_sum(inpt: &str) -> u32 {
    // let chars1 = inpt.lines().chain(other)
    // let sum: u32 = inpt
    // .lines()
    // .filter(|line| !line.trim().is_empty())
    // .map(|line| {
    //     let (s1, s2) = line.split_at(line.len() / 2);
    //     let c = s1.chars().find(|c| s2.contains(*c)).unwrap();
    //     val_from_char(&c).unwrap()
    // })
    // .sum::<u32>();
    // return sum;

    // let x = inpt
    //     .splitn(3, "\n")
    //     .collect::<Vec<_>>();

    let mut sum: u32 = 0;
    let mut line_count: u32 = 0;
    let mut lines: Vec<&str> = vec![];
    for v in inpt.lines() {
        // current line count
        line_count += 1;
        lines.push(v.trim());

        if line_count == 3 {
            // hack :) hehe
            let l1 = lines[0];
            let l2 = lines[1];
            let l3 = lines[2];
            let l2_and_l2 = l1.chars().filter(|c| l2.contains(*c)).collect::<Vec<_>>();
            let filan = l3
                .chars()
                .filter(|c| l2_and_l2.contains(c))
                .collect::<Vec<_>>();

            // probably never will happend but still....
            if filan.len() == 0 {
                panic!("final can't len be zerooooo")
            }

            sum += val_from_char(&filan[0]).unwrap();

            // rest stuff
            line_count = 0;
            lines.clear();
        }
    }

    return sum;
}

fn val_from_char(c: &char) -> Option<u32> {
    // uppercase a, z
    let u_a = 'A' as u32;
    let u_z = 'Z' as u32;

    let a = 'a' as u32;
    let z = 'z' as u32;

    let c = *c as u32;

    if c >= u_a && c <= u_z {
        return Some(c - u_a + 1 + 26);
    } else if c >= a && c <= z {
        return Some(c - a + 1);
    }

    return None;
}

pub const TEST_INPT: &str = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1_sum() {
        let result = p1_sum(TEST_INPT);
        assert_eq!(result, 157);
    }

    #[test]
    fn test_p2_sum() {
        let res = p2_group_sum(TEST_INPT);
        assert_eq!(res, 70);
    }
}
