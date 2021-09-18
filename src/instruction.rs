
// inside instruction.rs

use std::convert::TryInto;
use solana_program::program_error::ProgramError;
use crate::error::Escrow::InvalidInstruction;
pub enum EscrowInstruction {

    /// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
    ///
    ///
    /// Accounts expected:
    ///
    /// 0. `[signer]` The account of the person initializing the escrow. [initializer]
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade go through [] read only
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitEscrow {
        /// The amount party A expects to receive of token Y
        amount: u64
    }
}
impl EscrowInstruction {
    /// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> { //input is a borrowed read only u8, returns a OK or ProgrammError
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?; // create var (tag, rest) then split the first element, if it is Ok then proceed. else InvalidInstruction

        Ok(match tag { //match tag to 0 or the signer
            0 => Self::InitEscrow { // if match, release the escrow account
                amount: Self::unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()), //if not a match, error
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> { //amount in the escrow, returns(Ok) the amount
        let amount = input
            .get(..8)
            .and_then(|slice| slice.try_into().ok())
            .map(u64::from_le_bytes)
            .ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}