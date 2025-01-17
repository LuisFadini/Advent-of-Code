use std::cmp::Ordering::{Greater, Less};

use utils::coordinates::Point;

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

#[derive(Debug)]
struct Robot {
    pos: Point<i32>,
    vel: Point<i32>,
}

fn capture_point(input: &str) -> Point<i32> {
    let (x, y) = input.split_once(",").unwrap();

    Point::new(x.parse().unwrap(), y.parse().unwrap())
}

fn part1(input_robots: String) -> i32 {
    let mut robots = input_robots
        .lines()
        .map(|r| {
            let (pos, vel) = r.split_once(' ').unwrap();
            let pos = capture_point(pos.strip_prefix("p=").unwrap());
            let mut vel = capture_point(vel.strip_prefix("v=").unwrap());

            while vel.x < 0 {
                vel.x += WIDTH;
            }

            while vel.y < 0 {
                vel.y += HEIGHT;
            }

            Robot { pos, vel }
        })
        .collect::<Vec<_>>();

    for _ in 0..100 {
        for robot in robots.iter_mut() {
            robot.pos.x += robot.vel.x;
            robot.pos.x %= WIDTH;
            robot.pos.y += robot.vel.y;
            robot.pos.y %= HEIGHT;
        }
    }

    let mut quadrants = (0, 0, 0, 0);
    for robot in robots {
        match (
            robot.pos.x.cmp(&(WIDTH / 2)),
            robot.pos.y.cmp(&(HEIGHT / 2)),
        ) {
            (Less, Less) => quadrants.0 += 1,
            (Less, Greater) => quadrants.1 += 1,
            (Greater, Less) => quadrants.2 += 1,
            (Greater, Greater) => quadrants.3 += 1,
            _ => (),
        }
    }

    quadrants.0 * quadrants.1 * quadrants.2 * quadrants.3
}

fn part2(input_robots: String) -> i32 {
    let mut robots = input_robots
        .lines()
        .map(|r| {
            let (pos, vel) = r.split_once(' ').unwrap();
            let pos = capture_point(pos.strip_prefix("p=").unwrap());
            let mut vel = capture_point(vel.strip_prefix("v=").unwrap());

            while vel.x < 0 {
                vel.x += WIDTH;
            }

            while vel.y < 0 {
                vel.y += HEIGHT;
            }

            Robot { pos, vel }
        })
        .collect::<Vec<_>>();

    let mut time = 1;

    loop {
        for robot in robots.iter_mut() {
            robot.pos.x += robot.vel.x;
            robot.pos.x %= WIDTH;
            robot.pos.y += robot.vel.y;
            robot.pos.y %= HEIGHT;
        }

        for y in 0..HEIGHT {
            let mut x_positions: Vec<i32> = robots
                .iter()
                .filter(|r| r.pos.y == y)
                .map(|r| r.pos.x)
                .collect();

            x_positions.sort_unstable();

            let mut consecutive_count = 1;

            for i in 1..x_positions.len() {
                if x_positions[i] == x_positions[i - 1] + 1 {
                    consecutive_count += 1;

                    if consecutive_count == 15 {
                        return time;
                    }
                } else {
                    consecutive_count = 1;
                }
            }
        }
        time += 1;
    }
}

#[cfg(test)]
mod test {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 12)
    }
}

fn main() {
    utils::run(14, &["input.txt"], &part1, &part2);
}
