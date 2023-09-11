pub fn p1_find_greatest_cal(inpt: &str) -> String {
    let res = inpt
        .split("\n\n")
        .map(|elf_load| {
            elf_load
                .lines()
                .map(|itm| itm.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string();
    return res;
}

pub fn p2_find_top_3(input: &str) -> u32 {
    let mut o: Vec<u32> = input
        .split("\n\n")
        .map(|ele_cal| {
            ele_cal
                .lines()
                .map(|val| val.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<u32>>(); // u may use "_" rether than "u32"

    o.sort_by(|a, b| b.cmp(a));

    o.iter().take(3).sum()

}

#[cfg(test)]
const INPUT: &str = "1000
2000
3000

4000

5000
6000

7000
8000
9000

10000";

// -> 24000
// -> 11000
// -> 10000
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn p1() {
        assert_eq!(p1_find_greatest_cal(INPUT), "24000");
    }

    #[test]
    fn p2() {
        let top3 = 45000;
        assert_eq!(p2_find_top_3(INPUT), top3);
    }
}
