use std::{self, env, fs, process::exit};

fn part1(input_data: String) -> u32 {
    let input_data_lines = input_data.split("\n").collect::<Vec<_>>();

    let mut sum: i32 = 0;

    for line in input_data_lines {
        let mut first_num: i32 = -1;
        let mut last_num: i32 = -1;

        for c in line.chars().filter(|d| d.is_numeric()) {
            if let Some(digit) = c.to_digit(10) {
                let digit = digit as i32;

                if first_num == -1 {
                    first_num = digit;
                }

                last_num = digit;
            }
        }

        sum += 10 * first_num + last_num;
    }

    return u32::try_from(sum).unwrap();
}

fn valid_digit(s: &str) -> Option<u32> {
    let digit = s.chars().nth(0).unwrap();
    let digit = digit.to_digit(10);

    if digit.is_some() {
        digit
    } else if s.starts_with("zero") {
        Some(0)
    } else if s.starts_with("one") {
        Some(1)
    } else if s.starts_with("two") {
        Some(2)
    } else if s.starts_with("three") {
        Some(3)
    } else if s.starts_with("four") {
        Some(4)
    } else if s.starts_with("five") {
        Some(5)
    } else if s.starts_with("six") {
        Some(6)
    } else if s.starts_with("seven") {
        Some(7)
    } else if s.starts_with("eight") {
        Some(8)
    } else if s.starts_with("nine") {
        Some(9)
    } else {
        None
    }
}

fn part2(input_data: String) -> u32 {
    let input_data_lines = input_data.split("\n").collect::<Vec<_>>();

    let mut sum = 0;

    for line in input_data_lines {
        let mut first_num: i32 = -1;
        let mut last_num: i32 = -1;

        for i in 0..line.len() {
            if let Some(first) = valid_digit(&line[i..]) {
                first_num = first as i32;
                break;
            }
        }

        for i in (0..line.len()).rev() {
            if let Some(last) = valid_digit(&line[i..]) {
                last_num = last as i32;
                break;
            }
        }

        sum += 10 * first_num + last_num;
    }

    return u32::try_from(sum).unwrap();
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test1() {
        let sample1 = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(sample1), 142);
    }

    #[test]
    fn test2() {
        let sample2 = fs::read_to_string("./sample2.txt").unwrap();
        assert_eq!(part2(sample2), 281);
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
