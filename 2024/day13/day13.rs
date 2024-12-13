use std::{env, fs, process::exit, vec};

use regex::Regex;

fn capture_x_y(input: &str) -> (i32, i32) {
    let re = Regex::new(r"X[+=](\d+), Y[+=](\d+)").unwrap();

    let captures = re.captures(input).unwrap();

    let x = captures.get(1).unwrap().as_str().parse::<i32>().unwrap();
    let y = captures.get(2).unwrap().as_str().parse::<i32>().unwrap();

    (x, y)
}

fn part1(input_controls: String) -> i32 {
    let control_groups = input_controls.split("\n\n").collect::<Vec<_>>();

    let mut total_tokens = 0;

    for group in control_groups {
        let group_lines: Vec<_> = group.lines().collect();
        let (button_a_line, button_b_line, prize_line) =
            (group_lines[0], group_lines[1], group_lines[2]);

        let (button_a_x, button_a_y) = capture_x_y(button_a_line);
        let (button_b_x, button_b_y) = capture_x_y(button_b_line);
        let (prize_x, prize_y) = capture_x_y(prize_line);

        let mut cheapest_way: (i32, i32) = (i32::MAX, i32::MAX);

        for i in 1..=prize_x / button_a_x {
            if (prize_x - (button_a_x * i)) % button_b_x == 0 {
                let b_presses = (prize_x - (button_a_x * i)) / button_b_x;

                if (prize_y - (button_a_y * i)) % button_b_y == 0 {
                    let b_presses_y = (prize_y - (button_a_y * i)) / button_b_y;

                    if b_presses == b_presses_y {
                        let cost = i * 3 + b_presses * 1;

                        if cheapest_way == (i32::MAX, i32::MAX)
                            || cost < ((cheapest_way.0 * 3) + cheapest_way.1)
                        {
                            cheapest_way = (i, b_presses);
                        }
                    }
                }
            }
        }

        if cheapest_way != (i32::MAX, i32::MAX) {
            if [cheapest_way.0, cheapest_way.1]
                .iter()
                .all(|&value| value <= 100)
            {
                total_tokens += cheapest_way.0 * 3 + cheapest_way.1 * 1;
            }
        }
    }

    total_tokens
}

fn _part2(_input_controls: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 480);
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
        part1(fs::read_to_string(input_path.unwrap()).unwrap())
    );
}
