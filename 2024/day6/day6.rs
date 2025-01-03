use std::collections::HashSet;

const ROTATIONS: [(i32, i32); 4] = [
    (-1, 0), /* UP    */
    (0, 1),  /* RIGHT */
    (1, 0),  /* DOWN  */
    (0, -1), /* LEFT  */
];

fn next_position(
    lines: &[Vec<char>],
    direction: (i32, i32),
    position: (i32, i32),
) -> Option<(i32, i32)> {
    let (mut y, mut x) = position;
    y += direction.0;
    x += direction.1;

    if y < 0 || y as usize >= lines.len() || x < 0 || x as usize >= lines[y as usize].len() {
        return None;
    }

    Some((y, x))
}

fn find_loop(lines: &[Vec<char>], start: (i32, i32)) -> bool {
    let mut visited = HashSet::new();
    let mut iter = ROTATIONS.iter().cycle();
    let mut dir = iter.next().unwrap();
    let mut pos = start;

    loop {
        if let Some(new_pos) = next_position(lines, *dir, pos) {
            if lines[new_pos.0 as usize][new_pos.1 as usize] == '#' {
                if visited.contains(&(pos, *dir)) {
                    return true;
                }
                visited.insert((pos, *dir));
                dir = iter.next().unwrap();
                continue;
            }
            pos = new_pos;
        } else {
            return false;
        }
    }
}

fn part1(input_data: String) -> i32 {
    let mut start: (i32, i32) = (0, 0);
    let input_lines = input_data.lines().collect::<Vec<_>>();

    let lines: Vec<Vec<char>> = input_lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let mut line = line.to_string();
            if let Some(x) = line.find('^') {
                start = (y as i32, x as i32);
                line = line.replace('^', ".");
            }
            line.chars().collect()
        })
        .collect();

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut iter = ROTATIONS.iter().cycle();
    let mut direction = iter.next().unwrap();
    let mut pos = start;

    loop {
        if let Some(next_pos) = next_position(&lines, *direction, pos) {
            if lines[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                direction = iter.next().unwrap();
                continue;
            }
            pos = next_pos;
            visited.insert(pos);
        } else {
            break;
        }
    }

    visited.len() as i32
}

fn part2(input_data: String) -> i32 {
    let mut start: (i32, i32) = (0, 0);
    let input_lines = input_data.lines().collect::<Vec<_>>();

    let mut lines: Vec<Vec<char>> = input_lines
        .iter()
        .enumerate()
        .map(|(y, line)| {
            let mut line = line.to_string();
            if let Some(x) = line.find('^') {
                start = (y as i32, x as i32);
                line = line.replace('^', ".");
            }
            line.chars().collect()
        })
        .collect();

    let mut visited = HashSet::new();
    visited.insert(start);

    let mut iter = ROTATIONS.iter().cycle();
    let mut direction = iter.next().unwrap();
    let mut pos = start;

    loop {
        if let Some(next_pos) = next_position(&lines, *direction, pos) {
            if lines[next_pos.0 as usize][next_pos.1 as usize] == '#' {
                direction = iter.next().unwrap();
                continue;
            }
            pos = next_pos;
            visited.insert(pos);
        } else {
            break;
        }
    }

    let mut part2 = 0;

    for (y, x) in visited {
        if (y, x) == start {
            continue;
        }
        let y = y as usize;
        let x = x as usize;

        let c = lines[y][x];
        if c == '#' {
            continue;
        }
        lines[y][x] = '#';

        if find_loop(&lines, start) {
            part2 += 1;
        }

        lines[y][x] = c;
    }

    part2
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 41);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 6);
    }
}

fn main() {
    utils::run(6, &["sample1.txt", "input.txt"], &part1, &part2);
}
