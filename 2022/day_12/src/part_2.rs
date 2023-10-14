use std::collections::HashSet;

use crate::common::*;

pub fn p2(input: &str) -> usize {
    let (_, end, map) = crete_map(input);
    let mut que: Vec<Point> = vec![(0, end.0, end.1)];

    let mut vis: HashSet<(usize, usize)> = HashSet::new();
    vis.insert(end);

    let r_ln = map.len() as i32;
    let c_ln = map.first().unwrap().len() as i32;

    while que.len() > 0 {
        // bum bummmmm
        let (d, r, c) = que.remove(0);
        let r = r as i32;
        let c = c as i32;

        for (nr, nc) in [(r + 1, c), (r, c + 1), (r - 1, c), (r, c - 1)] {
            if nr < 0 || nc < 0 || nr >= r_ln || nc >= c_ln {
                continue;
            }
            let nr = nr as usize;
            let nc = nc as usize;

            if vis.contains(&(nr, nc)) {
                continue;
            }
            if (map[nr][nc] as i32 - map[r as usize][c as usize] as i32) < -1 {
                continue;
            }
            if map[nr][nc] == 'a' {
                // done
                return d + 1;
            }
            vis.insert((nr, nc));
            que.push((d + 1, nr, nc));
        }
    }
    // now how can i do bfs???
    // ok lets do a recursive search!

    0
}

/*
fn walk(curr: Point, map: &Map) {
    // base case
    if map[curr.1][curr.0] == 'z' {
        return;
    }

    let x_len = map.first().unwrap().len();
    let y_len = map.len();

}
*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_2() {
        assert_eq!(p2(crate::_TEST_INPUT), 29);
    }
}
