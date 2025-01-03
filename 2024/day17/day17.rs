use utils;

struct Computer {
    a_reg: usize,
    b_reg: usize,
    c_reg: usize,
    instruction_pointer: usize,
    program: Vec<usize>,
    output_buffer: Vec<usize>,
}

impl Computer {
    fn new(a_reg: usize, b_reg: usize, c_reg: usize, program: Vec<usize>) -> Computer {
        Computer {
            a_reg,
            b_reg,
            c_reg,
            instruction_pointer: 0,
            program,
            output_buffer: vec![],
        }
    }

    fn run(&mut self) {
        while self.instruction_pointer < self.program.len() {
            self.execute_instruction();
        }
    }

    fn execute_instruction(&mut self) {
        let operand = self.program[self.instruction_pointer + 1];
        match self.program[self.instruction_pointer] {
            0 => self.adv(operand),
            1 => self.bxl(operand),
            2 => self.bst(operand),
            3 => self.jnz(operand),
            4 => self.bxc(),
            5 => self.out(operand),
            6 => self.bdv(operand),
            7 => self.cdv(operand),
            _ => unreachable!(),
        }
    }

    fn process_output(&self) -> String {
        self.output_buffer
            .iter()
            .map(|o| o.to_string())
            .collect::<Vec<String>>()
            .join(",")
    }

    fn combo_operand(&mut self, operand: usize) -> usize {
        match operand {
            0..=3 => operand as usize,
            4 => self.a_reg,
            5 => self.b_reg,
            6 => self.c_reg,
            _ => panic!("Invalid operand {}", operand),
        }
    }

    fn adv(&mut self, operand: usize) {
        let operand = self.combo_operand(operand);
        self.a_reg >>= operand;
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, operand: usize) {
        self.b_reg ^= operand;
        self.instruction_pointer += 2;
    }

    fn bst(&mut self, operand: usize) {
        let operand = self.combo_operand(operand);
        self.b_reg = operand & 7;
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self, operand: usize) {
        if self.a_reg != 0 {
            self.instruction_pointer = operand.try_into().unwrap();
        } else {
            self.instruction_pointer += 2;
        }
    }

    fn bxc(&mut self) {
        self.b_reg ^= self.c_reg;
        self.instruction_pointer += 2;
    }

    fn out(&mut self, operand: usize) {
        let operand = self.combo_operand(operand);
        self.output_buffer.push(operand & 7);
        self.instruction_pointer += 2;
    }

    fn bdv(&mut self, operand: usize) {
        let operand = self.combo_operand(operand);
        self.b_reg = self.a_reg >> operand;
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, operand: usize) {
        let operand = self.combo_operand(operand);
        self.c_reg = self.a_reg >> operand;
        self.instruction_pointer += 2;
    }
}

fn get_registers_and_program(input: String) -> (usize, usize, usize, Vec<usize>) {
    let (registers, program) = input.split_once("\n\n").unwrap();

    let instructions = program
        .strip_prefix("Program: ")
        .unwrap()
        .split(",")
        .map(|c| c.trim().parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let [a, b, c]: [usize; 3] = registers
        .lines()
        .map(|l| l.split_once(": ").unwrap().1.parse::<usize>().unwrap())
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    (a, b, c, instructions)
}

fn part1(input_program: String) -> String {
    let (a_register, b_register, c_register, program) = get_registers_and_program(input_program);

    let mut computer = Computer::new(a_register, b_register, c_register, program);
    computer.run();

    computer.process_output()
}

fn part2(input_program: String) -> usize {
    let (_, b_register, c_register, program) = get_registers_and_program(input_program);

    let mut a_register = 0;
    let mut counter = 1;
    let mut start_i = 0;

    while counter <= program.len() as i32 && counter >= 0 {
        a_register <<= 3;

        let mut found = false;
        for i in start_i..8 {
            let mut computer =
                Computer::new(a_register + i, b_register, c_register, program.clone());
            computer.run();

            if program[program.len().saturating_sub(counter as usize)..] == computer.output_buffer {
                found = true;
                a_register += i;
                start_i = 0;
                counter += 1;
                break;
            }
        }

        if !found {
            counter -= 1;
            a_register >>= 3;
            start_i = (a_register % 8) + 1;
            a_register >>= 3;
        }
    }

    a_register
}

#[cfg(test)]
mod test {
    use utils::read_file;

    use super::*;

    #[test]
    fn test1() {
        assert_eq!(
            part1(read_file("./sample1.txt")),
            "4,6,3,5,6,3,5,2,1,0"
        )
    }

    #[test]
    fn test2() {
        assert_eq!(part2(read_file("./sample2.txt")), 117440)
    }
}

fn main() {
    utils::run(
        17,
        &["sample1.txt", "sample2.txt", "input.txt"],
        &part1,
        &part2,
    );
}
