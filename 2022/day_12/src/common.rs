/// (dis, r, c)
pub type Point = (usize, usize, usize);

pub type Map = Vec<Vec<char>>;

/// (start, end, Map)
pub fn crete_map(input: &str) -> ((usize, usize), (usize, usize), Map) {
    let mut start_position: (usize, usize) = (0, 0);
    let mut end_position: (usize, usize) = (0, 0);
    let map: Map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'S' {
                        start_position = (r, col);
                        'a'
                    } else if c == 'E' {
                        end_position = (r, col);
                        'z'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();
    return (start_position, end_position, map);
}

/// get all the indexes of a t00
pub fn crete_map_give_a(input: &str) -> (Vec<(usize, usize)>, (usize, usize), Map) {
    let mut start_position: Vec<(usize, usize)> = vec![];
    let mut end_position: (usize, usize) = (0, 0);
    let map: Map = input
        .trim()
        .lines()
        .enumerate()
        .map(|(r, line)| {
            line.chars()
                .enumerate()
                .map(|(col, c)| {
                    if c == 'a' {
                        start_position.push((r, col));
                        c
                    } else if c == 'E' {
                        end_position = (r, col);
                        'z'
                    } else {
                        c
                    }
                })
                .collect()
        })
        .collect();
    return (start_position, end_position, map);
}
