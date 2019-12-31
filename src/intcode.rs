use std::collections::VecDeque;

use std::fs::File;
use std::io::{BufRead, BufReader};

use num::pow;

#[derive(Clone)]
pub struct IntCodeMachine {
    program: Vec<i64>,
    inputqueue: VecDeque<i64>,
    outputqueue: VecDeque<i64>,
    pc: usize,
    relbase: i64,
}
//
impl IntCodeMachine {
    pub fn new(program: &Vec<i64>) -> IntCodeMachine {
        IntCodeMachine {
            program: program.clone(),
            inputqueue: VecDeque::new(),
            outputqueue: VecDeque::new(),
            pc: 0,
            relbase: 0,
        }
    }
    pub fn from_string(s: String) -> IntCodeMachine {
        IntCodeMachine::new(&s.split(',').map(|x| x.parse().unwrap()).collect())
    }
    pub fn from_file(filename: &str) -> IntCodeMachine {
        let mut buffer = String::new();
        BufReader::new(File::open(filename).expect("File is fucked!"))
            .lines()
            .map(|l| l.expect("Line is fucked!"))
            .fold((), |_, s| buffer.push_str(&s));
        IntCodeMachine::from_string(buffer)
    }
    pub fn _programiter<'a>(&'a self) -> Box<dyn Iterator<Item = &i64> + 'a> {
        Box::new(self.program.iter())
    }
    pub fn getmem(&self, i: usize) -> i64 {
        if let Some(&x) = self.program.get(i) {
            x
        } else {
            0
        }
    }
    pub fn setmem(&mut self, i: usize, v: i64) {
        if let Some(x) = self.program.get_mut(i) {
            *x = v;
        } else {
            self.program.resize(i + 1, 0);
            self.program[i] = v;
        }
    }
    fn getmode(&self, i: usize) -> usize {
        ((self.getmem(self.pc) / pow(10, i + 1)) % 10) as usize
    }
    fn getval(&self, i: usize) -> i64 {
        let val = self.getmem(self.pc + i);
        match self.getmode(i) {
            0 => self.getmem(val as usize),
            1 => val,
            2 => self.getmem((val + self.relbase) as usize),
            mode => panic!("Invalid memory mode for reading: {}", mode),
        }
    }
    fn setval(&mut self, i: usize, v: i64) {
        let val = self.getmem(self.pc + i);
        match self.getmode(i) {
            0 => self.setmem(val as usize, v),
            2 => self.setmem((val + self.relbase) as usize, v),
            mode => panic!("Invalid memory mode for writing: {}", mode),
        }
    }
    pub fn sendinput<T: Iterator<Item = i64>>(&mut self, mut input: T) {
        while let Some(x) = input.next() {
            self.inputqueue.push_back(x);
        }
    }
    pub fn outputiter<'a>(&'a mut self) -> Box<dyn Iterator<Item = i64> + 'a> {
        Box::new(
            (0..)
                .map(move |_| self.outputqueue.pop_front())
                .take_while(|x| x.is_some())
                .map(|x| x.unwrap()),
        )
    }
    pub fn run(&mut self) -> bool {
        //
        let mut leave = false;
        //
        while !leave {
            let instruction = self.getmem(self.pc);
            //
            match instruction % 100 {
                1 => {
                    self.setval(3, self.getval(1) + self.getval(2));
                    self.pc += 4;
                }
                2 => {
                    self.setval(3, self.getval(1) * self.getval(2));
                    self.pc += 4;
                }
                3 => {
                    if let Some(x) = self.inputqueue.pop_front() {
                        self.setval(1, x);
                        self.pc += 2;
                    } else {
                        leave = true;
                    }
                }
                4 => {
                    self.outputqueue.push_back(self.getval(1));
                    self.pc += 2;
                }
                5 => {
                    if self.getval(1) != 0 {
                        self.pc = self.getval(2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                6 => {
                    if self.getval(1) == 0 {
                        self.pc = self.getval(2) as usize;
                    } else {
                        self.pc += 3;
                    }
                }
                7 => {
                    self.setval(3, (self.getval(1) < self.getval(2)) as i64);
                }
                8 => {
                    self.setval(3, (self.getval(1) == self.getval(2)) as i64);
                }
                9 => {
                    self.relbase += self.getval(1);
                    self.pc += 2;
                }
                99 => break,
                _ => panic!("Trying to run invalid intcode instruction!"),
            }
        }
        !leave
    }
}
