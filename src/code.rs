use std::collections::HashMap;
use cli_table::{format::Justify, Cell, Style, Table};

#[derive(Debug, Clone)]
struct Line {
    label: String,
    operation: String,
    operands: Vec<Operand>,
    number: u32,
}

impl Line {
    pub fn new(number: u32) -> Self {
        Self {
            label: String::new(),
            operation: String::new(),
            operands: vec![],
            number,
        }
    }
}

#[derive(Debug, Clone)]
enum Operand {
    Register(String),
    Immediate(String),
    Memory(String),
}

// enum Operation {
//     Lda(Operand),
//     Sta(Operand),
//
//     Add(Operand),
//     Sub(Operand),
//     Div(Operand),
//     Mul(Operand),
//
//     J(Operand),
//     Comp(Operand),
//     Jgt(Operand),
//     Jlt(Operand),
// }

#[derive(Debug)]
pub struct Code {
    lines: Vec<Line>,
    labels: HashMap<String, u32>,
    memory: HashMap<String, i16>,
    registers: HashMap<String, i16>,
    compare: Compare,
}

#[derive(Debug, PartialEq)]
enum Compare {
    Equal,
    Greater,
    Less,
    Undefined,
}

const REGISTERS: [&str; 3] = ["x", "t", "s"];

impl Code {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            labels: HashMap::new(),
            memory: HashMap::new(),
            registers: HashMap::new(),
            compare: Compare::Undefined,
        }
    }

    pub fn compile(code: &mut String) -> Self {
        let mut code_obj: Self = Self::new();

        for (index, line) in code.lines().enumerate() {
            let line = line
                .trim()
                .split_whitespace()
                .collect::<Vec<_>>()
                .join(" ");

            if line == "" { continue; }

            let mut words = line.split(" ");

            let count = words.clone().count();
            let mut line = Line::new(index as u32);

            if count == 3 || count == 1 {
                line.label.push_str(words.next().unwrap());
                code_obj.labels.insert(line.label.clone(), line.number);
            }
            if count == 1 { continue; }

            line.operation = words.next().expect(
                format!("Operation expected in line {}", index + 1).as_str()
            ).to_string();

            let operands: &str = words.next().expect(
                format!("Operand(s) expected in line {}", index + 1).as_str()
            );

            let operands = operands.split(",");
            for operand_str in operands {
                if operand_str == "" { continue; }

                let operand;
                if operand_str.chars().collect::<Vec<_>>()[0] == '#' {
                    operand = Operand::Immediate((&operand_str[1..]).to_string());
                } else if REGISTERS.contains(&operand_str.to_lowercase().as_str()) {
                    operand = Operand::Register(operand_str.to_string());
                } else {
                    operand = Operand::Memory(operand_str.to_string());
                }

                line.operands.push(operand);
            }

            code_obj.lines.push(line);
        }

        code_obj
    }

    fn get_accumulator(&self) -> i16 {
        match self.registers.get("A") {
            Some(value) => value.clone(),
            None => 0,
        }
    }

    fn set_accumulator(&mut self, value: i16) {
        self.registers.insert("A".to_string(), value);
    }

    fn get_label_index(&self, label: &str) -> usize {
         *self.labels.get(label)
                .expect(
                    format!("Label '{}' does not exist", label).as_str()
                ) as usize
    }

    fn get_value_of(&self, operand: &Operand) -> i16 {
        match operand {
            Operand::Register(name) => {
                self.registers.get(name.as_str()).expect(
                    format!("Register '{}' is empty", name.to_uppercase()).as_str()
                ).clone()
            }
            Operand::Immediate(value) => {
                value.parse::<i16>().expect(
                    format!("'{}' is not a valid immediate value", value).as_str()
                )
            }
            Operand::Memory(name) => {
                self.memory.get(name.as_str()).expect(
                    format!("No memory location with name {}", name).as_str()
                ).clone()
            }
        }
    }

    fn set_value_of(&mut self, operand: &Operand, value: i16) {
        match operand {
            Operand::Register(name) => {
                self.registers.insert(name.clone(), value);
            }
            Operand::Immediate(_) => {
                panic!("Cannot save value to immediate");
            }
            Operand::Memory(name) => {
                self.memory.insert(name.clone(), value);
            }
        }
    }

    pub fn execute(&mut self) {
        let mut index: usize = 0;
        let length = self.lines.len();
        while index < length {
            let line = &self.lines[index].clone();
            // println!("{:?}", line);
            index += 1;

            let operand_count = line.operands.len();

            match line.operation.to_uppercase().as_str() {
                "LDA" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    self.set_accumulator(self.get_value_of(&line.operands[0]));
                }
                "STA" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    self.set_value_of(
                        &line.operands[0],
                        self.get_accumulator(),
                    );
                }
                "ADD" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    self.set_accumulator(
                        self.get_accumulator() + self.get_value_of(&line.operands[0])
                    );
                }
                "SUB" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    self.set_accumulator(
                        self.get_accumulator() - self.get_value_of(&line.operands[0])
                    );
                }
                "MUL" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    self.set_accumulator(
                        self.get_accumulator() * self.get_value_of(&line.operands[0])
                    );
                }
                "DIV" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    self.set_accumulator(
                        self.get_accumulator() / self.get_value_of(&line.operands[0])
                    );
                }
                "J" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    if let Operand::Memory(label) = &line.operands[0] {
                        index = self.get_label_index(label);
                    }
                }
                "COMP" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }
                    if let Operand::Register(_name) = &line.operands[0] {
                        panic!("COMP cannot be used with a register, use COMPR instead, line {}", index);
                    }

                    let operand = self.get_value_of(&line.operands[0]);
                    let accumulator = self.get_accumulator();

                    self.compare = match accumulator {
                        acc if acc > operand => Compare::Greater,
                        acc if acc < operand => Compare::Less,
                        _ => Compare::Equal,
                    }
                }
                "COMPR" => {
                    if operand_count != 2 {
                        panic!("Expected {} operands, found {} at line {}", 2, operand_count, index);
                    }
                    if let Operand::Memory(_name) = &line.operands[0] {
                        panic!("COMPR is used with registers only, line {}", index);
                    }
                    if let Operand::Immediate(_name) = &line.operands[0] {
                        panic!("COMPR is used with registers only, line {}", index);
                    }

                    let register1 = self.get_value_of(&line.operands[0]);
                    let register2 = self.get_value_of(&line.operands[1]);

                    self.compare = match register1 {
                        reg if reg > register2 => Compare::Greater,
                        reg if reg < register2 => Compare::Less,
                        _ => Compare::Equal,
                    }
                }
                "JEQ" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    if self.compare != Compare::Equal {
                        continue;
                    }

                    if let Operand::Memory(label) = &line.operands[0] {
                        index = self.get_label_index(label);
                    }
                }
                "JGT" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    if self.compare != Compare::Greater {
                        continue;
                    }

                    if let Operand::Memory(label) = &line.operands[0] {
                        index = self.get_label_index(label);
                    }
                }
                "JLT" => {
                    if operand_count != 1 {
                        panic!("Expected {} operands, found {} at line {}", 1, operand_count, index);
                    }

                    if self.compare != Compare::Less {
                        continue;
                    }

                    if let Operand::Memory(label) = &line.operands[0] {
                        index = self.get_label_index(label);
                    }
                }
                _ => (),
            }
        }
    }

    pub fn print(&self) {
        let mut table = vec![];
        for (register, value) in &self.registers {
            table.push(vec![register.cell(), value.cell().justify(Justify::Right)]);
        }

        let table = table
            .table()
            .title(vec![
                "Register".cell().bold(true),
                "Value".cell().bold(true),
            ])
            .bold(true);

        let table_display = table.display().unwrap();

        println!("{}", table_display);

        let mut table = vec![];
        for (memory, value) in &self.memory{
            table.push(vec![memory.cell(), value.cell().justify(Justify::Right)]);
        }

        let table = table
            .table()
            .title(vec![
                "Memory Location".cell().bold(true),
                "Value".cell().bold(true),
            ])
            .bold(true);

        let table_display = table.display().unwrap();

        println!("{}", table_display);
    }
}