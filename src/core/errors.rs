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
    #[error("Stack Underflow")]
    StackUnderflow,
    #[error("Error loading ROM ")]
    LoadError,
}
#[derive(Error, Debug)]
pub enum OpcodeError {
    #[error("Unknown Mnemonic: {0}")]
    UnknownMnemonic(u16),
    #[error("Bad X,Y : {0}")]
    BadXYTail(u16),
}
#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("Unknown Key")]
    UnknownKey,
}
