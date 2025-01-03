use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
};

fn part1(input_data: String) -> i32 {
    let (p1, p2) = input_data.split_once("\n\n").unwrap();

    let mut orders = HashMap::<usize, HashSet<usize>>::new();
    for line in p1.lines() {
        let (x, y) = line.split_once("|").unwrap();
        orders
            .entry(y.parse().unwrap())
            .or_default()
            .insert(x.parse().unwrap());
    }

    let pages = p2.lines().map(|l| {
        l.split(',')
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut out_sum = 0;
    for page in pages {
        if is_sorted(&orders, &page) {
            out_sum += page[page.len() / 2];
        }
    }

    return out_sum.try_into().unwrap();
}

fn is_sorted(orders: &HashMap<usize, HashSet<usize>>, page: &[usize]) -> bool {
    let mut ok = true;
    for (i, x) in page.iter().enumerate() {
        for y in &page[i + 1..] {
            if orders.get(x).is_some() {
                if orders.get(x).unwrap().contains(y) {
                    ok = false;
                }
            }
        }
    }
    ok
}

fn part2(input_data: String) -> i32 {
    let (p1, p2) = input_data.split_once("\n\n").unwrap();

    let mut orders = HashMap::<usize, HashSet<usize>>::new();
    for line in p1.lines() {
        let (x, y) = line.split_once("|").unwrap();
        orders
            .entry(y.parse().unwrap())
            .or_default()
            .insert(x.parse().unwrap());
    }

    let pages = p2.lines().map(|l| {
        l.split(',')
            .map(|w| w.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
    });

    let mut out_sum = 0;
    for mut page in pages {
        if !is_sorted(&orders, &page) {
            page.sort_by(|a, b| {
                if let Some(b_vals) = orders.get(b) {
                    if b_vals.contains(a) {
                        Ordering::Greater
                    } else {
                        Ordering::Less
                    }
                } else {
                    Ordering::Less
                }
            });

            out_sum += page[page.len() / 2];
        }
    }

    out_sum.try_into().unwrap()
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
