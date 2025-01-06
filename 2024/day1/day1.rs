use itertools::Itertools;

fn get_distances(input: String) -> (Vec<usize>, Vec<usize>) {
    input
        .trim()
        .lines()
        .map(|line| {
            let (left, right) = line.split_once("   ").unwrap();
            (
                left.parse::<usize>().unwrap(),
                right.parse::<usize>().unwrap(),
            )
        })
        .collect()
}

fn part1(input_data: String) -> usize {
    let (mut left_distances, mut right_distances) = get_distances(input_data);

    left_distances.sort_unstable();
    right_distances.sort_unstable();

    left_distances
        .iter()
        .zip(right_distances)
        .map(|(l, r)| l.abs_diff(r))
        .sum()
}

fn part2(input_data: String) -> usize {
    let (left_distances, right_distances) = get_distances(input_data);

    let frequencies = right_distances.into_iter().counts();

    left_distances
        .iter()
        .filter_map(|n| frequencies.get(n).map(|f| f * n))
        .sum()
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 11);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("sample1.txt")), 31);
    }
}

fn main() {
    utils::run(1, &["sample1.txt", "input.txt"], &part1, &part2);
}
