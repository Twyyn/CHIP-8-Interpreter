use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Memory Overflow")]
    MemoryOverflow,
    #[error("Failed to read RAM Address: {0}")]
    MemoryReadError(u16),
    #[error("Failed to writing value ({value}) to RAM address ({addr})")]
    MemoryWriteError { addr: u16, value: u8 },
}
