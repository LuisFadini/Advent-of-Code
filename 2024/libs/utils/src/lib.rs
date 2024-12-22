use std::{fs, time::Instant};

pub mod coordinates;

pub fn run<T>(day: i32, input_files: Vec<&str>, part1: &dyn Fn(String) -> T, part2: &dyn Fn(String) -> T)
where
    T: Into<i64> + std::fmt::Display,
{
    for input_file in input_files {
        let mut output: Vec<String> = vec![];

        if !fs::exists(input_file).unwrap() {
            println!("Skipping {} as the file doesn't exist on the file system", input_file);
            continue;
        }

        let input = read_file(input_file);

        println!("Running {}...", input_file);

        output.push(format!("+---------+-------------+------------+"));
        output.push(format!("| Day {:>2}  | Result      | Time Taken |", day));
        output.push(format!("+---------+-------------+------------+"));

        let start_part1 = Instant::now();
        let part1_result = part1(input.clone());
        let duration_part1 = start_part1.elapsed().as_nanos();
        output.push(format!(
            "| Part 1  | {:<11} | {:>10} |",
            part1_result,
            format_duration(duration_part1)
        ));

        let start_part2 = Instant::now();
        let part2_result = part2(input.clone());
        let duration_part2 = start_part2.elapsed().as_nanos();
        output.push(format!(
            "| Part 2  | {:<11} | {:>10} |",
            part2_result,
            format_duration(duration_part2)
        ));

        output.push(format!("+---------+-------------+------------+"));

        println!("{}", output.join("\n"));
        println!("");
    }
}

fn format_duration(duration_nanos: u128) -> String {
    if duration_nanos >= 1_000_000_000 {
        format!("{:.2} s", duration_nanos as f64 / 1_000_000_000.0)
    } else if duration_nanos >= 1_000_000 {
        format!("{:.2} ms", duration_nanos as f64 / 1_000_000.0)
    } else if duration_nanos >= 1_000 {
        format!("{:.2} us", duration_nanos as f64 / 1_000.0)
    } else {
        format!("{:.0} ns", duration_nanos)
    }
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path)
        .unwrap_or_else(|_| panic!("Could not find file: {}", path))
        .replace("\r\n", "\n")
}
