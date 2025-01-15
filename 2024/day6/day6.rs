use std::collections::HashSet;

use utils::coordinates::{Direction, Point};

fn next_position(
    lines: &[Vec<char>],
    direction: Point<i32>,
    position: Point<i32>,
) -> Option<Point<i32>> {
    let Point { x, y } = position + direction;

    if y < 0 || y as usize >= lines.len() || x < 0 || x as usize >= lines[y as usize].len() {
        return None;
    }

    Some(Point::new(x, y))
}

fn parse_input(input: String) -> (Point<i32>, Vec<Vec<char>>) {
    let mut start = Point::new(0, 0);

    let lines: Vec<Vec<char>> = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            let mut line = line.to_string();
            if let Some(x) = line.find('^') {
                start = Point::new(x as i32, y as i32);
                line = line.replace('^', ".");
            }
            line.chars().collect()
        })
        .collect();

    (start, lines)
}

fn run(start: Point<i32>, lines: Vec<Vec<char>>) -> HashSet<Point<i32>> {
    let mut visited = HashSet::new();
    visited.insert(start);

    let mut iter = Direction::CARDINAL.iter().cycle();
    let mut direction = iter.next().unwrap();
    let mut pos = start;

    loop {
        if let Some(next_pos) = next_position(&lines, *direction, pos) {
            if lines[next_pos.y as usize][next_pos.x as usize] == '#' {
                direction = iter.next().unwrap();
                continue;
            }
            pos = next_pos;
            visited.insert(pos);
        } else {
            break;
        }
    }

    visited
}

fn part1(input_data: String) -> usize {
    let (start, lines) = parse_input(input_data);
    let visited = run(start, lines);

    visited.len()
}

fn find_loop(lines: &[Vec<char>], start: Point<i32>) -> bool {
    let mut visited = HashSet::new();
    let mut iter = Direction::CARDINAL.iter().cycle();
    let mut dir = iter.next().unwrap();
    let mut pos = start;

    loop {
        if let Some(new_pos) = next_position(lines, *dir, pos) {
            if lines[new_pos.y as usize][new_pos.x as usize] == '#' {
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

fn part2(input_data: String) -> i32 {
    let (start, mut lines) = parse_input(input_data);
    let visited = run(start, lines.clone());

    let mut part2 = 0;

    for visited_point in visited {
        if visited_point == start {
            continue;
        }

        let Point { x, y } = visited_point;
        let x = x as usize;
        let y = y as usize;

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
