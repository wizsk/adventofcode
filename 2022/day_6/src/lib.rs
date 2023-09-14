pub fn p1(input: &str) -> usize {
    let mut end: usize;
    for i in 0..input.trim().len() {
        // 4 char msg
        end = i + 4;
        if end > input.len() {
            break;
        }
        if has_unique_chars(&input[i..end]) {
            return end;
        }
    }
    return 0;
}

pub fn p2(input: &str) -> usize {
    let mut end: usize;
    for i in 0..input.trim().len() {
        // 4 char msg
        end = i + 14;
        if end > input.len() {
            break;
        }
        if has_unique_chars(&input[i..end]) {
            return end;
        }
    }
    return 0;
}

fn has_unique_chars(input: &str) -> bool {
    let mut seen = String::new();
    for c in input.chars() {
        if seen.contains(c) {
            return false;
        }
        seen.push(c);
    }
    return true;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_p1() {
        assert!(has_unique_chars("jpqm"));
        assert!(has_unique_chars("abcdefgh"));
        assert!(!has_unique_chars("abcada"));
        assert_eq!(p1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(p1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(p1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(p1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_p2() {
        assert_eq!(p2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(p2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(p2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
