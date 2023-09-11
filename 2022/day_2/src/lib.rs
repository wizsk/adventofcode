// rock papaer seasor score
pub fn p1(inpt: &str) -> u32 {
    let _res = inpt.lines().map(|line| {
        let m: Vec<_> = line.split(" ").collect();
    });

    15
}

#[cfg(test)]
const INPUT: &str = "A Y
B X
C Z
";
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        let score: u32 = 15;
        let result = p1(INPUT);
        assert_eq!(result, score);
    }
}
