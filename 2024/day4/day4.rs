fn part1(input_data: String) -> i32 {
    let input_data_lines: Vec<&str> = input_data.lines().collect();
    let mut valid_sequences = 0;

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

fn part2(input_data: String) -> i32 {
    let input_data_lines: Vec<&str> = input_data.lines().collect();
    let mut valid_sequences = 0;

    for i in 1..input_data_lines.len() - 1 {
        let line = input_data_lines[i];
        for j in 1..line.len() - 1 {
            let cur_char = get_char(line, j);

            if cur_char == 'A' {
                if get_neighbors(&input_data_lines, i, j) == ['M', 'M', 'S', 'S']
                    || get_neighbors(&input_data_lines, i, j) == ['S', 'S', 'M', 'M']
                    || get_neighbors(&input_data_lines, i, j) == ['M', 'S', 'M', 'S']
                    || get_neighbors(&input_data_lines, i, j) == ['S', 'M', 'S', 'M']
                {
                    valid_sequences += 1
                }
            }
        }
    }

    return valid_sequences;
}

fn get_char(line: &str, i: usize) -> char {
    return line.chars().nth(i).unwrap();
}

fn get_neighbors(lines: &[&str], i: usize, j: usize) -> [char; 4] {
    [
        get_char(lines[i - 1], j - 1), // Top-left
        get_char(lines[i - 1], j + 1), // Top-right
        get_char(lines[i + 1], j - 1), // Bottom-left
        get_char(lines[i + 1], j + 1), // Bottom-right
    ]
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 18);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 9);
    }
}

fn main() {
    utils::run(4, &["sample1.txt", "input.txt"], &part1, &part2);
}
