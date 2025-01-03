use std::{fs, time::Instant};

pub mod coordinates;

pub fn run<T1, T2>(
    day: i32,
    input_files: &[&str],
    part1: &dyn Fn(String) -> T1,
    part2: &dyn Fn(String) -> T2,
) where
    T1: std::fmt::Display,
    T2: std::fmt::Display,
{
    for input_file in input_files {
        let mut output: Vec<String> = vec![];

        if !fs::metadata(input_file).is_ok() {
            println!("Skipping {} as the file doesn't exist on the file system", input_file);
            continue;
        }

        let input = read_file(input_file);

        println!("Running {}...", input_file);

        // Collect results
        let start_part1 = Instant::now();
        let part1_result = part1(input.clone());
        let duration_part1 = start_part1.elapsed().as_nanos();

        let start_part2 = Instant::now();
        let part2_result = part2(input.clone());
        let duration_part2 = start_part2.elapsed().as_nanos();

        // Determine column widths dynamically
        let day_column = format!("Day {}", day);
        let part1_result_str = part1_result.to_string();
        let part2_result_str = part2_result.to_string();
        let time1 = format_duration(duration_part1);
        let time2 = format_duration(duration_part2);

        let day_width = day_column.len().max("Part 2".len());
        let result_width = "Result".len()
            .max(part1_result_str.len())
            .max(part2_result_str.len());
        let time_width = "Time Taken".len().max(time1.len()).max(time2.len());

        // Draw table header
        output.push(format!(
            "+-{:-<day_width$}-+-{:-<result_width$}-+-{:-<time_width$}-+",
            "", "", ""
        ));
        output.push(format!(
            "| {:^day_width$} | {:^result_width$} | {:^time_width$} |",
            day_column, "Result", "Time Taken"
        ));
        output.push(format!(
            "+-{:-<day_width$}-+-{:-<result_width$}-+-{:-<time_width$}-+",
            "", "", ""
        ));

        // Part 1 row
        output.push(format!(
            "| {:<day_width$} | {:<result_width$} | {:>time_width$} |",
            "Part 1", part1_result_str, time1
        ));

        // Part 2 row
        output.push(format!(
            "| {:<day_width$} | {:<result_width$} | {:>time_width$} |",
            "Part 2", part2_result_str, time2
        ));

        // Draw table footer
        output.push(format!(
            "+-{:-<day_width$}-+-{:-<result_width$}-+-{:-<time_width$}-+",
            "", "", ""
        ));

        // Print table
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
