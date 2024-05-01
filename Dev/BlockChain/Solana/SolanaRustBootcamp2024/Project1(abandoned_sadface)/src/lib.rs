use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};
pub mod error;
pub mod instructions;
pub mod processor;
pub mod state;


pub fn process_initialize_game(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    entry_fee: u64,
) -> ProgramResult {
    let account_info_iter = &mut accounts.iter();
    let game_account = next_account_info(account_info_iter)?;
    let payer_account = next_account_info(account_info_iter)?;
    let system_program = next_account_info(account_info_iter)?;

    if !payer_account.is_signer {
        msg!("Payer account must be a signer.");
        return Err(ProgramError::MissingRequiredSignature);
    }

    // Validation for Game Over to be true (TODO)

    Ok(())
}
