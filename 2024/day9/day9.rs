use std::{env, fs, ops::Range, process::exit};

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

fn part2(input_data: String) -> u64 {
    let mut files: Vec<(Range<usize>, u64)> = Vec::new();
    let mut free_blocks: Vec<Range<usize>> = Vec::new();

    let mut curr_i = 0;

    for (index, size_char) in input_data.chars().enumerate() {
        let space_size = size_char.to_digit(10).unwrap();
        let space_range = curr_i as usize..curr_i as usize + space_size as usize;
        curr_i = space_range.end;

        if index % 2 == 0 {
            files.push((space_range, index as u64 / 2));
        } else {
            free_blocks.push(space_range);
        }
    }

    for file in files.iter_mut().rev() {
        if let Some((i, free)) = free_blocks
            .iter_mut()
            .enumerate()
            .find(|(_, f)| f.end <= file.0.start && f.len() >= file.0.len())
        {
            let free_start = free.start;
            *free = free_start + file.0.len()..free.end;
            *file = (free_start..free_start + file.0.len(), file.1);
            if free.len() == 0 {
                free_blocks.remove(i);
            }
        }
    }

    files
        .into_iter()
        .map(|f| f.0.sum::<usize>() as u64 * f.1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 1928);
    }

    #[test]
    fn test2() {
        let file_content = fs::read_to_string("./sample2.txt").unwrap();
        assert_eq!(part2(file_content), 2858);
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
        part2(fs::read_to_string(input_path.unwrap()).unwrap())
    );
}
