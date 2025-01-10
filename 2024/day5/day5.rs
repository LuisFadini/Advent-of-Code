use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn parse_input(input: String) -> (HashMap<usize, HashSet<usize>>, Vec<Vec<usize>>) {
    let (p1, p2) = input.split_once("\n\n").unwrap();

    let mut orders = HashMap::<usize, HashSet<usize>>::new();
    for line in p1.lines() {
        let (x, y) = line.split_once("|").unwrap();
        orders
            .entry(y.parse().unwrap())
            .or_default()
            .insert(x.parse().unwrap());
    }

    let pages = p2
        .lines()
        .map(|l| {
            l.split(',')
                .map(|w| w.parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    (orders, pages)
}

fn part1(input_data: String) -> usize {
    let (orders, pages) = parse_input(input_data);

    pages
        .iter()
        .filter_map(|p| {
            if is_sorted(&orders, p) {
                Some(p[p.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

fn is_sorted(orders: &HashMap<usize, HashSet<usize>>, page: &[usize]) -> bool {
    for (i, &x) in page.iter().enumerate() {
        if let Some(dependencies) = orders.get(&x) {
            for &y in &page[i + 1..] {
                if dependencies.contains(&y) {
                    return false;
                }
            }
        }
    }
    true
}

fn part2(input_data: String) -> usize {
    let (orders, mut pages) = parse_input(input_data);

    pages
        .iter_mut()
        .filter_map(|p| {
            if !is_sorted(&orders, &p) {
                p.sort_by(|a, b| {
                    orders.get(b).map_or(Ordering::Less, |b_vals| {
                        if b_vals.contains(a) {
                            Ordering::Greater
                        } else {
                            Ordering::Less
                        }
                    })
                });

                Some(p[p.len() / 2])
            } else {
                None
            }
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("./sample1.txt")), 143);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample1.txt")), 123);
    }
}

fn main() {
    utils::run(5, &["sample1.txt", "input.txt"], &part1, &part2);
}
