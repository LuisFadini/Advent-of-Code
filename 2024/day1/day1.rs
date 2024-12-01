use std::{env, fs, process::exit};

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        part1(file_content);
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
        part1(fs::read_to_string(input_path.unwrap()).unwrap())
    );
}
