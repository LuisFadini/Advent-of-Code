fn part1(report_data: String) -> i32 {
    let reports = report_data
        .trim()
        .split("\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|l| {
            l.trim()
                .split_ascii_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    return reports
        .into_iter()
        .filter(|r| is_safe(&r))
        .count()
        .try_into()
        .unwrap();
}

fn part2(report_data: String) -> i32 {
    let reports = report_data
        .trim()
        .split("\n")
        .collect::<Vec<_>>()
        .into_iter()
        .map(|l| {
            l.trim()
                .split(" ")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    return reports
        .into_iter()
        .filter(|r| safe_with_removal(&r))
        .count()
        .try_into()
        .unwrap();
}

fn is_safe(level: &Vec<i32>) -> bool {
    let only_increasing = level.windows(2).all(|w| w[0] < w[1]);
    let only_decreasing = level.windows(2).all(|w| w[0] > w[1]);
    let dist_ok = level
        .windows(2)
        .all(|w| 1 <= (w[0] - w[1]).abs() && (w[0] - w[1]).abs() <= 3);

    return (only_increasing ^ only_decreasing) && dist_ok;
}

fn safe_with_removal(level: &Vec<i32>) -> bool {
    if is_safe(level) {
        return true;
    }

    for i in 0..level.len() {
        let mut cloned_level = level.clone();
        cloned_level.remove(i);

        if is_safe(&cloned_level) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 4);
    }
}

fn main() {
    utils::run(2, &["sample1.txt", "input.txt"], &part1, &part2);
}
