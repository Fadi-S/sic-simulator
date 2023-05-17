use std::collections::HashMap;
use std::ops::{Add, Div, Mul, Sub};
use cli_table::{format::Justify, Cell, Style, Table};

#[derive(Debug, Clone)]
struct Line {
    label: String,
    operation: Operation,
    operands: Vec<Operand>,
    number: usize,
}

impl Line {
    pub fn new(number: usize) -> Self {
        Self {
            label: String::new(),
            operation: Operation::None,
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

#[derive(Debug, Clone, PartialEq)]
enum Operation {
    Arithmetic(ArithmeticOperation),
    ArithmeticWithOffset(ArithmeticOperation),
    ArithmeticRegisters(ArithmeticOperation),
    Branch(BranchOperation),
    Compare,
    CompareRegisters,
    Load(String),
    Store(String),
    LoadWithOffset(String),
    StoreWithOffset(String),
    Exchange,
    None,
}

#[derive(Debug, Clone, PartialEq)]
enum ArithmeticOperation {
    ADD,
    SUB,
    MUL,
    DIV,
}

impl ArithmeticOperation {
    fn calculate<T: Add<Output=T> + Sub<Output=T> + Mul<Output=T> + Div<Output=T>>(&self, value1: T, value2: T) -> T {
        match self {
            ArithmeticOperation::ADD => value1 + value2,
            ArithmeticOperation::SUB => value1 - value2,
            ArithmeticOperation::MUL => value1 * value2,
            ArithmeticOperation::DIV => value1 / value2,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
enum BranchOperation {
    J,
    JEQ,
    JGT,
    JLT,
}

impl BranchOperation {
    fn should_branch(&self, compare: &Compare) -> bool {
        match self {
            BranchOperation::J => true,
            BranchOperation::JEQ => compare == &Compare::Equal,
            BranchOperation::JGT => compare == &Compare::Greater,
            BranchOperation::JLT => compare == &Compare::Less,
        }
    }
}

#[derive(Debug, Clone)]
enum MemoryType {
    Integer(i16),
    Array(Vec<i16>, Size),
}

#[derive(Debug, Clone)]
enum Size {
    Byte(usize),
    Word(usize),
}

#[derive(Debug)]
pub struct Code {
    lines: Vec<Line>,
    labels: HashMap<String, usize>,
    memory: HashMap<String, MemoryType>,
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

impl Compare {
    fn from_values<T: PartialOrd>(value1: T, value2: T) -> Self {
        match value1 {
            val1 if val1 > value2 => Compare::Greater,
            val1 if val1 < value2 => Compare::Less,
            _ => Compare::Equal,
        }
    }
}

const REGISTERS: [&str; 7] = ["a", "x", "l", "b", "s", "t", "f"];

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
            let mut line = Line::new(index);

            if count == 3 || count == 1 {
                line.label.push_str(words.next().unwrap());
                code_obj.labels.insert(line.label.clone(), line.number);
            }
            if count == 1 { continue; }

            let operation = words.next().expect(
                format!("Operation expected in line {}", index + 1).as_str()
            ).to_string().to_uppercase();

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

            let operands_count = line.operands.len();
            match operation.as_str() {
                "RESW" => {
                    if let Operand::Memory(value) = &line.operands[0] {
                        let size = value.parse::<usize>().expect(
                            format!("'{}' is not a valid RESW value", value).as_str()
                        );

                        code_obj.memory.insert(
                            line.label.clone(),
                            MemoryType::Array(
                                vec![1; size],
                                Size::Word(size),
                            ));
                    }
                    continue;
                }
                "RESB" => {
                    if let Operand::Memory(value) = &line.operands[0] {
                        let size = value.parse::<usize>().expect(
                            format!("'{}' is not a valid RESB value", value).as_str()
                        );

                        code_obj.memory.insert(
                            line.label.clone(),
                            MemoryType::Array(
                                vec![0; size],
                                Size::Byte(size),
                            ));
                    }
                    continue;
                }
                "WORD" => {
                    let mut word: Vec<i16> = vec![];
                    for operand in &line.operands {
                        let value = match operand {
                            Operand::Memory(value) => value,
                            _ => panic!("Invalid value for WORD, line {}", index+1),
                        };
                        let value = value.parse::<i16>().expect(
                            format!("Invalid value for WORD, line {}", index+1).as_str()
                        );
                        word.push(value);
                    }

                    let size = (&word).len();
                    code_obj.memory.insert(
                        line.label.clone(),
                        MemoryType::Array(
                            word,
                            Size::Word(size),
                        ));
                    continue;
                }
                "BYTE" => {
                    let mut byte: Vec<i16> = vec![];
                    for operand in &line.operands {
                        let value = match operand {
                            Operand::Memory(value) => value,
                            _ => panic!("Invalid value for Byte, line {}", index+1),
                        };
                        let value = value.parse::<i16>().expect(
                            format!("Invalid value for Byte, line {}", index+1).as_str()
                        );
                        byte.push(value);
                    }

                    let size = (&byte).len();
                    code_obj.memory.insert(
                        line.label.clone(),
                        MemoryType::Array(
                            byte,
                            Size::Byte(size),
                        ));
                    continue;
                }
                _ => ()
            }

            if operands_count == 1 {
                line.operation = match operation.as_str() {
                    "ADD" => Operation::Arithmetic(ArithmeticOperation::ADD),
                    "SUB" => Operation::Arithmetic(ArithmeticOperation::SUB),
                    "DIV" => Operation::Arithmetic(ArithmeticOperation::DIV),
                    "MUL" => Operation::Arithmetic(ArithmeticOperation::MUL),
                    "J" => Operation::Branch(BranchOperation::J),
                    "JEQ" => Operation::Branch(BranchOperation::JEQ),
                    "JGT" => Operation::Branch(BranchOperation::JGT),
                    "JLT" => Operation::Branch(BranchOperation::JLT),
                    "COMP" => Operation::Compare,
                    "LDA" => Operation::Load("A".to_string()),
                    "LDS" => Operation::Load("S".to_string()),
                    "LDT" => Operation::Load("T".to_string()),
                    "LDX" => Operation::Load("X".to_string()),
                    "STA" => Operation::Store("A".to_string()),
                    "STS" => Operation::Store("S".to_string()),
                    "STT" => Operation::Store("T".to_string()),
                    "STX" => Operation::Store("X".to_string()),
                    _ => Operation::None,
                };
            } else if operands_count == 2 {
                line.operation = match operation.as_str() {
                    "ADD" => Operation::ArithmeticWithOffset(ArithmeticOperation::ADD),
                    "SUB" => Operation::ArithmeticWithOffset(ArithmeticOperation::SUB),
                    "DIV" => Operation::ArithmeticWithOffset(ArithmeticOperation::DIV),
                    "MUL" => Operation::ArithmeticWithOffset(ArithmeticOperation::MUL),
                    "ADDR" => Operation::ArithmeticRegisters(ArithmeticOperation::ADD),
                    "SUBR" => Operation::ArithmeticRegisters(ArithmeticOperation::SUB),
                    "DIVR" => Operation::ArithmeticRegisters(ArithmeticOperation::DIV),
                    "MULR" => Operation::ArithmeticRegisters(ArithmeticOperation::MUL),
                    "COMPR" => Operation::CompareRegisters,
                    "RMO" => Operation::Exchange,
                    "LDA" => Operation::LoadWithOffset("A".to_string()),
                    "LDS" => Operation::LoadWithOffset("S".to_string()),
                    "LDT" => Operation::LoadWithOffset("T".to_string()),
                    "LDX" => Operation::LoadWithOffset("X".to_string()),
                    "STA" => Operation::StoreWithOffset("A".to_string()),
                    "STS" => Operation::StoreWithOffset("S".to_string()),
                    "STT" => Operation::StoreWithOffset("T".to_string()),
                    "STX" => Operation::StoreWithOffset("X".to_string()),
                    _ => Operation::None,
                };
            }

            if line.operation == Operation::None {
                panic!("Operation and number of operands are incompatible, line {}", index + 1);
            }

            code_obj.lines.push(line);
        }

        code_obj
    }

    fn get_accumulator(&self) -> i16 {
        self.get_register("A")
    }

    fn set_accumulator(&mut self, value: i16) {
        self.set_register("A", value);
    }

    fn set_register(&mut self, register: &str, value: i16) {
        self.registers.insert(register.to_string(), value);
    }

    fn get_register(&self, register: &str) -> i16 {
        match self.registers.get(register) {
            Some(value) => value.clone(),
            None => 0,
        }
    }

    fn get_label_index(&self, label: &str) -> usize {
        *self.labels.get(label)
            .expect(
                format!("Label '{}' does not exist", label).as_str()
            ) as usize
    }

    fn get_value_of(&self, operand: &Operand, offset: Option<usize>) -> i16 {
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
                match self.memory.get(name.as_str()).expect(
                    format!("No memory location with name {}", name).as_str()
                ) {
                    MemoryType::Integer(val) => val.clone(),
                    MemoryType::Array(vector, size) => {
                        let offset = offset.unwrap_or(0);

                        let s = match size {
                            Size::Byte(_) => 1,
                            Size::Word(_) => 3,
                        };

                        vector[offset / s]
                    }
                }
            }
        }
    }

    fn set_value_of(&mut self, operand: &Operand, value: i16, offset: Option<usize>) {
        match operand {
            Operand::Register(name) => {
                self.registers.insert(name.clone(), value);
            }
            Operand::Immediate(_) => {
                panic!("Cannot save value to immediate");
            }
            Operand::Memory(name) => {
                match offset {
                    None => {
                        self.memory.insert(name.clone(), MemoryType::Integer(value));
                    }
                    Some(offset) => {
                        let mem = self.memory.get_mut(name.clone().as_str())
                            .expect(format!("Memory {} not found", name).as_str());

                        if let MemoryType::Array(vec, size) = mem {
                            let s = match size {
                                Size::Byte(_) => 1,
                                Size::Word(_) => 3,
                            };

                            vec[offset / s] = value;
                        }
                    }
                };
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

            match &line.operation {
                Operation::Arithmetic(operation) => {
                    let accumulator = self.get_accumulator();
                    let value = self.get_value_of(&line.operands[0], None);

                    self.set_accumulator(operation.calculate(accumulator, value));
                }
                Operation::ArithmeticWithOffset(operation) => {
                    let accumulator = self.get_accumulator();
                    let value = self.get_value_of(&line.operands[0], Some(self.get_value_of(&line.operands[1], None) as usize));

                    self.set_accumulator(operation.calculate(accumulator, value));
                }
                Operation::ArithmeticRegisters(operation) => {
                    let register1 = self.get_value_of(&line.operands[0], None);
                    let register2 = self.get_value_of(&line.operands[1], None);

                    if let Operand::Register(register) = &line.operands[1] {
                        self.set_register(register, operation.calculate(register1, register2));
                    }
                }
                Operation::Branch(operation) => {
                    if !operation.should_branch(&self.compare) { continue; }

                    if let Operand::Memory(label) = &line.operands[0] {
                        index = self.get_label_index(label);
                    }
                }
                Operation::Compare => {
                    let operand = self.get_value_of(&line.operands[0], None);
                    let accumulator = self.get_accumulator();

                    self.compare = Compare::from_values(accumulator, operand);
                }
                Operation::CompareRegisters => {
                    let register1 = self.get_value_of(&line.operands[0], None);
                    let register2 = self.get_value_of(&line.operands[1], None);

                    self.compare = Compare::from_values(register1, register2);
                }
                Operation::Load(register) => self.set_register(register, self.get_value_of(&line.operands[0], None)),
                Operation::Store(register) => self.set_value_of(
                    &line.operands[0],
                    self.get_register(register),
                    None,
                ),
                Operation::LoadWithOffset(register) => {
                    self.set_register(
                        register,
                        self.get_value_of(
                            &line.operands[0],
                            Some(self.get_value_of(&line.operands[1], None) as usize)
                        )
                    )
                }
                Operation::StoreWithOffset(register) => {
                    self.set_value_of(
                        &line.operands[0],
                        self.get_register(register),
                        Some(self.get_value_of(&line.operands[1], None) as usize)
                    )
                }
                Operation::Exchange => {
                    if let Operand::Register(reg1) = &line.operands[0] {
                        let value = self.get_register(reg1);

                        if let Operand::Register(reg2) = &line.operands[1] {
                            self.set_register(reg1, self.get_register(reg2));

                            self.set_register(reg2, value);
                        } else {
                            panic!("RMO works with registers only, line {}", index);
                        }
                    } else {
                        panic!("RMO works with registers only, line {}", index);
                    }
                }
                Operation::None => {}
            }
        }
    }

    pub fn print(&self) {
        let mut table = vec![];
        for register in REGISTERS {
            let value = match &self.registers.get(register.to_uppercase().as_str()) {
                None => String::new(),
                Some(value) => (&value).to_string(),
            };

            table.push(
                vec![
                    register.to_uppercase().cell(),
                    value.cell().justify(Justify::Right)
                ]
            );
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

        println!("Memory: ");
        for (memory, value) in &self.memory {
            let val = match value {
                MemoryType::Integer(v) => v.to_string(),
                MemoryType::Array(vec, _) => vec.iter().map(|&id| id.to_string() + ",").collect(),
            };

            println!("{}: {}", memory, val);
        }
    }
}