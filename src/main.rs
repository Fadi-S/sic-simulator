use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::Read;

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
struct Code {
    lines: Vec<Line>,
    labels: HashMap<String, u32>,
    memory: HashMap<String, i16>,
    registers: HashMap<String, i16>,
}

impl Code {
    pub fn new() -> Self {
        Self {
            lines: vec![],
            labels: HashMap::new(),
            memory: HashMap::new(),
            registers: HashMap::new(),
        }
    }

    fn get_value_of(&self, operand: &Operand) -> i16 {
        match operand {
            Operand::Register(name) => {
                self.registers.get(name.as_str()).unwrap().clone()
            }
            Operand::Immediate(value) => {
                value.parse::<i16>().unwrap()
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
            println!("{:?}", line);
            index += 1;

            match line.operation.to_uppercase().as_str() {
                "LDA" => {
                    self.registers
                        .insert("A".to_string(), self.get_value_of(&line.operands[0]));
                }
                "STA" => {
                    self.set_value_of(
                        &line.operands[0],
                        self.registers.get("A").unwrap().clone()
                    );
                }
                "ADD" => {
                    self.registers
                        .insert("A".to_string(),
                                self.get_value_of(&line.operands[0]) + self.registers.get("A").unwrap().clone()
                        );
                }
                "J" => {
                    if let Operand::Memory(label) = &line.operands[0] {
                        index = *self.labels.get(label).expect(format!("Label '{}' does not exist", label).as_str()) as usize;
                    }
                }
                _ => (),
            }
        }
    }
}

const REGISTERS: [&str; 3] = ["x", "t", "s"];

fn main() -> Result<(), io::Error>{
    let mut code = compile("code.sic")?;

    code.execute();

    println!("{:?}", code);

    Ok(())
}

pub fn trim_whitespace(s: &str) -> String {
    let words: Vec<_> = s.split_whitespace().collect();
    words.join(" ")
}

fn compile(filename: &str) -> Result<Code, io::Error> {
    let mut code = String::new();
    File::open(filename)?.read_to_string(&mut code)?;

    let mut code_obj :Code = Code::new();

    'lines: for (index, line) in code.lines().enumerate() {
        let line = trim_whitespace(line.trim());
        if line == "" {
            continue;
        }

        let mut words = line.split(" ");

        let count = words.clone().count();
        let mut line = Line::new(index as u32);

        // println!("{:?}", words.clone().collect::<Vec<_>>());
        if count == 1 {
            line.label.push_str(words.next().unwrap());

            code_obj.labels.insert(line.label.clone(), line.number);
            continue;
        }

        if count == 3 {
            line.label.push_str(words.next().unwrap());

            code_obj.labels.insert(line.label.clone(), line.number);
        }
        line.operation = words.next().unwrap().to_string();

        let operands : &str;
        loop {
            match words.next() {
                Some(str) => {
                    if str != "" {
                        operands = str;
                        break;
                    }
                },
                None => continue 'lines
            }
        }

        let operands = operands.split(",");

        for operand_str in operands {
            if operand_str == "" {
                continue;
            }
            let operand;
            if operand_str.chars().collect::<Vec<_>>()[0] == '#' {
                operand = Operand::Immediate(String::from(&operand_str[1..]));
            } else if REGISTERS.contains(&&*operand_str.to_lowercase()) {
                operand = Operand::Register(operand_str.to_string());
            } else {
                operand = Operand::Memory(operand_str.to_string());
            }

            line.operands.push(operand);
        }

        code_obj.lines.push(line);
    }

    println!("{:?}", code_obj);
    Ok(code_obj)
}