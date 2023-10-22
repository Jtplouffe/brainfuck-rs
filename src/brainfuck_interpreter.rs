use std::io::Read;
use std::str::FromStr;

use crate::command::Command;
use crate::memory::Memory;

pub(crate) struct BrainfuckInterpreter {
    ip: usize,
    commands: Vec<Command>,
    memory: Memory,
}

impl BrainfuckInterpreter {
    pub(crate) fn run(&mut self) -> Result<(), ()> {
        while let Some(command) = self.commands.get(self.ip) {
            match command {
                Command::IncrementDP => self.memory.increment_dp(),
                Command::DecrementDP => self.memory.decrement_dp(),
                Command::IncrementMem => self.memory.increment(),
                Command::DecrementMem => self.memory.decrement(),
                Command::Input => {
                    let char = std::io::stdin().bytes().next().ok_or(())?.or(Err(()))?;
                    self.memory.set_value(char as i32);
                }
                Command::Output => {
                    let a = self.memory.value();
                    print!("{}", a as u8 as char);
                }
                Command::JmpNext => {
                    if self.memory.value() == 0 {
                        let next_ip = self.next_matching_bracket().ok_or(())?;
                        self.ip = next_ip;
                    }
                }
                Command::JmpPrev => {
                    if self.memory.value() != 0 {
                        let next_ip = self.previous_matching_bracket().ok_or(())?;
                        self.ip = next_ip;
                    }
                }
            }

            self.ip += 1;
        }

        Ok(())
    }

    fn next_matching_bracket(&self) -> Option<usize> {
        let mut depth = 0;

        for i in self.ip..self.commands.len() {
            match self.commands[i] {
                Command::JmpNext => depth += 1,
                Command::JmpPrev => depth -= 1,
                _ => {}
            }

            if depth == 0 {
                return Some(i);
            }
        }

        None
    }

    fn previous_matching_bracket(&self) -> Option<usize> {
        let mut depth = 0;

        for i in (0..=self.ip).rev() {
            match self.commands[i] {
                Command::JmpNext => depth -= 1,
                Command::JmpPrev => depth += 1,
                _ => {}
            }

            if depth == 0 {
                return Some(i);
            }
        }

        None
    }
}

impl FromStr for BrainfuckInterpreter {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.trim().replace(['\n', '\r', ' '], "");
        let commands = s
            .chars()
            .map(Command::from_char)
            .collect::<Result<Vec<_>, ()>>()?;

        Ok(Self {
            ip: 0,
            memory: Memory::new(),
            commands,
        })
    }
}
