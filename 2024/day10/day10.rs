use std::collections::{HashSet, VecDeque};

use utils::coordinates::{Direction, Point};

fn get_char(lines: &Vec<&str>, position: Point<i32>) -> Option<char> {
    lines
        .get(position.y as usize)
        .and_then(|line| line.chars().nth(position.x as usize))
}

fn bfs(lines: &Vec<&str>, start: Point<i32>) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut score = 0;

    queue.push_back(start);
    visited.insert(start);

    while let Some(position) = queue.pop_front() {
        for direction in Direction::CARDINAL {
            let next_position = position + direction;

            if visited.contains(&next_position) {
                continue;
            }

            if let Some(next_char) = get_char(&lines, next_position) {
                let current_height = get_char(&lines, position).unwrap().to_digit(10).unwrap();
                let next_height = next_char.to_digit(10).unwrap();

                if next_height == current_height + 1 {
                    visited.insert(next_position);

                    if next_height == 9 {
                        score += 1;
                    } else {
                        queue.push_back(next_position);
                    }
                }
            }
        }
    }

    score
}

fn part1(input_data: String) -> i32 {
    let lines: Vec<&str> = input_data.lines().collect();

    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let lines_c = lines.clone();
            line.chars().enumerate().map(move |(x, char)| {
                if char == '0' {
                    bfs(&lines_c, Point::new(x as i32, y as i32))
                } else {
                    0
                }
            })
        })
        .sum()
}

fn part2(input_data: String) -> i32 {
    let lines: Vec<&str> = input_data.lines().collect();

    lines
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let lines_c = lines.clone();
            line.chars().enumerate().map(move |(x, char)| {
                let mut paths = 0;
                if char == '0' {
                    let mut queue = VecDeque::new();

                    queue.push_back(Point::new(x as i32, y as i32));

                    while let Some(position) = queue.pop_front() {
                        for direction in Direction::CARDINAL {
                            let next_position = position + direction;

                            if let Some(next_char) = get_char(&lines_c, next_position) {
                                let current_height =
                                    get_char(&lines_c, position).unwrap().to_digit(10).unwrap();
                                let next_height = next_char.to_digit(10).unwrap();

                                if next_height == current_height + 1 {
                                    if next_height == 9 {
                                        paths += 1;
                                    } else {
                                        queue.push_back(next_position);
                                    }
                                }
                            }
                        }
                    }
                }
                paths
            })
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 36);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 81);
    }
}

fn main() {
    utils::run(10, &["sample1.txt", "input.txt"], &part1, &part2);
}
