fn generate_secret(mut num: i64) -> i64 {
    num = ((64 * num) ^ num) % 16777216;
    num = (num ^ (num / 32)) % 16777216;
    num = ((num * 2048) ^ num) % 16777216;
    num
}

fn part1(input: String) -> i64 {
    let buyers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let mut total = 0;

    let mut chunk = [0i64; 16];
    let mut chunk_index = 0;

    for &buyer in &buyers {
        chunk[chunk_index] = buyer;
        chunk_index += 1;

        if chunk_index == 16 {
            let mut temp_chunk = chunk;
            for _ in 0..2000 {
                for i in 0..16 {
                    temp_chunk[i] = generate_secret(temp_chunk[i]);
                }
            }
            total += temp_chunk.iter().sum::<i64>();
            chunk_index = 0;
        }
    }

    for i in 0..chunk_index {
        let mut buyer = chunk[i];
        for _ in 0..2000 {
            buyer = generate_secret(buyer);
        }
        total += buyer;
    }

    total
}

fn part2(input: String) -> i32 {
    let buyers = input
        .lines()
        .map(|l| l.parse::<i64>().unwrap())
        .collect::<Vec<_>>();

    let size = 19usize.pow(4);
    let mut map = vec![0; size];

    for mut buyer in buyers {
        let mut seen = vec![0u8; (size + 7) / 8];
        let mut deltas = [0i8; 4];
        let mut old_price = (buyer % 10) as i8;

        for _ in 0..2000 {
            buyer = generate_secret(buyer);
            let price = (buyer % 10) as i8;

            let delta = price - old_price;
            old_price = price;

            deltas[0] = deltas[1];
            deltas[1] = deltas[2];
            deltas[2] = deltas[3];
            deltas[3] = delta;

            if deltas.iter().any(|&x| x == 0) {
                continue;
            }

            let index = (deltas[0] + 9) as usize * 19usize.pow(3)
                + (deltas[1] + 9) as usize * 19usize.pow(2)
                + (deltas[2] + 9) as usize * 19usize
                + (deltas[3] + 9) as usize;

            let byte_index = index / 8;
            let bit_index = index % 8;
            if seen[byte_index] & (1 << bit_index) != 0 {
                continue;
            }
            seen[byte_index] |= 1 << bit_index;
            map[index] += price as i32;
        }
    }

    map.into_iter().max().unwrap()
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 37327623);
    }
}

fn main() {
    utils::run(22, &["sample1.txt", "input.txt"], &part1, &part2);
}
