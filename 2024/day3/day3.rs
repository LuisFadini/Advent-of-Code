use regex::Regex;

fn part1(corrupted_input_data: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    return re
        .captures_iter(&corrupted_input_data)
        .map(|cap| {
            let first_mul = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
            let second_mul = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

            first_mul * second_mul
        })
        .sum();
}

fn part2(corrupted_input_data: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do(|n't)\(\)").unwrap();

    let mut mul_enable = true;
    return re
        .captures_iter(&corrupted_input_data)
        .map(|cap| {
            if cap.get(0).unwrap().as_str() == "don't()" {
                mul_enable = false;
            } else if cap.get(0).unwrap().as_str() == "do()" {
                mul_enable = true;
            } else if mul_enable {
                let first_mul = cap.get(1).unwrap().as_str().parse::<i32>().unwrap();
                let second_mul = cap.get(2).unwrap().as_str().parse::<i32>().unwrap();

                return first_mul * second_mul;
            }

            0
        })
        .sum();
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 161);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample2.txt")), 48);
    }
}

fn main() {
    utils::run(
        3,
        &["sample1.txt", "sample2.txt", "input.txt"],
        &part1,
        &part2,
    );
}
