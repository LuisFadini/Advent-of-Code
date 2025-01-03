use std::collections::HashSet;

use utils::{
    self,
    coordinates::{Direction, Point},
};

fn get_track(map: Vec<&str>, size: i32) -> Vec<Point<i32>> {
    let mut walls = HashSet::<Point<i32>>::new();
    let mut end_pos = Point { x: 0, y: 0 };
    let mut track = vec![];

    for (y, line) in (0..size).zip(map.iter().skip(1)) {
        for (x, tile) in (0..size).zip(line.bytes().skip(1)) {
            let point = Point {
                x: x as i32,
                y: y as i32,
            };

            match tile {
                b'S' => {
                    track.push(point);
                }
                b'E' => end_pos = point,
                b'#' => {
                    walls.insert(point);
                }
                _ => {}
            };
        }
    }

    while let Some(next_point) = Direction::CARDINAL
        .iter()
        .filter_map(|&dir| next_move(dir, *track.last().unwrap(), size))
        .find(|&next| !walls.contains(&next) && (track.len() < 2 || track[track.len() - 2] != next))
    {
        track.push(next_point);

        if next_point == end_pos {
            break;
        }
    }

    track
}

fn next_move(direction: Point<i32>, start: Point<i32>, size: i32) -> Option<Point<i32>> {
    let next = start + direction;

    match direction {
        Direction::UP if start.y > 0 => Some(next),
        Direction::RIGHT if start.x + 1 < size => Some(next),
        Direction::DOWN if start.y + 1 < size => Some(next),
        Direction::LEFT if start.x > 0 => Some(next),
        _ => None,
    }
}

fn find_cheats(track: Vec<Point<i32>>, min_saving: usize, max_cheat_time: usize) -> usize {
    let mut res = 0;

    for start in 0..track.len() {
        for end in (start + min_saving)..track.len() {
            let distance = track.get(start).unwrap().distance(track.get(end).unwrap());
            if distance <= max_cheat_time && distance <= (end - start - min_saving) as usize {
                res += 1;
            }
        }
    }

    res
}

fn part1(input: String) -> i32 {
    let size = input.lines().count() - 1;
    let track = get_track(input.lines().collect(), size as i32);

    find_cheats(track, 100, 2) as i32
}

fn part2(input: String) -> i32 {
    let size = input.lines().count() - 1;
    let track = get_track(input.lines().collect(), size as i32);

    find_cheats(track, 100, 20) as i32
}

fn main() {
    utils::run(20, vec!["input.txt"], &part1, &part2);
}
