use std::{collections::HashMap, env, fs, process::exit};

fn increment_paddle(k: i64, v: i64, paddles: &mut HashMap<i64, i64>) {
    let count = paddles.entry(k).or_insert(0);
    *count += v;
}

fn blink(input_paddles: String, blink_times: i32) -> i64 {
    let mut paddles: HashMap<i64, i64> = HashMap::new();

    input_paddles
        .split(" ")
        .map(|p| p.parse::<i64>().unwrap())
        .for_each(|p| {
            increment_paddle(p, 1, &mut paddles);
        });

    for _ in 0..blink_times {
        let mut new_paddles: HashMap<i64, i64> = HashMap::new();

        paddles
            .iter()
            .filter(|&(_, &v)| v != 0)
            .for_each(|(&k, &v)| match k {
                0 => increment_paddle(1, v, &mut new_paddles),
                k if k.to_string().len() % 2 == 0 => {
                    let k_str = k.to_string();
                    let (left, right) = k_str.split_at(k.to_string().len() / 2);

                    increment_paddle(left.to_string().parse().unwrap(), v, &mut new_paddles);
                    increment_paddle(right.to_string().parse().unwrap(), v, &mut new_paddles);
                }
                _ => increment_paddle(k * 2024, v, &mut new_paddles),
            });

        paddles = new_paddles;
    }

    paddles.values().sum::<i64>()
}

fn part1(input_raw_paddles: String) -> i64 {
    blink(input_raw_paddles, 25)
}

fn part2(input_raw_paddles: String) -> i64 {
    blink(input_raw_paddles, 75)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 55312);
    }

    #[test]
    fn test2() {
        let file_content = fs::read_to_string("./sample2.txt").unwrap();
        assert_eq!(part2(file_content), 65601038650482);
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
