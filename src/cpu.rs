use std::io::Read;

use crate::parser::{Instructions, ParseError};

pub struct CPU {
    pub memory: [u8;1024],
    pub dp: usize, // Data Pointer
    pub tmp: u8,
    pub program: Vec<Instructions>,
    pub ip: usize, // Instruction Pointer
}

impl CPU {
    pub fn new () -> CPU{
        CPU {
         memory : [0;1024],
         dp : 0,
         ip : 0,
         tmp: 0,
         program: Vec::new(),
        }
    }
    pub fn exec(&mut self) {
        loop {
            let instruction = self.program[self.ip];
            match instruction {
                Instructions::IncPtr => self.dp += 1,
                Instructions::DecPtr => {
                    if self.dp > 0 {
                        self.dp -= 1;
                    }
                },
                Instructions::IncVal => self.memory[self.dp] += 1,
                Instructions::DecVal => {
                    if self.memory[self.dp] > 0 {
                        self.memory[self.dp] -= 1;
                    }
                },
                Instructions::PrintVal => {
                    // convert to char and print
                    print!("{}", self.memory[self.dp] as char);
                },
                Instructions::ReadVal => {
                    // Read 1 character and store it to pointer
                    std::io::stdin().read(&mut [self.tmp]).unwrap();
                    self.memory[self.dp] = self.tmp;
                },
                Instructions::LoopStart => {
                   if self.memory[self.dp] == 0 {
                    let mut ctr = 0;
                    loop {
                        self.ip += 1;
                        match self.program[self.ip] {
                            Instructions::LoopStart => ctr += 1,
                            Instructions::LoopEnd => {
                                if ctr == 0 {
                                    break;
                                } else {
                                    ctr -= 1;
                                }
                            },
                            _ => (),
                        }
                        
                    }
                   }
                },
                Instructions::LoopEnd => {
                  if self.memory[self.dp] != 0 {
                    let mut ctr = 0;
                    loop {
                        self.ip -= 1;
                        match self.program[self.ip] {
                            Instructions::LoopEnd => ctr += 1,
                            Instructions::LoopStart => {
                                if ctr == 0 {
                                    break;
                                } else {
                                    ctr -= 1;
                                }
                            },
                            _ => (),
                        }
                        
                    }
                  }
                },

            }
            
            self.ip += 1;

            if self.program.len() == self.ip {
                break
            }
        }
        // Execute the program
    }
    pub fn load_program(&mut self, program: &str) -> Result<(), ParseError> {
        self.program = program.chars().enumerate().map(|(i, c)| Instructions::from_bytes(c, i)).collect::<Result<Vec<Instructions>, ParseError>>()?;

        Ok(())
    }
}
