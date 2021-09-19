use thiserror::Error;
use solana_program::program_error::ProgramError;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError { //enum EscrowError
    /// Invalid instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}

impl From<EscrowError> for ProgramError { //implement trait From<EscrowError> to ProgramError
    fn from(e: EscrowError) -> Self { // from method that returns the error message
        ProgramError::Custom(e as u32) // convert escrow error to program error
    }
}