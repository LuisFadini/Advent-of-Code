use std::collections::HashMap;

#[derive(PartialEq, PartialOrd, Ord, Eq, Clone, Copy, Hash)]
enum Operator {
    And,
    Or,
    Xor,
}

impl Operator {
    fn from_str(to_parse: &str) -> Operator {
        match to_parse {
            "AND" => Operator::And,
            "OR" => Operator::Or,
            "XOR" => Operator::Xor,
            _ => panic!("Invalid Operator"),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Gate {
    operator: Operator,
    wire1: String,
    wire2: String,
    wire3: String,
}

impl Gate {
    fn new(operator: &str, wire1: &str, wire2: &str, wire3: &str) -> Gate {
        Gate {
            operator: Operator::from_str(operator),
            wire1: wire1.to_string(),
            wire2: wire2.to_string(),
            wire3: wire3.to_string(),
        }
    }
}

fn parse_input(input: String) -> (Vec<Gate>, HashMap<String, i32>) {
    let (wires, gates) = input.split_once("\n\n").unwrap();
    let mut all_gates = Vec::new();
    let mut all_wires = HashMap::new();

    for gate in gates.lines() {
        let parts: Vec<_> = gate.split_whitespace().collect();

        let (op, wire1, wire2, wire3) = (parts[1], parts[0], parts[2], parts[4]);

        let gate_obj = Gate::new(op, wire1, &wire2, &wire3);

        all_gates.push(gate_obj);

        all_wires.insert(wire1.to_string(), -1);
        all_wires.insert(wire2.to_string(), -1);
        all_wires.insert(wire3.to_string(), -1);
    }

    for wire in wires.lines() {
        let (wire_name, value) = wire.split_once(": ").unwrap();
        all_wires.insert(wire_name.to_string(), value.parse().unwrap());
    }

    (all_gates, all_wires)
}

fn run_once(gates: Vec<Gate>, wires: &mut HashMap<String, i32>) -> Vec<Gate> {
    let mut to_run = Vec::new();

    for gate in gates {
        let Gate {
            ref wire1,
            ref wire2,
            ref wire3,
            operator,
        } = gate;

        let (value1, value2) = (
            wires.get(wire1).unwrap_or(&-1),
            wires.get(wire2).unwrap_or(&-1),
        );

        if *value1 == -1 || *value2 == -1 {
            to_run.push(gate);
            continue;
        }

        let output = match operator {
            Operator::And if value1 + value2 == 2 => 1,
            Operator::Or if value1 + value2 != 0 => 1,
            Operator::Xor if value1 + value2 == 1 => 1,
            _ => 0,
        };

        wires.insert(wire3.to_string(), output);
    }

    return to_run;
}

fn calculate_result(wires: HashMap<String, i32>) -> i64 {
    let mut binary_digits = Vec::new();
    let mut reversed_pos = 0;

    loop {
        let prefix = if reversed_pos < 10 { "0" } else { "" };
        let wire = format!("z{}{}", prefix, reversed_pos);
        if let Some(value) = wires.get(&wire) {
            binary_digits.push(value);
        } else {
            break;
        }
        reversed_pos += 1;
    }

    let binary = binary_digits
        .into_iter()
        .rev()
        .map(|&d| std::char::from_digit(d as u32, 10).unwrap())
        .collect::<String>();
    i64::from_str_radix(&binary, 2).unwrap()
}

fn part1(input: String) -> i64 {
    let (mut all_gates, mut all_wires) = parse_input(input);

    while !all_gates.is_empty() {
        all_gates = run_once(all_gates, &mut all_wires);
    }

    calculate_result(all_wires)
}

fn part2(input: String) -> String {
    let (_, connections) = input.split_once("\n\n").unwrap();
    let mut wire_map: HashMap<String, Vec<(Operator, String)>> = HashMap::default();
    let gate_connections: Vec<Gate> = connections
        .lines()
        .map(|gate| {
            let parts: Vec<_> = gate.split_whitespace().collect();
            Gate::new(parts[1], parts[0], parts[2], parts[4])
        })
        .collect();

    for gate in &gate_connections {
        for (wire, wire3) in [
            (gate.wire1.clone(), gate.wire3.clone()),
            (gate.wire2.clone(), gate.wire3.clone()),
        ] {
            wire_map
                .entry(wire)
                .or_insert(vec![])
                .push((gate.operator, wire3));
        }
    }

    let mut wrong_outputs = vec![];
    for gate in gate_connections.iter() {
        let chained_ops_contain = |operation| {
            wire_map
                .get(&gate.wire3)
                .is_some_and(|wire_values| wire_values.iter().any(|value| value.0 == operation))
        };

        let (has_chained_xor, has_chained_and, has_chained_or) = (
            chained_ops_contain(Operator::Xor),
            chained_ops_contain(Operator::And),
            chained_ops_contain(Operator::Or),
        );

        let takes_first_input = gate.wire1.ends_with("00") && gate.wire2.ends_with("00");
        let takes_input_bit = (gate.wire1.starts_with('x') && gate.wire2.starts_with('y'))
            || (gate.wire2.starts_with('x') && gate.wire1.starts_with('y'));
        let (outputs_bit, outputs_last_bit) = (gate.wire3.starts_with('z'), gate.wire3 == "z45");

        let valid = match gate.operator {
            Operator::Xor => {
                !takes_input_bit && outputs_bit
                    || takes_input_bit && has_chained_xor
                    || takes_first_input && outputs_bit
            }
            Operator::Or => outputs_last_bit || (has_chained_and && has_chained_xor),
            Operator::And => has_chained_or || takes_first_input,
        };

        if !valid {
            wrong_outputs.push(gate.wire3.clone());
        }
    }

    wrong_outputs.sort();
    wrong_outputs.join(",")
}

#[cfg(test)]
mod tests {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(part1(read_file("sample1.txt")), 2024);
    }
}

fn main() {
    utils::run(24, &["input.txt"], &part1, &part2);
}
