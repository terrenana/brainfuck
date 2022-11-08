use std::io::Read;

pub enum BrainfuckError {
    LexError(String),
    ParseError(String),
    ExecuteError(String)
}

#[derive(Clone)]
enum OpCode {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    LoopBegin,
    LoopEnd,
}

enum Instruction {
    IncrementPointer,
    DecrementPointer,
    Increment,
    Decrement,
    Write,
    Read,
    Loop(Vec<Instruction>),
}

fn lex(stream: String) -> Result<Vec<OpCode>, BrainfuckError> {
    let mut opcodes: Vec<OpCode> = Vec::new();

    for char in stream.chars() {
        let opcode = match char {
            '>' => Some(OpCode::IncrementPointer),
            '<' => Some(OpCode::DecrementPointer),
            '+' => Some(OpCode::Increment),
            '-' => Some(OpCode::Decrement),
            '.' => Some(OpCode::Write),
            ',' => Some(OpCode::Read),
            '[' => Some(OpCode::LoopBegin),
            ']' => Some(OpCode::LoopEnd),
            _ => None
        };
        
        match opcode {
            Some(opcode) => opcodes.push(opcode),
            None => ()
        }
    }
    Ok(opcodes)
}

fn parse(opcodes: Vec<OpCode>) -> Result<Vec<Instruction>, BrainfuckError> {
    let mut instructions: Vec<Instruction> = Vec::new();
    let mut loop_stack = 0;
    let mut loop_start = 0;

    for (ix, opcode) in opcodes.iter().enumerate() {
        if loop_stack == 0 {
            let instruction = match opcode {
                OpCode::IncrementPointer => Some(Instruction::IncrementPointer),
                OpCode::DecrementPointer => Some(Instruction::DecrementPointer),
                OpCode::Increment => Some(Instruction::Increment),
                OpCode::Decrement => Some(Instruction::Decrement),
                OpCode::Write => Some(Instruction::Write),
                OpCode::Read => Some(Instruction::Read), 

                OpCode::LoopBegin => {
                    loop_start = ix;
                    loop_stack += 1;
                    None
                }

                OpCode::LoopEnd => {
                    return Err(BrainfuckError::ParseError(format!("Loop ending at #{} has no beginning!", ix)));
                }
            };
            match instruction {
                Some(instruction) => instructions.push(instruction),
                None => ()
            };
        } else {
            match opcode {
                OpCode::LoopBegin => {
                    loop_stack += 1;
                },
                OpCode::LoopEnd => {
                    loop_stack -= 1;

                    if loop_stack == 0 {
                        instructions.push(Instruction::Loop(parse(opcodes[loop_start+1..ix].to_vec())?));
                    }
                },
                _ => ()
            }
        }
    }
    if loop_stack != 0 {
        return Err(BrainfuckError::ParseError(format!("Loop starting at #{} has no matching ending!", loop_start)));
    }

    Ok(instructions)
}

fn execute(instructions: &Vec<Instruction>, tape: &mut Vec<u8>, pointer: &mut usize)-> Result<(), BrainfuckError> {
    for instruction in instructions {
        match instruction {
            Instruction::IncrementPointer => *pointer += 1,
            Instruction::DecrementPointer => *pointer -= 1,
            Instruction::Increment => tape[*pointer] += 1,
            Instruction::Decrement => tape[*pointer] -= 1,
            Instruction::Write => print!("{}", tape[*pointer] as char),
            Instruction::Read => {
                let mut input: [u8; 1] = [0; 1];
                match std::io::stdin().read_exact(&mut input) {
                    Ok(_) => (),
                    Err(a) => {
                        return Err(BrainfuckError::ExecuteError(a.to_string()))
                    }
                };
                tape[*pointer] = input[0];
            },
            Instruction::Loop(loop_instructions) => {
                while tape[*pointer] != 0 {
                    execute(&loop_instructions, tape, pointer)?
                }
            }
        }
    }
    Ok(())
}

pub fn run(stream: String, tape: &mut Vec<u8>, pointer: &mut usize) -> Result<(), BrainfuckError> {
    Ok(execute(&parse(lex(stream)?)?, tape, pointer)?)
}
