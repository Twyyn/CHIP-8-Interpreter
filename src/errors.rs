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
}
#[derive(Error, Debug)]
pub enum OpCodeError {
    #[error("Unknown Opcode : {0}")]
    UnknownOpcode(u16),
}
