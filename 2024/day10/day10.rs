use std::collections::VecDeque;

const DIRECTIONS: [(i32, i32); 4] = [
    (-1, 0), /* UP    */
    (0, 1),  /* RIGHT */
    (1, 0),  /* DOWN  */
    (0, -1), /* LEFT  */
];

fn get_char(lines: &Vec<&str>, position: (i32, i32)) -> Option<char> {
    lines
        .get(position.0 as usize)
        .and_then(|line| line.chars().nth(position.1 as usize))
}

fn bfs(lines: &Vec<&str>, start: (i32, i32)) -> i32 {
    let mut queue = VecDeque::new();
    let mut visited = vec![start];
    let mut score = 0;

    queue.push_back(start);

    while let Some(position) = queue.pop_front() {
        for direction in DIRECTIONS {
            let next_position = (position.0 + direction.0, position.1 + direction.1);

            if visited.contains(&next_position) {
                continue;
            }

            if let Some(next_char) = get_char(&lines, next_position) {
                let current_height = get_char(&lines, position).unwrap().to_digit(10).unwrap();
                let next_height = next_char.to_digit(10).unwrap();

                if next_height == current_height + 1 {
                    visited.push(next_position);

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
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let lines_c = lines.clone();
            line.chars().enumerate().map(move |(x, char)| {
                if char == '0' {
                    bfs(&lines_c, (y as i32, x as i32))
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
        .clone()
        .iter()
        .enumerate()
        .flat_map(|(y, line)| {
            let lines_c = lines.clone();
            line.chars().enumerate().map(move |(x, char)| {
                let mut paths = 0;
                if char == '0' {
                    let mut queue = VecDeque::new();

                    queue.push_back((y as i32, x as i32));

                    while let Some(position) = queue.pop_front() {
                        for direction in DIRECTIONS {
                            let next_position =
                                (position.0 + direction.0, position.1 + direction.1);

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
