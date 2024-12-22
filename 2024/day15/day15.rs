use std::{
    collections::{HashMap, VecDeque},
    vec,
};
use utils::{
    self,
    coordinates::{Direction, Point},
};

#[derive(Clone, Copy, PartialEq)]
enum Object {
    Wall,
    Empty,
    Box,
    Robot,
    BigBoxLeft,
    BigBoxRight,
}

fn try_movement(
    direction: Point<i32>,
    position: Point<i32>,
    map: &mut HashMap<Point<i32>, Object>,
    apply: bool,
) -> bool {
    let target_position = position + direction;

    match map.get(&(target_position)).unwrap() {
        Object::Empty => {
            if apply {
                map.insert(target_position, map.get(&position).unwrap().clone());
                map.insert(position, Object::Empty);
            }
            true
        }
        Object::Box => {
            if try_movement(direction, target_position, map, true) {
                if apply {
                    map.insert(target_position, map.get(&position).unwrap().clone());
                    map.insert(position, Object::Empty);
                }
                true
            } else {
                false
            }
        }
        Object::Wall => false,
        Object::BigBoxLeft => match direction {
            Direction::UP | Direction::DOWN => {
                if try_movement(direction, target_position, map, apply)
                    && try_movement(direction, target_position + Direction::RIGHT, map, apply)
                {
                    if apply {
                        map.insert(target_position, map.get(&position).unwrap().clone());
                        map.insert(position, Object::Empty);
                    }
                    true
                } else {
                    false
                }
            }
            _ => {
                if try_movement(direction, target_position, map, apply) {
                    if apply {
                        map.insert(target_position, map.get(&position).unwrap().clone());
                        map.insert(position, Object::Empty);
                    }
                    true
                } else {
                    false
                }
            }
        },
        Object::BigBoxRight => match direction {
            Direction::UP | Direction::DOWN => {
                if try_movement(direction, target_position, map, apply)
                    && try_movement(direction, target_position + Direction::LEFT, map, apply)
                {
                    if apply {
                        map.insert(target_position, map.get(&position).unwrap().clone());
                        map.insert(position, Object::Empty);
                    }
                    true
                } else {
                    false
                }
            }
            _ => {
                if try_movement(direction, target_position, map, apply) {
                    if apply {
                        map.insert(target_position, map.get(&position).unwrap().clone());
                        map.insert(position, Object::Empty);
                    }
                    true
                } else {
                    false
                }
            }
        },
        Object::Robot => true,
    }
}

fn part1(input: String) -> i32 {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut moves = moves
        .chars()
        .map(|m| match m {
            '<' => Direction::LEFT,
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            _ => Direction::NONE,
        })
        .collect::<VecDeque<_>>();

    let mut map_grid = HashMap::<Point<i32>, Object>::new();
    let mut robot_pos = Point::<i32> { x: -1, y: -1 };

    for (y, line) in map.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };

            match char {
                '#' => {
                    map_grid.insert(point, Object::Wall);
                }
                '.' => {
                    map_grid.insert(point, Object::Empty);
                }
                'O' => {
                    map_grid.insert(point, Object::Box);
                }
                '@' => {
                    robot_pos = point;
                    map_grid.insert(point, Object::Robot);
                }
                _ => {}
            }
        }
    }

    while !moves.is_empty() {
        let next_move = moves.pop_front().unwrap();
        if try_movement(next_move, robot_pos, &mut map_grid, true) {
            robot_pos += next_move;
        }
    }

    let mut gps = 0;

    for (point, obj) in map_grid.iter() {
        if *obj == Object::Box {
            gps += 100 * point.y + point.x
        }
    }

    gps
}

fn convert_map(map: &str) -> (HashMap<Point<i32>, Object>, Point<i32>) {
    let mut larger_map = HashMap::new();
    let mut robot_pos = Point::<i32> { x: -1, y: -1 };
    for (y, line) in map.lines().enumerate() {
        let mut larger_line = vec![];
        for obj in line.chars() {
            if obj == 'O' {
                larger_line.push('[');
                larger_line.push(']');
            } else if obj == '@' {
                larger_line.push('@');
                larger_line.push('.');
            } else {
                larger_line.push(obj);
                larger_line.push(obj);
            }
        }

        for (x, obj) in larger_line.iter().enumerate() {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };

            match obj {
                '#' => {
                    larger_map.insert(point, Object::Wall);
                }
                '.' => {
                    larger_map.insert(point, Object::Empty);
                }
                '[' => {
                    larger_map.insert(point, Object::BigBoxLeft);
                }
                ']' => {
                    larger_map.insert(point, Object::BigBoxRight);
                }
                '@' => {
                    robot_pos = point;
                    larger_map.insert(point, Object::Robot);
                }
                _ => {}
            }
        }
    }

    (larger_map, robot_pos)
}

fn part2(input: String) -> i32 {
    let (map, moves) = input.split_once("\n\n").unwrap();
    let mut moves = moves
        .chars()
        .map(|m| match m {
            '<' => Direction::LEFT,
            '^' => Direction::UP,
            '>' => Direction::RIGHT,
            'v' => Direction::DOWN,
            _ => Direction::NONE,
        })
        .collect::<VecDeque<_>>();

    let (mut map_grid, mut robot_pos) = convert_map(map);

    while !moves.is_empty() {
        let next_move = moves.pop_front().unwrap();
        if try_movement(next_move, robot_pos, &mut map_grid, false) {
            try_movement(next_move, robot_pos, &mut map_grid, true);
            robot_pos += next_move;
        }
    }

    let mut gps = 0;

    for (point, obj) in map_grid.iter() {
        if *obj == Object::BigBoxLeft
            && *map_grid.get(&(point.clone() + Direction::RIGHT)).unwrap() == Object::BigBoxRight
        {
            gps += point.y * 100 + point.x;
        }
    }

    gps
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(utils::read_file("./sample1.txt")), 2028)
    }

    #[test]
    fn test2() {
        assert_eq!(part1(utils::read_file("./sample2.txt")), 10092)
    }

    #[test]
    fn test3() {
        assert_eq!(part1(utils::read_file("./sample3.txt")), 908)
    }

    #[test]
    fn test4() {
        assert_eq!(part2(utils::read_file("./sample1.txt")), 1751)
    }
    #[test]
    fn test5() {
        assert_eq!(part2(utils::read_file("./sample2.txt")), 9021)
    }

    #[test]
    fn test6() {
        assert_eq!(part2(utils::read_file("./sample3.txt")), 618)
    }
}

fn main() {
    utils::run(
        15,
        vec!["sample1.txt", "sample2.txt", "sample3.txt", "input.txt"],
        &part1,
        &part2,
    );
}
