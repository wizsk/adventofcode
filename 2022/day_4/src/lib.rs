pub fn p1(input: &str) -> u32 {
    let res = input
        .trim()
        .lines()
        .map(|line| {
            let pair = line.split(",").collect::<Vec<_>>();
            if pair.len() != 2 {
                panic!("pair len isn't 2");
            }

            let p1 = pair[0]
                .split("-")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let p2 = pair[1]
                .split("-")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            // p1:[2, 8] p2:[3, 7]
            let is_sub = p1[0] <= p2[0] && p1[1] >= p2[1];
            // p1:[6, 6] p2:[4, 6]
            let is_sub2 = p1[0] >= p2[0] && p1[1] <= p2[1];
            if is_sub || is_sub2 {
                return 1;
            }
            return 0;
        })
        .sum::<u32>();

    return res;
}

pub fn p2(input: &str) -> u32 {
    let res = input
        .trim()
        .lines()
        .map(|line| {
            let pair = line.split(",").collect::<Vec<_>>();
            if pair.len() != 2 {
                panic!("pair len isn't 2");
            }

            let p1 = pair[0]
                .split("-")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>();
            let p2 = pair[1]
                .split("-")
                .map(|num| num.parse::<u32>().unwrap())
                .collect::<Vec<_>>();

            let mut p1 = p1[0]..p1[1] + 1;
            let p2 = p2[0]..p2[1] + 1;

            let res = match p1.find(|num| p2.contains(&num)) {
                Some(_) => 1,
                None => 0,
            };

            return res;
        })
        .sum::<u32>();

    return res;
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8
";

    #[test]
    fn p1_test() {
        let result = p1(INPUT);
        assert_eq!(result, 2);
    }

    #[test]
    fn p2_test() {
        let result = p2(INPUT);
        assert_eq!(result, 4);
    }
}
