use regex::Regex;

fn part1(corrupted_input_data: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    re.captures_iter(&corrupted_input_data)
        .map(|cap| {
            let (_, [lhs, rhs]) = cap.extract();
            lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap()
        })
        .sum()
}

fn part2(corrupted_input_data: String) -> i32 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do(|n't)\(\)").unwrap();

    let mut mul_enable = true;
    re.captures_iter(&corrupted_input_data)
        .map(|cap| {
            let instruction = cap.get(0).unwrap().as_str();

            if instruction == "don't()" {
                mul_enable = false;
            } else if instruction == "do()" {
                mul_enable = true;
            } else if mul_enable {
                let (lhs, rhs) = (cap.get(1).unwrap().as_str(), cap.get(2).unwrap().as_str());

                return lhs.parse::<i32>().unwrap() * rhs.parse::<i32>().unwrap();
            }

            0
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 161);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("sample2.txt")), 48);
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
