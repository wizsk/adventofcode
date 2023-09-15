pub fn p1(input: &str) -> usize {
    let res = directory_sizes(input)
        .iter()
        .map(|val| {
            if val.le(&100000) {
                return *val;
            }
            return 0;
        })
        .sum::<usize>();

    return res;
}

pub fn p2(input: &str) -> usize {
    let filesystem_size: usize = 70000000;
    let update_size: usize = 30000000;

    let filesystem = directory_sizes(input);

    let root_size = filesystem.last().unwrap();
    let storage_needed = update_size - (filesystem_size - root_size);

    // best candiate for dilation
    let mut best_candidate: usize = 0;
    let mut smallest_diviation: usize = 0;
    let mut run_1 = true;

    for dir_size in filesystem[..filesystem.len() - 1].iter() {
        if dir_size.gt(&storage_needed) {
            let diviation = dir_size - storage_needed;
            if run_1 || diviation.lt(&smallest_diviation) {
                smallest_diviation = diviation;
                best_candidate = *dir_size;
                run_1 = false;
            }
        }
    }

    best_candidate
}

// the last item is the 1st item btw
fn directory_sizes(input: &str) -> Vec<usize> {
    let mut stack: Vec<usize> = vec![];
    let mut read: Vec<usize> = vec![];

    for line in input.trim().lines() {
        if line == "$ ls" {
            continue;
        } else if line == "$ cd .." {
            let tmp = stack.pop().unwrap();
            *stack.last_mut().unwrap() += tmp;
            read.push(tmp);
        } else if line.contains("$ cd") {
            stack.push(0);
        } else {
            match line.split(" ").nth(0).unwrap().parse::<usize>() {
                Ok(num) => *stack.last_mut().unwrap() += num,
                Err(_) => continue,
            }
        }
    }

    // remaing directories
    for i in (0..stack.len()).rev() {
        if i == 0 {
            read.push(stack.pop().unwrap());
            break;
        }
        let tmp = stack.pop().unwrap();
        *stack.last_mut().unwrap() += tmp;
        read.push(tmp);
    }

    return read;
}

// this time test passed but the actual input didn't
#[cfg(test)]
mod tests {
    use super::*;
    const INPUT: &str = "$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k
";
    // ---Part One---
    // 1307902
    // ---Part Two---
    // 7068748

    // #[test]
    // fn test_parse_cmd() {
    //     assert_eq!(p1(INPUT), 1);
    // }
    //
    #[test]
    fn test_p2() {
        assert_eq!(p2(INPUT), 24933642);
    }
}
