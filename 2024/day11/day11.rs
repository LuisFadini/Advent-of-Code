use std::{env, fs, process::exit};

fn blink(paddles: Vec<i64>) -> Vec<i64> {
    paddles
        .iter()
        .flat_map(|&p| {
            if p == 0 {
                vec![1]
            } else if p.to_string().len() % 2 == 0 {
                let s = p.to_string();
                let (left, right) = s.split_at(s.len() / 2);
                vec![left.parse().unwrap(), right.parse().unwrap()]
            } else {
                vec![p * 2024]
            }
        })
        .collect()
}

fn part1(input_raw_paddles: String) -> i64 {
    let mut paddles = input_raw_paddles
        .split(" ")
        .map(|p| p.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for _ in 0..25 {
        paddles = blink(paddles);
    }

    paddles.len() as i64
}

fn part2(input_data: String) -> i32 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 55312);
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
