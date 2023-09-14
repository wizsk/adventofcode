/*
 * recursive?
 * or non recursive?
 * which one to follow????
 */

// I think I sould ememnt a data structure for the ??>>
#[derive(Debug)]
struct Dir {
    dir_name: String,
    size: usize,
}

fn new_dir(name: &str) -> Dir {
    Dir {
        // parent_dir: *Dir;
        dir_name: name.to_string(),
        size: 0,
    }
}

pub fn p1(input: &str) -> usize {
    let mut stack: Vec<Dir> = vec![];
    let mut read: Vec<Dir> = vec![];

    for line in input.trim().lines() {
        // println!("{}\n{:#?}", line, root);
        if line.contains("$ ls") {
            continue;
        }

        if line.contains("cd") {
            if line.contains("..") {
                let tmp = stack.pop().unwrap();
                stack.last_mut().unwrap().size += tmp.size;
                read.push(tmp);
                continue;
            }

            let curr_dir_name = line.split(" ").last().unwrap();
            stack.push(new_dir(curr_dir_name));
        }

        match line.split(" ").nth(0).unwrap().parse::<usize>() {
            Ok(num) => stack.last_mut().unwrap().size += num,
            Err(_) => continue,
        }
    }
    println!("stack: {:#?}", stack);

    for i in (0..stack.len()).rev() {
        if i == 0 {
            read.push(stack.pop().unwrap());
            break;
        }
        let tmp = stack.pop().unwrap();
        stack.last_mut().unwrap().size += tmp.size;
        read.push(tmp);
    }

    // println!("read: {:#?}", read);

    let res = read
        .iter()
        .map(|val| {
            if val.size >= 100000 {
                return 1;
            }
            return 0;
        })
        .sum::<usize>();

    return res;
}

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

    #[test]
    fn test_parse_cmd() {
        assert_eq!(p1(INPUT), 1);

    }
}
