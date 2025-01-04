use std::collections::{HashMap, HashSet, VecDeque};

fn bron_kerbosch<'a>(
    curr_clique: &HashSet<&'a str>,
    can_be_added: &mut HashSet<&'a str>,
    cannot_be_added: &mut HashSet<&'a str>,
    edges: &HashMap<&'a str, HashSet<&'a str>>,
) -> HashSet<&'a str> {
    if can_be_added.is_empty() && cannot_be_added.is_empty() {
        return curr_clique.clone();
    }

    let mut largest_clique = HashSet::new();
    for vertex in can_be_added.clone() {
        let mut curr_clique = curr_clique.clone();
        curr_clique.insert(vertex);

        let neighbors = edges.get(vertex).unwrap();

        let clique = bron_kerbosch(
            &curr_clique,
            &mut can_be_added.intersection(neighbors).cloned().collect(),
            &mut cannot_be_added.intersection(neighbors).cloned().collect(),
            &edges,
        );
        if clique.len() > largest_clique.len() {
            largest_clique = clique;
        }

        can_be_added.remove(vertex);
        cannot_be_added.insert(vertex);
    }

    largest_clique
}

fn find_cliques_n<'a>(
    vertices: &HashSet<&'a str>,
    edges: &HashMap<&'a str, HashSet<&'a str>>,
    n: usize,
) -> HashSet<Vec<&'a str>> {
    let mut cliques = HashSet::new();

    let mut queue = vertices
        .into_iter()
        .map(|&v| (v, vec![v]))
        .collect::<VecDeque<_>>();
    while let Some((vertex, mut clique)) = queue.pop_front() {
        if clique.len() == n {
            clique.sort();
            cliques.insert(clique.clone());
            continue;
        }

        let mut neighbors = edges.get(vertex).unwrap().clone();
        neighbors.retain(|neighbor| {
            clique
                .iter()
                .all(|v| edges.get(v).unwrap().contains(neighbor))
        });

        for neighbor in neighbors {
            let mut new_clique = clique.clone();
            new_clique.push(neighbor);
            queue.push_back((neighbor, new_clique));
        }
    }

    cliques
}
fn parse_input<'a>(input: &'a String) -> (HashSet<&'a str>, HashMap<&'a str, HashSet<&'a str>>) {
    input.lines().fold(
        (HashSet::new(), HashMap::new()),
        |(mut vertices, mut edges), line| {
            let (left, right) = line.split_once('-').unwrap();
            vertices.insert(left);
            vertices.insert(right);

            edges.entry(left).or_insert_with(HashSet::new).insert(right);
            edges.entry(right).or_insert_with(HashSet::new).insert(left);

            (vertices, edges)
        },
    )
}

fn part1(input: String) -> usize {
    let (vertices, edges) = parse_input(&input);

    find_cliques_n(&vertices, &edges, 3)
        .iter()
        .filter(|clique| clique.iter().find(|v| v.starts_with('t')).is_some())
        .count()
}

fn part2(input: String) -> String {
    let (vertices, edges) = parse_input(&input);

    let largest_clique = bron_kerbosch(
        &HashSet::new(),
        &mut vertices.iter().cloned().collect(),
        &mut HashSet::new(),
        &edges,
    );

    let mut largest_clique = largest_clique.iter().cloned().collect::<Vec<_>>();
    largest_clique.sort();
    largest_clique.join(",")
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 7);
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("sample1.txt")), "co,de,ka,ta");
    }
}

fn main() {
    utils::run(23, &["sample1.txt", "input.txt"], &part1, &part2);
}
