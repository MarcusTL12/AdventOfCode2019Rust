use std::{
    error::Error,
    ops::{Index, IndexMut},
};

use crossbeam::channel::{self, Receiver, Sender};

#[derive(Debug)]
pub struct IntcodeMachine {
    data: Vec<i64>,
    pc: usize,
    rb: usize,
    stdin: Receiver<i64>,
    stdout: Sender<i64>,
}

#[derive(Debug)]
enum Parameter {
    Address(usize),
    Immediate(i64),
    Relative(isize),
}

impl Parameter {
    fn new(flag: u8, data: i64) -> Self {
        match flag {
            0 => Self::Address(data as usize),
            1 => Self::Immediate(data),
            2 => Self::Relative(data as isize),
            _ => panic!("Illegal parameter flag: {flag}"),
        }
    }
}

#[derive(Debug)]
enum Instruction {
    Add(Parameter, Parameter, Parameter),
    Mul(Parameter, Parameter, Parameter),
    Input(Parameter),
    Output(Parameter),
    JumpTrue(Parameter, Parameter),
    JumpFalse(Parameter, Parameter),
    IsLess(Parameter, Parameter, Parameter),
    IsEq(Parameter, Parameter, Parameter),
    AdjustRB(Parameter),
    Halt,
}

pub fn parse_intcode_program(input: &str) -> Vec<i64> {
    input
        .trim_ascii_end()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect()
}

impl IntcodeMachine {
    pub fn new(
        data: Vec<i64>,
        stdin: Receiver<i64>,
        stdout: Sender<i64>,
    ) -> Self {
        Self {
            data,
            pc: 0,
            rb: 0,
            stdin,
            stdout,
        }
    }

    pub fn new_with_channel(
        data: Vec<i64>,
    ) -> (Self, Sender<i64>, Receiver<i64>) {
        let (stdin_s, stdin) = channel::unbounded();
        let (stdout, stdout_r) = channel::unbounded();

        (Self::new(data, stdin, stdout), stdin_s, stdout_r)
    }

    pub fn load_progam(&mut self, data: &[i64]) {
        self.data.clear();
        self.data.extend_from_slice(data);
        self.pc = 0;
        self.rb = 0;
    }

    fn fetch(&self) -> Instruction {
        let opdata = self[self.pc];
        let (opdata, opcode) = (opdata / 100, opdata % 100);
        let (opdata, flag1) = (opdata / 10, (opdata % 10) as u8);
        let (opdata, flag2) = (opdata / 10, (opdata % 10) as u8);
        let flag3 = (opdata % 10) as u8;

        match opcode {
            99 => Instruction::Halt,
            1 => Instruction::Add(
                Parameter::new(flag1, self[self.pc + 1]),
                Parameter::new(flag2, self[self.pc + 2]),
                Parameter::new(flag3, self[self.pc + 3]),
            ),
            2 => Instruction::Mul(
                Parameter::new(flag1, self[self.pc + 1]),
                Parameter::new(flag2, self[self.pc + 2]),
                Parameter::new(flag3, self[self.pc + 3]),
            ),
            3 => Instruction::Input(Parameter::new(flag1, self[self.pc + 1])),
            4 => Instruction::Output(Parameter::new(flag1, self[self.pc + 1])),
            5 => Instruction::JumpTrue(
                Parameter::new(flag1, self[self.pc + 1]),
                Parameter::new(flag2, self[self.pc + 2]),
            ),
            6 => Instruction::JumpFalse(
                Parameter::new(flag1, self[self.pc + 1]),
                Parameter::new(flag2, self[self.pc + 2]),
            ),
            7 => Instruction::IsLess(
                Parameter::new(flag1, self[self.pc + 1]),
                Parameter::new(flag2, self[self.pc + 2]),
                Parameter::new(flag3, self[self.pc + 3]),
            ),
            8 => Instruction::IsEq(
                Parameter::new(flag1, self[self.pc + 1]),
                Parameter::new(flag2, self[self.pc + 2]),
                Parameter::new(flag3, self[self.pc + 3]),
            ),
            9 => {
                Instruction::AdjustRB(Parameter::new(flag1, self[self.pc + 1]))
            }
            _ => panic!("Illegal opcode: {opcode}"),
        }
    }

    fn get(&self, param: Parameter) -> i64 {
        match param {
            Parameter::Immediate(data) => data,
            Parameter::Address(index) => self[index],
            Parameter::Relative(offset) => {
                self[(self.rb as isize + offset) as usize]
            }
        }
    }

    fn set(&mut self, param: Parameter, value: i64) {
        match param {
            Parameter::Immediate(_) => {
                panic!("Cannot write to immediate value!")
            }
            Parameter::Address(index) => self[index] = value,
            Parameter::Relative(offset) => {
                let index = (self.rb as isize + offset) as usize;
                self[index] = value;
            }
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        loop {
            match self.fetch() {
                Instruction::Add(a, b, c) => {
                    self.set(c, self.get(a) + self.get(b));
                    self.pc += 4;
                }
                Instruction::Mul(a, b, c) => {
                    self.set(c, self.get(a) * self.get(b));
                    self.pc += 4;
                }
                Instruction::Input(a) => {
                    self.set(a, self.stdin.recv()?);
                    self.pc += 2;
                }
                Instruction::Output(a) => {
                    self.stdout.send(self.get(a))?;
                    self.pc += 2;
                }
                Instruction::Halt => return Ok(()),
                Instruction::JumpTrue(a, b) => {
                    if self.get(a) != 0 {
                        self.pc = self.get(b) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::JumpFalse(a, b) => {
                    if self.get(a) == 0 {
                        self.pc = self.get(b) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                Instruction::IsLess(a, b, c) => {
                    self.set(c, if self.get(a) < self.get(b) { 1 } else { 0 });
                    self.pc += 4;
                }
                Instruction::IsEq(a, b, c) => {
                    self.set(c, if self.get(a) == self.get(b) { 1 } else { 0 });
                    self.pc += 4;
                }
                Instruction::AdjustRB(a) => {
                    self.rb =
                        (self.rb as isize + self.get(a) as isize) as usize;
                    self.pc += 2;
                }
            }
        }
    }
}

impl Index<usize> for IntcodeMachine {
    type Output = i64;

    fn index(&self, index: usize) -> &Self::Output {
        self.data.get(index).unwrap_or(&0)
    }
}

impl IndexMut<usize> for IntcodeMachine {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        if index >= self.data.len() {
            self.data.resize(index + 1, 0);
        }

        &mut self.data[index]
    }
}
