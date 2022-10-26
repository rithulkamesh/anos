#[derive(Debug, Copy, Clone)]
pub enum Instructions {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    PrintVal,
    ReadVal,
    LoopStart,
    LoopEnd,
}

#[derive(Debug)]
pub struct ParseError(pub char, pub usize);

impl Instructions {
    pub fn from_bytes(byte: char, index: usize) -> Result<Self, ParseError> {
            match byte {
                '>' => Ok(Instructions::IncPtr),
                '<' => Ok(Instructions::DecPtr),
                '+' => Ok(Instructions::IncVal),
                '-' => Ok(Instructions::DecVal),
                '.' => Ok(Instructions::PrintVal),
                ',' => Ok(Instructions::ReadVal),
                '[' => Ok(Instructions::LoopStart),
                ']' => Ok(Instructions::LoopEnd),
                token => Err(ParseError(token, index)),
            }
    }
}
