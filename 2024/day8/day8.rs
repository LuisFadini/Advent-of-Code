use std::collections::{HashMap, HashSet};

fn get_point_diff(point_a: (i32, i32), point_b: (i32, i32)) -> (i32, i32) {
    let (a, b) = if point_a > point_b {
        (point_b, point_a)
    } else {
        (point_a, point_b)
    };
    return sub_point(b, a);
}

fn sub_point(point_a: (i32, i32), point_b: (i32, i32)) -> (i32, i32) {
    return (point_a.0 - point_b.0, point_a.1 - point_b.1);
}

fn add_point(point_a: (i32, i32), point_b: (i32, i32)) -> (i32, i32) {
    return (point_a.0 + point_b.0, point_a.1 + point_b.1);
}

fn point_within(point: (i32, i32), width: i32, height: i32) -> bool {
    return point.0 >= 0 && point.1 >= 0 && point.0 <= width - 1 && point.1 <= height - 1;
}

fn parse_input(input_data: String) -> (HashMap<char, Vec<(i32, i32)>>, i32, i32) {
    let lines = input_data
        .lines()
        .filter(|l| !l.is_empty())
        .collect::<Vec<_>>();

    let width = lines[0].len() as i32;
    let height = lines.len() as i32;

    let mut map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    let mut points = HashSet::new();

    for (y, line) in lines.iter().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }
            let point: (i32, i32) = (x as i32, y as i32);
            map.entry(char).or_default().push(point);
            points.insert(point);
        }
    }

    return (map, width, height);
}

fn part1(input_data: String) -> i32 {
    let (map, width, height) = parse_input(input_data);

    let mut antinodes = HashSet::new();

    for value in map.values() {
        for &point_a in value.iter() {
            for &point_b in value.iter().filter(|&&b| b != point_a) {
                let diff = get_point_diff(point_a, point_b);
                let (min, max) = (point_a.min(point_b), point_a.max(point_b));

                let antinode_1 = sub_point(min, diff);
                let antinode_2 = add_point(max, diff);

                if point_within(antinode_1, width, height) {
                    antinodes.insert(antinode_1);
                }
                if point_within(antinode_2, width, height) {
                    antinodes.insert(antinode_2);
                }
            }
        }
    }

    antinodes.len() as i32
}

fn part2(input_data: String) -> i32 {
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
                    current_point = sub_point(current_point, diff);
                }

                let mut current_point = max;
                while point_within(current_point, width, height) {
                    antinodes.insert(current_point);
                    current_point = add_point(current_point, diff);
                }
            }
        }
    }

    antinodes.len() as i32
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
