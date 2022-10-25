struct CPU {
    memory: [u8;1024],
    ip: usize,
    tmp: u8
}

pub type Program = [u8;1024];

impl CPU {
    pub fn new () -> CPU{
        CPU {
         memory : [0;1024],
         ip : 0,
         tmp: 0
        }
    }
    pub fn exec(&mut self) {
        loop {
            let data = self.memory[self.ip];
            self.ip += 1;

            match data {
                1 => {
                    let lhs = self.memory[1];
                    let rhs = self.memory[2];
                    self.memory[3] = lhs+rhs;
                },
                2 => {
                    let lhs = self.memory[1];
                    let rhs = self.memory[2];
                    self.memory[3] = lhs-rhs;

                }
                3 => {
                   self.tmp= self.memory[self.ip];
                    
                },
                4 => {
                    let dest = self.memory[self.ip] as usize;
                    self.memory[dest] = self.tmp
                },
                5 => {
                    println!("{}", self.memory[3])
                },
                254 => {
                    loop{}
                },
                255 => {
                    self.ip = 0;
                },
                _ => {} 
            }
        }
    }
    pub fn load_program(&mut self, program: Program) {
        self.memory = program;
    }
}


enum Instructions {
    Add,
    Sub,
    Load,
    Store,
    Print,
    HALT,
    EOF
}

impl Instructions {
    pub fn from_bytes(byte: u8) -> Option<Self> {
            match byte {
                1 => Some(Self::Add),
                2 => Some(Self::Sub),
                3 => Some(Self::Load),
                4 => Some(Self::Store),
                5 => Some(Self::Print),
                255 => Some(Self::EOF),
                _ => None,
            }
    }
}


fn main() {
    let mut program = [0; 1024];    
    program[0] = 3; // Load
    program[1] = 9; // the value 9
    program[2] = 4; // store
    program[3] = 1; // into 1 (lhs)
    program[4] = 3; // Load
    program[5] = 3; // the value 4
    program[6] = 4; // store
    program[7] = 2; // into 2 (rhs)
    program[8] = 1; // Add
    program[9] = 5; // Print
    program[10] = 254; // End of program
    let mut cpu = CPU::new();
    cpu.load_program(program);
    cpu.exec();
}
