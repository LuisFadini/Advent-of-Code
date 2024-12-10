use std::{env, fs, process::exit};

fn part1(input_data: String) -> u64 {
    let mut disk_content = Vec::new();

    for (index, size_char) in input_data.chars().enumerate() {
        let space_size = size_char.to_digit(10).unwrap();

        disk_content.extend(
            std::iter::repeat(if index % 2 == 0 {
                Some(index / 2)
            } else {
                None
            })
            .take(space_size as usize),
        );
    }

    let mut head = 0;
    let mut tail = disk_content.len() - 1;

    loop {
        while disk_content[head].is_some() {
            head += 1;
        }
        while disk_content[tail].is_none() {
            tail -= 1;
        }
        if head >= tail {
            break;
        }
        disk_content.swap(head, tail);
    }

    disk_content
        .iter()
        .enumerate()
        .map(|(index, id)| (index as u64) * u64::try_from(id.unwrap_or(0)).unwrap())
        .sum()
}

fn part2(input_data: String) -> i32 {
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 1928);
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
