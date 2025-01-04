use std::collections::{HashMap, VecDeque};

use utils::{
    self,
    coordinates::{Direction, Point},
};

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Keys {
    Keypad0,
    Keypad1,
    Keypad2,
    Keypad3,
    Keypad4,
    Keypad5,
    Keypad6,
    Keypad7,
    Keypad8,
    Keypad9,
    KeypadA,
    KeypadUp,
    KeypadRight,
    KeypadDown,
    KeypadLeft,
    Empty,
}

type Keypad = HashMap<Point<i32>, Keys>;

fn get_numpad() -> Keypad {
    vec![
        (Point { x: 0, y: 0 }, Keys::Keypad7),
        (Point { x: 1, y: 0 }, Keys::Keypad8),
        (Point { x: 2, y: 0 }, Keys::Keypad9),
        (Point { x: 0, y: 1 }, Keys::Keypad4),
        (Point { x: 1, y: 1 }, Keys::Keypad5),
        (Point { x: 2, y: 1 }, Keys::Keypad6),
        (Point { x: 0, y: 2 }, Keys::Keypad1),
        (Point { x: 1, y: 2 }, Keys::Keypad2),
        (Point { x: 2, y: 2 }, Keys::Keypad3),
        (Point { x: 0, y: 3 }, Keys::Empty),
        (Point { x: 1, y: 3 }, Keys::Keypad0),
        (Point { x: 2, y: 3 }, Keys::KeypadA),
    ]
    .into_iter()
    .collect()
}

fn get_control_pad() -> Keypad {
    vec![
        (Point { x: 0, y: 0 }, Keys::Empty),
        (Point { x: 1, y: 0 }, Keys::KeypadUp),
        (Point { x: 2, y: 0 }, Keys::KeypadA),
        (Point { x: 0, y: 1 }, Keys::KeypadLeft),
        (Point { x: 1, y: 1 }, Keys::KeypadDown),
        (Point { x: 2, y: 1 }, Keys::KeypadRight),
    ]
    .into_iter()
    .collect()
}

fn parse_input(input: String) -> Vec<(Vec<Keys>, usize)> {
    input
        .trim()
        .lines()
        .map(|line| {
            (
                line.chars()
                    .map(|c| match c {
                        '0' => Keys::Keypad0,
                        '1' => Keys::Keypad1,
                        '2' => Keys::Keypad2,
                        '3' => Keys::Keypad3,
                        '4' => Keys::Keypad4,
                        '5' => Keys::Keypad5,
                        '6' => Keys::Keypad6,
                        '7' => Keys::Keypad7,
                        '8' => Keys::Keypad8,
                        '9' => Keys::Keypad9,
                        'A' => Keys::KeypadA,
                        _ => unreachable!(),
                    })
                    .collect(),
                line[0..3].parse().unwrap(),
            )
        })
        .collect()
}

fn get_shortest_path(start: Point<i32>, keypad: &Keypad) -> HashMap<Keys, Vec<Vec<Keys>>> {
    let mut paths: HashMap<Keys, Vec<Vec<Keys>>> = HashMap::new();

    let mut to_visit: VecDeque<(Point<i32>, Vec<Keys>)> = vec![(start, vec![])].into();

    let initial_key = keypad.get(&start).unwrap();
    paths.insert(*initial_key, vec![]);

    while let Some((coordinate, path)) = to_visit.pop_front() {
        let current_key = keypad.get(&coordinate).unwrap();

        let shortest = paths.entry(*current_key).or_default();

        let mut dedup_shortest = shortest.last().unwrap_or(&vec![]).clone();
        dedup_shortest.dedup();
        let mut dedup_path = path.clone();
        dedup_path.dedup();

        if dedup_shortest.is_empty() || dedup_shortest.len() > dedup_path.len() {
            *shortest = vec![path.clone()];
        } else if dedup_shortest.len() == dedup_path.len() {
            (*shortest).push(path.clone());
        } else {
            continue;
        }

        let neighbors = [
            (coordinate + Direction::UP, Keys::KeypadUp),
            (coordinate + Direction::RIGHT, Keys::KeypadRight),
            (coordinate + Direction::DOWN, Keys::KeypadDown),
            (coordinate + Direction::LEFT, Keys::KeypadLeft),
        ];

        for (neighbor, direction) in neighbors {
            if let Some(key) = keypad.get(&neighbor) {
                if *key != Keys::Empty && key != initial_key {
                    let mut next_path = path.clone();
                    next_path.push(direction);
                    to_visit.push_back((neighbor, next_path));
                }
            }
        }
    }

    for sub_path in paths.values_mut() {
        for path in sub_path.iter_mut() {
            path.push(Keys::KeypadA);
        }
    }
    paths
}

fn get_all_shortest_paths(keypad: &Keypad) -> HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>> {
    keypad
        .iter()
        .map(|(coord, key)| (*key, get_shortest_path(*coord, keypad)))
        .collect()
}

fn find_recursive(
    path: (Keys, Keys),
    max: usize,
    current: usize,
    shortest_numpads: &HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>>,
    shortest_controls: &HashMap<Keys, HashMap<Keys, Vec<Vec<Keys>>>>,
    memo: &mut HashMap<(Keys, Keys), HashMap<usize, usize>>,
    last_at_level: &mut HashMap<usize, Keys>,
) -> usize {
    if current == max {
        1
    } else {
        if let Some(path_cost) = memo.get(&(path.0, path.1)) {
            if let Some(cost) = path_cost.get(&current) {
                return *cost;
            }
        }

        let next_path = (if current == 0 {
            shortest_numpads
        } else {
            shortest_controls
        })
        .get(&path.0)
        .unwrap()
        .get(&path.1)
        .unwrap();

        let last = *last_at_level.entry(current).or_insert(Keys::KeypadA);
        let mut next_last = last;
        let mut total = usize::MAX;

        for possible_paths in next_path {
            let mut sub_total = 0;
            let mut previous = last;
            for part in possible_paths {
                let fragment_cost = find_recursive(
                    (previous, *part),
                    max,
                    current + 1,
                    shortest_numpads,
                    shortest_controls,
                    memo,
                    last_at_level,
                );
                previous = *part;
                sub_total += fragment_cost;
            }
            if sub_total < total {
                total = sub_total;
                next_last = *possible_paths.last().unwrap();
            }
        }

        last_at_level.insert(current, next_last);

        let saved_path = memo.entry(path).or_default();
        saved_path.insert(current, total);

        total
    }
}

fn solve(input: String, max: usize) -> usize {
    let mut complexity = 0;
    let shortest_paths_numpad = get_all_shortest_paths(&get_numpad());
    let shortest_paths_control = get_all_shortest_paths(&get_control_pad());

    for (code, value) in parse_input(input) {
        let mut previous_key = Keys::KeypadA;
        let mut len = 0;
        for key in code {
            len += find_recursive(
                (previous_key, key),
                max,
                0,
                &shortest_paths_numpad,
                &shortest_paths_control,
                &mut HashMap::new(),
                &mut HashMap::new(),
            );
            previous_key = key;
        }
        complexity += value * len;
    }

    complexity
}

fn part1(input: String) -> usize {
    solve(input, 3)
}

fn part2(input: String) -> usize {
    solve(input, 26)
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 126384);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("sample1.txt")), 154115708116294);
    }
}

fn main() {
    utils::run(21, &["input.txt"], &part1, &part2);
}
