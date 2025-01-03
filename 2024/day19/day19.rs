use std::collections::HashMap;

use utils;

fn recursive_design(design: &str, patterns: &[&str]) -> bool {
    if design.is_empty() {
        return true;
    }

    for pattern in patterns.iter() {
        if design.starts_with(pattern) {
            let remainder = design.strip_prefix(pattern).unwrap();
            if recursive_design(remainder, &patterns) {
                return true;
            }
        }
    }

    false
}

fn part1(input: String) -> usize {
    let (available_patterns, designs) = input.split_once("\n\n").unwrap();
    let available_patterns = available_patterns.split(", ").collect::<Vec<_>>();
    let designs = designs.lines().collect::<Vec<_>>();

    designs
        .iter()
        .map(|d| recursive_design(d, &available_patterns))
        .filter(|d| *d == true)
        .count()
}

fn count_combinations(design: &str, patterns: &[&str], memo: &mut HashMap<String, usize>) -> usize {
    if let Some(&count) = memo.get(design) {
        return count;
    }

    if design.is_empty() {
        return 1;
    }

    let mut total_combinations = 0;

    for pattern in patterns.iter() {
        if design.starts_with(pattern) {
            let remainder = &design[pattern.len()..];
            total_combinations += count_combinations(remainder, &patterns, memo);
        }
    }

    memo.insert(design.to_string(), total_combinations);

    total_combinations
}

fn part2(input: String) -> usize {
    let (available_patterns, designs) = input.split_once("\n\n").unwrap();
    let available_patterns = available_patterns.split(", ").collect::<Vec<_>>();
    let designs = designs.lines().collect::<Vec<_>>();

    let mut memo = HashMap::new();

    designs
        .iter()
        .map(|d| count_combinations(d, &available_patterns, &mut memo))
        .into_iter()
        .sum()
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 6);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("sample1.txt")), 16);
    }
}

fn main() {
    utils::run(19, &["sample1.txt", "input.txt"], &part1, &part2);
}
