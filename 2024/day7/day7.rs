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
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 3749);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 11387);
    }
}

fn main() {
    utils::run(7, &["sample1.txt", "input.txt"], &part1, &part2);
}
