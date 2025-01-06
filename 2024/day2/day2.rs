fn part1(report_data: String) -> usize {
    report_data
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|r| is_safe(r))
        .count()
}

fn part2(report_data: String) -> usize {
    report_data
        .lines()
        .map(|l| {
            l.trim()
                .split(" ")
                .map(|c| c.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|r| safe_with_removal(&r))
        .count()
}

fn is_safe(level: &Vec<i32>) -> bool {
    let mut is_increasing = true;
    let mut is_decreasing = true;
    let mut is_distance_ok = true;
    for w in level.windows(2) {
        let difference = w[1] - w[0];
        is_increasing &= difference > 0;
        is_decreasing &= difference < 0;
        if !(1..=3).contains(&difference.abs()) {
            is_distance_ok = false;
            break;
        }
    }
    (is_increasing ^ is_decreasing) && is_distance_ok
}

fn safe_with_removal(level: &Vec<i32>) -> bool {
    if is_safe(level) {
        return true;
    }

    let mut cloned_level = level.clone();
    for i in 0..level.len() {
        cloned_level.remove(i);
        if is_safe(&cloned_level) {
            return true;
        }
        cloned_level = level.clone();
    }

    false
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 2);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("sample1.txt")), 4);
    }
}

fn main() {
    utils::run(2, &["sample1.txt", "input.txt"], &part1, &part2);
}
