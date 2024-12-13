use std::{env, fs, process::exit};

use regex::Regex;

fn capture_x_y(input: &str) -> (i64, i64) {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    let captures = re.captures(input).unwrap();

    let x = captures.get(1).unwrap().as_str().parse::<i64>().unwrap();
    let y = captures.get(2).unwrap().as_str().parse::<i64>().unwrap();

    (x, y)
}

fn linear_eq(
    button_x1: i64,
    button_x2: i64,
    prize_x: i64,
    button_y1: i64,
    button_y2: i64,
    prize_y: i64,
) -> (i64, i64) {
    let d = button_x1 * button_y2 - button_y1 * button_x2;
    let dx = prize_x * button_y2 - prize_y * button_x2;
    let dy = button_x1 * prize_y - button_y1 * prize_x;

    let x = dx / d;
    let y = dy / d;

    (x, y)
}

fn solve(input_controls: String, part2: bool) -> i64 {
    let control_groups = input_controls.split("\n\n").collect::<Vec<_>>();

    let mut total_tokens = 0;

    for group in control_groups {
        let group_lines: Vec<_> = group.lines().collect();
        let (button_a_line, button_b_line, prize_line) =
            (group_lines[0], group_lines[1], group_lines[2]);

        let (button_a_x, button_a_y) = capture_x_y(button_a_line);
        let (button_b_x, button_b_y) = capture_x_y(button_b_line);
        let (mut prize_x, mut prize_y) = capture_x_y(prize_line);

        if part2 {
            prize_x += 10000000000000;
            prize_y += 10000000000000;
        }

        let mut cheapest_way: (i64, i64) = (i64::MAX, i64::MAX);
        let (a, b) = linear_eq(
            button_a_x, button_b_x, prize_x, button_a_y, button_b_y, prize_y,
        );

        if a * button_a_x + b * button_b_x == prize_x && a * button_a_y + b * button_b_y == prize_y
        {
            cheapest_way = (a, b)
        }

        if cheapest_way != (i64::MAX, i64::MAX) {
            if part2
                || [cheapest_way.0, cheapest_way.1]
                    .iter()
                    .all(|&value| value <= 100)
            {
                total_tokens += cheapest_way.0 * 3 + cheapest_way.1 * 1;
            }
        }
    }

    total_tokens
}

fn part1(input_controls: String) -> i32 {
    solve(input_controls, false) as i32
}

fn part2(input_controls: String) -> i64 {
    solve(input_controls, true)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 480);
    }

    #[test]
    fn test2() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part2(file_content), 875318608908);
    }
}

fn main() {
    let file_content = fs::read_to_string("./sample1.txt").unwrap();
    println!("{}", part2(file_content));
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
