use std::collections::HashMap;

fn increment_paddle(k: i64, v: i64, paddles: &mut HashMap<i64, i64>) {
    *paddles.entry(k).or_insert(0) += v;
}

fn blink(input_paddles: String, blink_times: i32) -> i64 {
    let mut paddles: HashMap<i64, i64> = HashMap::new();

    input_paddles
        .split_ascii_whitespace()
        .map(|p| p.parse::<i64>().unwrap())
        .for_each(|p| {
            increment_paddle(p, 1, &mut paddles);
        });

    for _ in 0..blink_times {
        let mut new_paddles: HashMap<i64, i64> = HashMap::new();

        paddles.iter().for_each(|(&k, &v)| match k {
            0 => increment_paddle(1, v, &mut new_paddles),
            k if k.to_string().len() % 2 == 0 => {
                let k_str = k.to_string();
                let (left, right) = k_str.split_at(k.to_string().len() / 2);

                increment_paddle(left.parse().unwrap(), v, &mut new_paddles);
                increment_paddle(right.parse().unwrap(), v, &mut new_paddles);
            }
            _ => increment_paddle(k * 2024, v, &mut new_paddles),
        });

        paddles = new_paddles;
    }

    paddles.values().sum()
}

fn part1(input_raw_paddles: String) -> i64 {
    blink(input_raw_paddles, 25)
}

fn part2(input_raw_paddles: String) -> i64 {
    blink(input_raw_paddles, 75)
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 55312);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 65601038650482);
    }
}

fn main() {
    utils::run(11, &["sample1.txt", "input.txt"], &part1, &part2);
}
