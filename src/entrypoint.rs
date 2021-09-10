use solana_program::{
    account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

entrypoint!(process_instruction); // uses macro entrypoint! to declare something as the entrypoint of the program, only way to call the program
fn process_instruction( 
    program_id: &Pubkey, //name of the currently executing program
    accounts: &[AccountInfo], 
    instruction_data: &[u8], // data passed to the program by the caller
) -> ProgramResult {
    msg!(
        "process_instruction: {}: {} accounts, data={:?}",
        program_id,
        accounts.len(),
        instruction_data
    );
    Ok(())
}
/* To be stateful means that some amount of storage on the chain is used to store values. 
This storage can be either global or local. 
Local storage refers to storing values in an accounts balance record if that account participates in the contract.
 Global storage is data that is specifically stored on the blockchain for the contract globally. 
 Like stateless smart contracts, stateful contracts are written in TEAL and can be deployed to the
  blockchain using either the goal command-line tool or the SDKs. 
 Stateless smart contracts’ primary purpose is to approve or reject spending transactions.
  Stateful contracts do not approve spending transactions but provide logic that allows 
  the state (globally or locally) of the contract to be manipulated. */