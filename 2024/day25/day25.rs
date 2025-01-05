#[derive(Debug)]
struct Schematic {
    height1: usize,
    height2: usize,
    height3: usize,
    height4: usize,
    height5: usize,
}

impl Schematic {
    fn new(height1: usize, height2: usize, height3: usize, height4: usize, height5: usize) -> Self {
        Schematic {
            height1,
            height2,
            height3,
            height4,
            height5,
        }
    }
}

fn calculate_schematic(lines: Vec<&str>) -> Schematic {
    let mut schematic = Schematic::new(0, 0, 0, 0, 0);

    for line in lines {
        for (i, c) in line.chars().take(5).enumerate() {
            if c == '#' {
                match i {
                    0 => schematic.height1 += 1,
                    1 => schematic.height2 += 1,
                    2 => schematic.height3 += 1,
                    3 => schematic.height4 += 1,
                    4 => schematic.height5 += 1,
                    _ => {}
                }
            }
        }
    }

    schematic
}

fn fit(key: &Schematic, lock: &Schematic) -> bool {
    key.height1 + lock.height1 < 6
        && key.height2 + lock.height2 < 6
        && key.height3 + lock.height3 < 6
        && key.height4 + lock.height4 < 6
        && key.height5 + lock.height5 < 6
}

fn part1(input: String) -> i32 {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in input.split("\n\n") {
        let lines: Vec<_> = schematic.lines().collect();
        if let (Some(first_line), Some(last_line)) = (lines.first(), lines.last()) {
            if first_line.chars().all(|c| c == '#') && last_line.chars().all(|c| c == '.') {
                locks.push(calculate_schematic(lines[1..].to_vec()));
            } else {
                keys.push(calculate_schematic(
                    lines[..lines.len() - 1].iter().rev().cloned().collect(),
                ));
            }
        }
    }

    locks
        .iter()
        .map(|lock| keys.iter().filter(|key| fit(key, lock)).count())
        .sum::<usize>() as i32
}

fn part2(_: String) -> String {
    "Finished my first AoC!".to_string()
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 3);
    }
}

fn main() {
    utils::run(25, &["sample1.txt", "input.txt"], &part1, &part2);
}
