use solana_program::program_error::ProgramError;

#[derive(Debug, Clone, Copy)]
pub enum MemeCoinError {
    // Custom errors related to your meme coin operations
    InvalidInstruction,
}

impl From<MemeCoinError> for ProgramError {
    fn from(e: MemeCoinError) -> Self {
        ProgramError::Custom(e as u32)
    }
}