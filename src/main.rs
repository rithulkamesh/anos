mod cpu;
mod parser;
use cpu::CPU;


fn main() {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";

    let mut cpu = CPU::new();
    match cpu.load_program(program) {
        Ok(_) => cpu.exec(),
        Err(e) => println!("Failed to parse program, character at position {} is a '{}' which is not a valid Brainfuck command.", e.1 + 1, e.0),
    }
}
