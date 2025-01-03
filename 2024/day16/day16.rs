use std::{
    cmp::Reverse,
    collections::{BinaryHeap, HashMap, HashSet},
};

use utils::coordinates::{self, Point};

#[derive(PartialEq, Clone)]
enum Tiles {
    Start,
    Empty,
    End,
    Wall,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

#[derive(Eq, Ord, PartialEq, PartialOrd)]
struct Location {
    point: Point<i32>,
    cost: i32,
    direction: Direction,
    current_path: Vec<Point<i32>>,
}

impl Location {
    fn new(
        point: Point<i32>,
        current_cost: i32,
        direction: Direction,
        current_path: Vec<Point<i32>>,
    ) -> Location {
        Location {
            point,
            cost: current_cost,
            direction,
            current_path,
        }
    }
}

fn get_next_locations(location: &Location) -> Vec<Location> {
    let mut up_path = location.current_path.clone();
    let mut down_path = location.current_path.clone();
    let mut right_path = location.current_path.clone();
    let mut left_path = location.current_path.clone();

    up_path.push(location.point + coordinates::Direction::UP);
    down_path.push(location.point + coordinates::Direction::DOWN);
    left_path.push(location.point + coordinates::Direction::LEFT);
    right_path.push(location.point + coordinates::Direction::RIGHT);

    match location.direction {
        Direction::UP => vec![
            Location::new(
                location.point + coordinates::Direction::UP,
                location.cost + 1,
                Direction::UP,
                up_path,
            ),
            Location::new(
                location.point + coordinates::Direction::LEFT,
                location.cost + 1001,
                Direction::LEFT,
                left_path,
            ),
            Location::new(
                location.point + coordinates::Direction::RIGHT,
                location.cost + 1001,
                Direction::RIGHT,
                right_path,
            ),
        ],
        Direction::DOWN => vec![
            Location::new(
                location.point + coordinates::Direction::DOWN,
                location.cost + 1,
                Direction::DOWN,
                down_path,
            ),
            Location::new(
                location.point + coordinates::Direction::LEFT,
                location.cost + 1001,
                Direction::LEFT,
                left_path,
            ),
            Location::new(
                location.point + coordinates::Direction::RIGHT,
                location.cost + 1001,
                Direction::RIGHT,
                right_path,
            ),
        ],
        Direction::LEFT => vec![
            Location::new(
                location.point + coordinates::Direction::UP,
                location.cost + 1001,
                Direction::UP,
                up_path,
            ),
            Location::new(
                location.point + coordinates::Direction::DOWN,
                location.cost + 1001,
                Direction::DOWN,
                down_path,
            ),
            Location::new(
                location.point + coordinates::Direction::LEFT,
                location.cost + 1,
                Direction::LEFT,
                left_path,
            ),
        ],
        Direction::RIGHT => vec![
            Location::new(
                location.point + coordinates::Direction::UP,
                location.cost + 1001,
                Direction::UP,
                up_path,
            ),
            Location::new(
                location.point + coordinates::Direction::DOWN,
                location.cost + 1001,
                Direction::DOWN,
                down_path,
            ),
            Location::new(
                location.point + coordinates::Direction::RIGHT,
                location.cost + 1,
                Direction::RIGHT,
                right_path,
            ),
        ],
    }
}

fn get_distance_from_end(current_point: Point<i32>, end_point: Point<i32>) -> i32 {
    (end_point.x - current_point.x).abs() + (end_point.y - current_point.y).abs()
}

fn a_star(
    map_grid: HashMap<Point<i32>, Tiles>,
    start_pos: Point<i32>,
    end_pos: Point<i32>,
    start_direction: Direction,
) -> i32 {
    let start_location = Location::new(start_pos, 0, start_direction, vec![start_pos]);
    let mut to_visit: Vec<Location> = vec![start_location];

    let max_point = *map_grid
        .iter()
        .max_by_key(|(point, _)| (point.x, point.y))
        .unwrap()
        .0;

    let mut visited: HashSet<Point<i32>> = HashSet::new();

    while !to_visit.is_empty() {
        let location = to_visit.remove(0);
        if *map_grid.get(&location.point).unwrap() == Tiles::End {
            return location.cost;
        }

        visited.insert(location.point);
        let next_locations = get_next_locations(&location);

        for l in next_locations {
            if l.point <= max_point
                && *map_grid.get(&l.point).unwrap() != Tiles::Wall
                && !visited.contains(&l.point)
            {
                to_visit.push(l);
            }
        }

        to_visit.sort_by(|x, y| {
            let hx = get_distance_from_end(x.point, end_pos);
            let hy = get_distance_from_end(y.point, end_pos);

            let diff = (x.cost + hx) - (y.cost + hy);
            if diff == 0 {
                return std::cmp::Ordering::Equal;
            }
            if diff < 0 {
                return std::cmp::Ordering::Less;
            }
            return std::cmp::Ordering::Greater;
        });
    }

    0
}

fn create_grid(map: Vec<&str>) -> (HashMap<Point<i32>, Tiles>, Point<i32>, Point<i32>) {
    let mut map_grid = HashMap::with_capacity(map.len() * map[0].len());
    let mut current_pos = Point { x: 0, y: 0 };
    let mut end_pos = Point { x: 0, y: 0 };

    for (y, line) in map.iter().enumerate() {
        for (x, tile) in line.chars().enumerate() {
            let point = Point { x: x as i32, y: y as i32 };

            let tile_type = match tile {
                'S' => {
                    current_pos = point;
                    Tiles::Start
                }
                '.' => Tiles::Empty,
                'E' => {
                    end_pos = point;
                    Tiles::End
                }
                '#' => Tiles::Wall,
                _ => unreachable!(),
            };

            map_grid.insert(point, tile_type);
        }
    }

    (map_grid, current_pos, end_pos)
}

fn part1(input_map: String) -> i32 {
    let map = input_map.lines().collect::<Vec<_>>();
    let (map_grid, current_pos, end_pos) = create_grid(map);

    a_star(map_grid, current_pos, end_pos, Direction::RIGHT)
}

fn find_all_paths(
    map_grid: HashMap<Point<i32>, Tiles>,
    start_pos: Point<i32>,
    end_pos: Point<i32>,
    start_direction: Direction,
    total_cost: i32,
) -> i32 {
    let start_location = Location::new(start_pos, 0, start_direction, vec![start_pos]);
    let mut to_visit = BinaryHeap::new();
    to_visit.push(Reverse((0, start_location)));

    let max_point = map_grid
        .keys()
        .max_by_key(|point| (point.x, point.y))
        .cloned()
        .unwrap();

    let mut grid = vec![vec![i32::MAX; max_point.x as usize + 1]; max_point.y as usize + 1];
    let mut paths: Vec<Vec<Point<i32>>> = Vec::new();

    while let Some(Reverse((_, location))) = to_visit.pop() {
        let current_len = location.current_path.len() as i32;
        let (x, y) = (location.point.x as usize, location.point.y as usize);

        if grid[y][x] < current_len {
            continue;
        }
        grid[y][x] = current_len;

        for next_location in get_next_locations(&location) {
            let next_point = next_location.point;

            if next_point > max_point {
                continue;
            }

            match map_grid.get(&next_point) {
                Some(Tiles::End) if location.cost == total_cost - 1 => {
                    paths.push(next_location.current_path);
                }
                Some(Tiles::Wall) => continue,
                Some(_) if location.cost < total_cost => {
                    let heuristic = get_distance_from_end(next_point, end_pos) as i32;
                    to_visit.push(Reverse((location.cost + heuristic, next_location)));
                }
                _ => {}
            }
        }
    }

    paths.into_iter().flatten().collect::<HashSet<_>>().len() as i32
}

fn part2(input_map: String) -> i32 {
    let map = input_map.lines().collect::<Vec<_>>();
    let (map_grid, current_pos, end_pos) = create_grid(map);

    let total_cost = a_star(map_grid.clone(), current_pos, end_pos, Direction::RIGHT);
    find_all_paths(map_grid, current_pos, end_pos, Direction::RIGHT, total_cost)
}

#[cfg(test)]
mod test {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 7036)
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 45)
    }
}

fn main() {
    utils::run(16, &["sample1.txt", "input.txt"], &part1, &part2);
}
