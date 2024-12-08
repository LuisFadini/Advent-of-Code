use std::{env, fs, process::exit};

fn part1(input_data: String) -> i64 {
    let equations_lines = input_data.lines();

    let mut valid_equations: i64 = 0;

    for equation in equations_lines {
        let (res, numbers) = equation.split_once(": ").unwrap();

        if evaluate_possible_operation(
            res.parse().unwrap(),
            numbers.split(" ").collect::<Vec<_>>()[0]
                .parse::<i64>()
                .unwrap(),
            numbers
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()[1..]
                .to_vec(),
            false,
        ) {
            valid_equations += res.parse::<i64>().unwrap();
        }
    }

    return valid_equations;
}

fn evaluate_possible_operation(
    target: i64,
    current_number: i64,
    next_numbers: Vec<i64>,
    concatenation: bool,
) -> bool {
    if next_numbers.len() == 0 {
        return target == current_number;
    }

    if current_number > target {
        return false;
    }

    return evaluate_possible_operation(
        target,
        current_number + next_numbers[0],
        next_numbers[1..].to_vec(),
        concatenation,
    ) || evaluate_possible_operation(
        target,
        current_number * next_numbers[0],
        next_numbers[1..].to_vec(),
        concatenation,
    ) || (concatenation
        && evaluate_possible_operation(
            target,
            (current_number.to_string() + &next_numbers[0].to_string())
                .parse()
                .unwrap(),
            next_numbers[1..].to_vec(),
            concatenation,
        ));
}

fn part2(input_data: String) -> i64 {
    let equations_lines = input_data.lines();

    let mut valid_equations: i64 = 0;

    for equation in equations_lines {
        let (res, numbers) = equation.split_once(": ").unwrap();

        if evaluate_possible_operation(
            res.parse().unwrap(),
            numbers.split(" ").collect::<Vec<_>>()[0]
                .parse::<i64>()
                .unwrap(),
            numbers
                .split(" ")
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<_>>()[1..]
                .to_vec(),
            true,
        ) {
            valid_equations += res.parse::<i64>().unwrap();
        }
    }

    return valid_equations;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 3749);
    }

    #[test]
    fn test2() {
        let file_content = fs::read_to_string("./sample2.txt").unwrap();
        assert_eq!(part2(file_content), 11387);
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
