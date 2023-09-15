// nice
trait Forest {
    fn witdh(&self) -> usize;
    fn height(&self) -> usize;
    fn is_ege(&self, r: &usize, c: &usize) -> bool;
    fn is_seeable(&self, r: &usize, c: &usize) -> bool;
    fn seeable_tree_count(&self, r: &usize, c: &usize) -> usize;
}

// printing red :)
/*
fn print<T>(item: T)
where
    T: std::fmt::Display,
{
    let red = "\x1B[31m";
    let reset = "\x1B[0m";
    print!("{red}{}{reset}", item);
}
*/

impl Forest for Vec<Vec<u8>> {
    fn witdh(&self) -> usize {
        self.first().unwrap().len()
    }

    fn height(&self) -> usize {
        self.len()
    }

    // row and collumn will be less by 1
    fn is_ege(&self, r: &usize, c: &usize) -> bool {
        *r == 0 || *c == 0 || self.witdh() == *c + 1 || self.height() == *r + 1
    }

    fn seeable_tree_count(&self, r: &usize, c: &usize) -> usize {
        if self.is_ege(r, c) {
            return 0;
        }

        let current = self[*r][*c];

        let mut left: usize = 0;
        for i in self[*r].iter().take(*c).rev() {
            left += 1;
            if *i >= current {
                break;
            }
        }

        let mut right: usize = 0;
        for i in (*c + 1)..self.witdh() {
            right += 1;
            if self[*r][i] >= current {
                break;
            }
        }

        let mut up: usize = 0;
        for rw in (0..*r).rev() {
            up += 1;
            if self[rw][*c] >= current {
                break;
            }
        }

        let mut down: usize = 0;
        for rw in (*r + 1)..self.len() {
            down += 1;
            if self[rw][*c] >= current {
                break;
            }
        }

        println!(
            "[{r}][{c}]:{} l{left} r{right} u{up} d{down}:: {}",
            current,
            (left * right * up * down)
        );

        return left * right * up * down;
    }

    fn is_seeable(&self, r: &usize, c: &usize) -> bool {
        if self.is_ege(r, c) {
            return true;
        }

        let current = self[*r][*c];
        let mut left: bool = false;
        for i in self[*r].iter().take(*c).rev() {
            if *i >= current {
                left = true;
                break;
            }
        }

        let mut right: bool = false;
        for i in (*c + 1)..self.witdh() {
            if self[*r][i] >= current {
                right = true;
                break;
            }
        }

        let mut up: bool = false;
        for rw in (0..*r).rev() {
            if self[rw][*c] >= current {
                up = true;
                break;
            }
        }

        let mut down: bool = false;
        for rw in (*r + 1)..self.len() {
            if self[rw][*c] >= current {
                down = true;
                break;
            }
        }

        // println!("{} {}", current, !(left && right && up && down));

        return !(left && right && up && down);
    }
}

pub fn p1(input: &str) -> usize {
    // println!("{input}");
    let forest = parse_data(input);
    forest
        .iter()
        .enumerate()
        .map(|(r, row)| {
            // println!();
            // println!("line: {}", r + 1);
            row.iter()
                .enumerate()
                .map(|(c, _)| {
                    if forest.is_seeable(&r, &c) {
                        // print(ch);
                        return 1;
                    }
                    // print!("{ch}");
                    return 0;
                })
                .sum::<usize>()
        })
        .sum::<usize>()
}

pub fn p2(input: &str) -> usize {
    // println!("{input}");
    let forest = parse_data(input);
    forest
        .iter()
        .enumerate()
        .map(|(r, row)| {
            // println!();
            row.iter()
                .enumerate()
                .map(|(c, _)| forest.seeable_tree_count(&r, &c))
                .max()
                .unwrap()
        })
        .max()
        .unwrap()
}

fn parse_data(input: &str) -> Vec<Vec<u8>> {
    input
        .trim()
        .lines()
        .map(|line| {
            line.trim()
                .chars()
                .map(|tree| tree.to_digit(10).unwrap() as u8)
                .collect::<Vec<_>>()
        })
        .collect::<Vec<Vec<u8>>>()
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &'static str = "30373
25512
65332
33549
35390
";

    #[test]
    fn test_p1() {
        let result = p1(INPUT);
        assert_eq!(result, 21);
    }

    #[test]
    fn test_p2() {
        let result = p2(INPUT);
        assert_eq!(result, 8);
    }
}
