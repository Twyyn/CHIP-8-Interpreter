use thiserror::Error;

#[derive(Error, Debug)]
pub enum MemoryError {
    #[error("Memory Overflow")]
    MemoryOverflow,

    #[error("Failed to read RAM address: {0}")]
    MemoryReadError(u16),

    #[error("Failed writing value ({value}) to RAM address ({addr})")]
    MemoryWriteError { addr: u16, value: u8 },

    #[error("Stack overflow")]
    StackOverflow,

    #[error("Stack underflow")]
    StackUnderflow,

    #[error("Error loading ROM")]
    ROMLoadError,
}

#[derive(Error, Debug)]
pub enum OpcodeError {
    #[error("Unknown Mnemonic: {0:#06x}")]
    UnknownMnemonic(u16),

    #[error("Bad X,Y tail: {0:#06x}")]
    BadXYTail(u16),
}
#[derive(Error, Debug)]
pub enum KeyboardError {
    #[error("Unknown key")]
    UnknownKey,
}

#[derive(Error, Debug)]
pub enum EmuError {
    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Memory(#[from] MemoryError),

    #[error(transparent)]
    Opcode(#[from] OpcodeError),

    #[error(transparent)]
    Keyboard(#[from] KeyboardError),
}
