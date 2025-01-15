use std::collections::{HashMap, HashSet};

use utils::coordinates::Point;

fn get_point_diff(point_a: Point<i32>, point_b: Point<i32>) -> Point<i32> {
    if point_a > point_b {
        point_a - point_b
    } else {
        point_b - point_a
    }
}

fn point_within(point: Point<i32>, width: i32, height: i32) -> bool {
    return point.x >= 0 && point.y >= 0 && point.x <= width - 1 && point.y <= height - 1;
}

fn parse_input(input_data: String) -> (HashMap<char, Vec<Point<i32>>>, i32, i32) {
    let lines = input_data
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut map: HashMap<char, Vec<Point<i32>>> = HashMap::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char != '.' {
                map.entry(char)
                    .or_default()
                    .push(Point::new(x as i32, y as i32));
            }
        }
    }

    return (map, width, height);
}

fn part1(input_data: String) -> usize {
    let (map, width, height) = parse_input(input_data);

    let mut antinodes = HashSet::new();

    for value in map.values() {
        for &point_a in value.iter() {
            for &point_b in value.iter().filter(|&&b| b != point_a) {
                let diff = get_point_diff(point_a, point_b);
                let (min, max) = (point_a.min(point_b), point_a.max(point_b));

                let antinode_1 = min - diff;
                let antinode_2 = max + diff;

                if point_within(antinode_1, width, height) {
                    antinodes.insert(antinode_1);
                }
                if point_within(antinode_2, width, height) {
                    antinodes.insert(antinode_2);
                }
            }
        }
    }

    antinodes.len()
}

fn part2(input_data: String) -> usize {
    let (map, width, height) = parse_input(input_data);

    let mut antinodes = HashSet::new();

    for value in map.values() {
        for &point_a in value.iter() {
            for &point_b in value.iter().filter(|&&b| b != point_a) {
                let diff = get_point_diff(point_a, point_b);
                let (min, max) = (point_a.min(point_b), point_a.max(point_b));

                let mut current_point = min;
                while point_within(current_point, width, height) {
                    antinodes.insert(current_point);
                    current_point = current_point - diff;
                }

                let mut current_point = max;
                while point_within(current_point, width, height) {
                    antinodes.insert(current_point);
                    current_point = current_point + diff;
                }
            }
        }
    }

    antinodes.len()
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 14);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 34);
    }
}

fn main() {
    utils::run(8, &["sample1.txt", "input.txt"], &part1, &part2);
}
