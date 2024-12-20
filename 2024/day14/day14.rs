use std::cmp::Ordering::{Greater, Less};
use std::io::{self, stdin, Write};
use std::process::exit;
use std::{env, fs};

const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;

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

fn print_to_file(robots: &Vec<Robot>, iteration: i32) {
    let mut file_buffer = vec![format!("Iteration {}", iteration)];

    let mut robot_counts = [[0; WIDTH as usize]; HEIGHT as usize];

    for robot in robots {
        robot_counts[robot.pos.y as usize][robot.pos.x as usize] += 1;
    }

    file_buffer.push(
        robot_counts
            .into_iter()
            .map(|r| {
                r.into_iter()
                    .map(|n| {
                        if n == 0 {
                            ".".to_string()
                        } else {
                            "#".to_string()
                        }
                    })
                    .collect::<String>()
                    + "\n"
            })
            .collect::<String>(),
    );

    if !fs::exists(format!("./output/output_{}.txt", iteration)).unwrap() {
        println!("{}", file_buffer.join("\n"));
        fs::write(
            format!("./output/output_{}.txt", iteration),
            file_buffer.join("\n"),
        )
        .unwrap();
    }
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

    fs::remove_dir_all("./output").unwrap();
    fs::create_dir("./output").unwrap();

    let mut time = 1;
    let mut last_asked_time = 0;

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

                    if consecutive_count == 10 && last_asked_time != time {
                        print_to_file(&robots, time);
                        print!("Press 'y' if it correct or Enter if it is not correct: ");
                        io::stdout().flush().unwrap();

                        let mut input = String::new();

                        io::stdin().read_line(&mut input).unwrap();

                        let input = input.trim();

                        if input == "y" {
                            return time;
                        } else {
                            fs::remove_file(format!("./output/output_{}.txt", time)).unwrap();
                            last_asked_time = time
                        }
                        break;
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
    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(fs::read_to_string("./sample1.txt").unwrap()), 12)
    }
}

fn main() {
    let input_path = env::args().nth(1);
    if input_path.is_none() {
        println!("Input path should be specified!");
        exit(1);
    }

    println!(
        "Output: {}",
        part2(fs::read_to_string(input_path.unwrap()).unwrap())
    );
}
