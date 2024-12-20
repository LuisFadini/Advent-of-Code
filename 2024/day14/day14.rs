use std::cmp::Ordering::{Greater, Less};
use std::process::exit;
use std::{env, fs};

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Robot {
    pos: Point,
    vel: Point,
}

fn capture_point(input: &str) -> Point {
    let (x, y) = input.split_once(",").unwrap();

    Point {
        x: x.parse().unwrap(),
        y: y.parse().unwrap(),
    }
}

fn part1(input_robots: String) -> i32 {
    const WIDTH: i32 = 101;
    const HEIGHT: i32 = 103;

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
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(fs::read_to_string("./sample1.txt").unwrap()), 12)
    }
}

fn main() {
    println!("{}", part1(fs::read_to_string("./sample1.txt").unwrap()));
    let input_path = env::args().nth(1);
    if input_path.is_none() {
        println!("Input path should be specified!");
        exit(1);
    }

    println!(
        "Output: {}",
        part1(fs::read_to_string(input_path.unwrap()).unwrap())
    );
}
