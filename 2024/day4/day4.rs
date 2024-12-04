use std::{env, fs, process::exit};

fn part1(input_data: String) -> i32 {
    let input_data_lines: Vec<&str> = input_data.lines().collect();
    let mut valid_sequences = 0;

    // Iterate through each line and its characters
    for (i, line) in input_data_lines.iter().enumerate() {
        for (j, _) in line.chars().enumerate() {
            valid_sequences += valid_xmas_sequences(input_data_lines.clone(), i, j);
        }
    }

    valid_sequences
}

fn valid_xmas_sequences(lines: Vec<&str>, i: usize, j: usize) -> i32 {
    let mut valid_sequences = 0;

    let xmas = vec!['X', 'M', 'A', 'S'];
    let samx = vec!['S', 'A', 'M', 'X'];

    if j + 4 <= lines[i].len() {
        let horizontal_sequence: Vec<_> = lines[i][j..j + 4].chars().collect();
        if horizontal_sequence == xmas || horizontal_sequence == samx {
            valid_sequences += 1;
        }
    }

    if i + 4 <= lines.len() {
        let vertical_sequence: Vec<_> = (0..4)
            .map(|k| lines[i + k].chars().nth(j).unwrap())
            .collect();

        if vertical_sequence == xmas || vertical_sequence == samx {
            valid_sequences += 1;
        }
    }

    if i + 4 <= lines.len() && j + 4 <= lines[i].len() {
        let to_right_diagonal_sequence: Vec<_> = (0..4)
            .map(|k| lines[i + k].chars().nth(j + k).unwrap())
            .collect();

        if to_right_diagonal_sequence == xmas || to_right_diagonal_sequence == samx {
            valid_sequences += 1;
        }
    }

    if i + 4 <= lines.len() && j >= 3 {
        let to_left_diagonal_sequence: Vec<_> = (0..4)
            .map(|k| lines[i + k].chars().nth(j - k).unwrap())
            .collect();

        if to_left_diagonal_sequence == xmas || to_left_diagonal_sequence == samx {
            valid_sequences += 1;
        }
    }

    valid_sequences
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 18);
    }
}

fn main() {
    // let file_content = fs::read_to_string("./sample1.txt").unwrap();
    // println!("{}", part1(file_content));
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
