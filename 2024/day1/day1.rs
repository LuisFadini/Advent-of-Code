use std::collections::HashMap;

fn part1(input_data: String) -> i32 {
    let lines = input_data.trim().split("\n").collect::<Vec<_>>();
    let mut left_dists: Vec<i32> = vec![];
    let mut right_dists: Vec<i32> = vec![];

    lines.into_iter().for_each(|l| {
        left_dists.push(
            l.trim().split("   ").collect::<Vec<_>>()[0]
                .to_string()
                .parse::<i32>()
                .unwrap(),
        );
        right_dists.push(
            l.trim().split("   ").collect::<Vec<_>>()[1]
                .to_string()
                .parse::<i32>()
                .unwrap(),
        );
    });

    left_dists.sort();
    right_dists.sort();

    let mut dist_sum = 0;

    for _i in 0..left_dists.len() {
        dist_sum += i32::from(left_dists.get(0).unwrap() - right_dists.get(0).unwrap()).abs();

        left_dists.remove(0);
        right_dists.remove(0);
    }

    return dist_sum;
}

fn part2(input_data: String) -> i32 {
    let lines = input_data.trim().split("\n").collect::<Vec<_>>();
    let mut left_dists: Vec<i32> = vec![];
    let mut right_dists: Vec<i32> = vec![];

    lines.into_iter().for_each(|l| {
        left_dists.push(
            l.trim().split("   ").collect::<Vec<_>>()[0]
                .to_string()
                .parse::<i32>()
                .unwrap(),
        );
        right_dists.push(
            l.trim().split("   ").collect::<Vec<_>>()[1]
                .to_string()
                .parse::<i32>()
                .unwrap(),
        );
    });

    let mut lookup: HashMap<i32, i32> = HashMap::new();

    let mut dist_sum = 0;

    for dist in left_dists {
        if lookup.contains_key(&dist) {
            dist_sum += lookup.get(&dist).unwrap();
            continue;
        }

        if !right_dists.contains(&dist) {
            continue;
        }

        let right_dists_equal_left_one = right_dists
            .clone()
            .into_iter()
            .filter(|f| *f == dist)
            .count();

        lookup.insert(
            dist,
            i32::try_from(right_dists_equal_left_one).unwrap() * dist,
        );
        dist_sum += lookup.get(&dist).unwrap();
    }

    return dist_sum;
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 11);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 31);
    }
}

fn main() {
    utils::run(1, &["sample1.txt", "input.txt"], &part1, &part2);
}
