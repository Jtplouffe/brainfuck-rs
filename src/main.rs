use crate::brainfuck_interpreter::BrainfuckInterpreter;
use std::str::FromStr;

mod brainfuck_interpreter;
mod command;
mod memory;

fn main() -> Result<(), ()> {
    let program = "++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.";
    BrainfuckInterpreter::from_str(program)?.run()?;

    Ok(())
}
