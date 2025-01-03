use std::collections::{HashMap, VecDeque};

use utils::{
    self,
    coordinates::{Direction, Point},
};

#[derive(PartialEq, Clone)]
enum Ram {
    Safe,
    Corrupted,
    Visited,
}

fn bfs(map: &mut HashMap<Point<i32>, Ram>, start: Point<i32>) -> Option<usize> {
    let directions = [
        Direction::UP,
        Direction::RIGHT,
        Direction::DOWN,
        Direction::LEFT,
    ];
    let end_point = Point { x: 70, y: 70 };

    let mut queue: VecDeque<(Point<i32>, usize)> = VecDeque::new();
    queue.push_back((start, 0));

    map.insert(start, Ram::Visited);

    while let Some((q_point, steps)) = queue.pop_front() {
        if q_point == end_point {
            return Some(steps);
        }

        for dir in &directions {
            let next_point = q_point + *dir;

            if next_point.x >= 71 || next_point.y >= 71 {
                continue;
            }

            if *map.get(&next_point).unwrap_or(&Ram::Visited) == Ram::Safe {
                map.insert(next_point, Ram::Visited);
                queue.push_back((next_point, steps + 1));
            }
        }
    }

    None
}

fn part1(input: String) -> i32 {
    let mut map = HashMap::<Point<i32>, Ram>::new();

    for y in 0..71 {
        for x in 0..71 {
            map.insert(Point { x, y }, Ram::Safe);
        }
    }

    input.lines().take(1024).for_each(|line| {
        let (x, y) = line.split_once(',').unwrap();

        map.insert(
            Point {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            },
            Ram::Corrupted,
        );
    });

    bfs(&mut map, Point { x: 0, y: 0 }).unwrap_or(0) as i32
}

fn part2(input: String) -> String {
    let mut map = HashMap::<Point<i32>, Ram>::new();

    for y in 0..71 {
        for x in 0..71 {
            map.insert(Point { x, y }, Ram::Safe);
        }
    }

    let bytes: Vec<_> = input
        .lines()
        .map(|line| {
            let (x, y) = line.split_once(',').unwrap();
            let point = Point::<i32> {
                x: x.parse().unwrap(),
                y: y.parse().unwrap(),
            };
            map.insert(point, Ram::Corrupted);
            point
        })
        .collect();

    bfs(&mut map, Point { x: 0, y: 0 });
    for p in bytes.into_iter().rev() {
        map.insert(p, Ram::Safe);

        if [
            p + Direction::UP,
            p + Direction::RIGHT,
            p + Direction::DOWN,
            p + Direction::LEFT,
        ]
        .into_iter()
        .any(|dir| {
            dir.x >= 0
                && dir.y >= 0
                && dir.x < 71
                && dir.y < 71
                && map.get(&dir) == Some(&Ram::Visited)
        }) && bfs(&mut map, p).is_some()
        {
            return format!("{},{}", p.x, p.y);
        }
    }

    panic!("Your input is probably invalid")
}

fn main() {
    utils::run(18, &["input.txt"], &part1, &part2);
}
