use std::{
    collections::{HashMap, HashSet}, env, fs, process::exit, usize
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let file_content = fs::read_to_string("./sample1.txt").unwrap();
        assert_eq!(part1(file_content), 143);
    }
}

fn main() {
    let file_content = fs::read_to_string("./sample1.txt").unwrap();
    println!("{}", part1(file_content));
    
    // let input_path = env::args().nth(1);
    // if input_path.is_none() {
    //     println!("Input path should be specified!");
    //     exit(1);
    // }

    // println!(
    //     "Output: {}",
    //     part1(fs::read_to_string(input_path.unwrap()).unwrap())
    // );
}
