use crate::parser::{Instructions, ParseError};

pub struct CPU {
    pub memory: [u8;1024],
    pub ptr: usize,
    pub tmp: u8,
    pub program: Vec<Instructions>,
}

impl CPU {
    pub fn new () -> CPU{
        CPU {
         memory : [0;1024],
         ptr : 0,
         tmp: 0,
         program: Vec::new(),
        }
    }
    pub fn exec(&mut self) {
        println!("{:?}", self.program);
    }
    pub fn load_program(&mut self, program: &str) -> Result<(), ParseError> {
        self.program = program.chars().enumerate().map(|(i, c)| Instructions::from_bytes(c, i)).collect::<Result<Vec<Instructions>, ParseError>>()?;

        Ok(())
    }
}
