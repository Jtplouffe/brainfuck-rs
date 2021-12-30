pub(crate) enum Command {
    IncrementDP,
    DecrementDP,
    IncrementMem,
    DecrementMem,
    Output,
    Input,
    JmpNext,
    JmpPrev,
}

impl Command {
    pub(crate) fn from_char(c: char) -> Result<Self, ()> {
        match c {
            '>' => Ok(Self::IncrementDP),
            '<' => Ok(Self::DecrementDP),
            '+' => Ok(Self::IncrementMem),
            '-' => Ok(Self::DecrementMem),
            '.' => Ok(Self::Output),
            ',' => Ok(Self::Input),
            '[' => Ok(Self::JmpNext),
            ']' => Ok(Self::JmpPrev),
            _ => Err(()),
        }
    }
}
