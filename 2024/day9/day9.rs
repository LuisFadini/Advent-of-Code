use std::{iter::repeat, ops::Range};

fn part1(input_data: String) -> usize {
    let mut disk_content = Vec::with_capacity(input_data.len());

    for (index, size_char) in input_data.chars().enumerate() {
        let space_size = size_char.to_digit(10).unwrap() as usize;

        let value = if index % 2 == 0 {
            Some(index / 2)
        } else {
            None
        };

        disk_content.extend(repeat(value).take(space_size));
    }

    let mut head = 0;
    let mut tail = disk_content.len() - 1;

    while head < tail {
        while head < disk_content.len() && disk_content[head].is_some() {
            head += 1;
        }
        while tail > 0 && disk_content[tail].is_none() {
            tail -= 1;
        }

        if head < tail {
            disk_content.swap(head, tail);
        }
    }

    disk_content
        .iter()
        .enumerate()
        .map(|(index, id)| (index) * id.unwrap_or(0))
        .sum()
}

fn part2(input_data: String) -> u64 {
    let mut files: Vec<(Range<usize>, u64)> = Vec::new();
    let mut free_blocks: Vec<Range<usize>> = Vec::new();

    let mut curr_i = 0;

    for (index, size_char) in input_data.chars().enumerate() {
        let space_size = size_char.to_digit(10).unwrap() as usize;
        let space_range = curr_i..curr_i + space_size;
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
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 1928);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 2858);
    }
}

fn main() {
    utils::run(9, &["sample1.txt", "input.txt"], &part1, &part2);
}
