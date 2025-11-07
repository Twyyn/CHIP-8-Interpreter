use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Memory Overflow")]
    MemoryOverflow,
    #[error("Failed to read RAM address: {0}")]
    MemoryReadError(u16),
    #[error("Failed to writing value ({value}) to RAM address ({addr})")]
    MemoryWriteError { addr: u16, value: u8 },
    #[error("Stack Overflow")]
    StackOverflow,
    #[error("Error parsing program: {0} ")]
    ParseError(String)
}
#[derive(Error, Debug)]
pub enum OpcodeError {
    #[error("Unknown Opcode : {0}")]
    UnknownOpcode(u16),
    #[error("Bad X,Y : {0}")]
    BadXYTail(u16),
}
#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("Unknown KEY : {0}")]
    UnknownKEY(usize),
}
