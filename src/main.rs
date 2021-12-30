// https://en.wikipedia.org/wiki/Brainfuck

use std::str::FromStr;
use crate::brainfuck_interpreter::BrainfuckInterpreter;

mod brainfuck_interpreter;
mod command;
mod memory;

fn main() -> Result<(), ()> {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    let mut brainfuck_interpreter = BrainfuckInterpreter::from_str(program)?;
    brainfuck_interpreter.run()?;

    Ok(())
}
