use utils::coordinates::Point;

fn get_char(y: usize, x: usize, map: &Vec<&str>) -> char {
    map.get(y).unwrap().chars().nth(x).unwrap()
}

fn coord_to_index(point: Point<usize>, width: usize) -> usize {
    point.y * width + point.x
}

fn is_same_area(
    lines: &Vec<&str>,
    area_char: char,
    point: Point<usize>,
    width: usize,
    height: usize,
) -> bool {
    point.x < width && point.y < height && get_char(point.y, point.x, &lines) == area_char
}

fn solve(input_map: String, is_part2: bool) -> i32 {
    let lines = input_map.lines().collect::<Vec<_>>();
    let width = lines[0].len();
    let height = lines.len();

    let mut checked = vec![false; lines.len() * width];
    let mut to_check = Vec::new();

    let mut total_price = 0;

    for start_y in 0..height {
        for start_x in 0..width {
            let start = Point::new(start_x, start_y);
            if checked[coord_to_index(start, width)] {
                continue;
            }

            let mut area = 0;
            let mut perimeter = 0;
            let mut corners = 0;
            let area_char = get_char(start_y, start_x, &lines.clone());

            to_check.push(start);
            checked[coord_to_index(start, width)] = true;

            while let Some(Point { x, y }) = to_check.pop() {
                area += 1;

                let adjacent = [
                    Point::new(x.wrapping_sub(1), y),
                    Point::new(x, y.wrapping_sub(1)),
                    Point::new(x + 1, y),
                    Point::new(x, y + 1),
                ];

                let same_adjacent = adjacent
                    .iter()
                    .map(|&pos| is_same_area(&lines, area_char, pos, width, height))
                    .collect::<Vec<bool>>();

                let adjacent_diagonally = [
                    Point::new(x.wrapping_sub(1), y.wrapping_sub(1)),
                    Point::new(x + 1, y.wrapping_sub(1)),
                    Point::new(x + 1, y + 1),
                    Point::new(x.wrapping_sub(1), y + 1),
                    // [x.wrapping_sub(1), y.wrapping_sub(1)],
                    // [x + 1, y.wrapping_sub(1)],
                    // [x + 1, y + 1],
                    // [x.wrapping_sub(1), y + 1],
                ];

                if is_part2 {
                    for i in 0..4 {
                        if !same_adjacent[i] && !same_adjacent[(i + 1) % 4] {
                            corners += 1;
                        } else if same_adjacent[i]
                            && same_adjacent[(i + 1) % 4]
                            && !is_same_area(
                                &lines,
                                area_char,
                                adjacent_diagonally[i],
                                width,
                                height,
                            )
                        {
                            corners += 1;
                        }
                    }
                }

                for (pos, same) in adjacent.iter().zip(same_adjacent.iter()) {
                    if !same {
                        perimeter += 1;
                    } else if !checked[coord_to_index(*pos, width)] {
                        to_check.push(*pos);
                        checked[coord_to_index(*pos, width)] = true;
                    }
                }
            }

            total_price += area * if is_part2 { corners } else { perimeter };
        }
    }

    total_price
}

fn part1(input_map: String) -> i32 {
    solve(input_map, false)
}

fn part2(input_map: String) -> i32 {
    solve(input_map, true)
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 140);
    }

    #[test]
    fn test2() {
        assert_eq!(part1(read_file("./sample2.txt")), 1930);
    }

    #[test]
    fn test3() {
        assert_eq!(part2(read_file("./sample1.txt")), 80);
    }

    #[test]
    fn test4() {
        assert_eq!(part2(read_file("./sample2.txt")), 236);
    }
}

fn main() {
    utils::run(
        12,
        &["sample1.txt", "sample2.txt", "sample3.txt", "input.txt"],
        &part1,
        &part2,
    );
}
