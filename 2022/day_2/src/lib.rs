// rock papaer seasor score
pub fn p1(inpt: &str) -> i32 {
    let res = inpt
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let m = line.split(" ").collect::<Vec<&str>>();
            mach_score(&m).unwrap()
        })
        .sum::<i32>();

    return res;
}

pub fn p2(inpt: &str) -> i32 {
    let res = inpt
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| {
            let m = line.split(" ").collect::<Vec<&str>>();
            get_move_score(&m).unwrap()
        })
        .sum::<i32>();

    return res;
}

// just tranlates the move into number
fn score(m: &str) -> Option<i32> {
    match m {
        "A" | "X" => Some(1),
        "B" | "Y" => Some(2),
        "C" | "Z" => Some(3),
        _ => None,
    }
}

// this is used by p1 to calculate the score
fn mach_score(mach: &Vec<&str>) -> Option<i32> {
    if mach.len() != 2 {
        return None;
    }

    let o = score(mach[0]).unwrap();
    let m = score(mach[1]).unwrap();

    // draw
    if o == m {
        return Some(3 + m);
    } else if o == 1 && m == 2 || o == 2 && m == 3 || o == 3 && m == 1 {
        // i win ig
        return Some(6 + m);
    }

    // or I lose :)
    // just get the points for drawing
    return Some(m);
}

// used by p2 for caculate the score
fn get_move_score(inpt: &Vec<&str>) -> Option<i32> {
    if inpt.len() != 2 {
        return None;
    }

    let mut o = score(inpt[0]).unwrap();
    let m = score(inpt[1]).unwrap();

    match m {
        // lose move "X"
        1 => {
            o -= 1;
            if o < 1 {
                o = 3;
            }
            Some(o)
        }
        // Draw move "Y"
        2 => Some(o + 3),
        3 => {
            o += 1;
            if o > 3 {
                o = 1;
            }
            Some(o + 6)
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "A Y
B X
C Z
";

    #[test]
    fn test_p1() {
        let scr: i32 = 15;
        let result = p1(INPUT);
        assert_eq!(result, scr);
    }

    #[test]
    fn test_p2() {
        let scr: i32 = 12;
        let result = p2(INPUT);
        assert_eq!(result, scr);
    }
}
