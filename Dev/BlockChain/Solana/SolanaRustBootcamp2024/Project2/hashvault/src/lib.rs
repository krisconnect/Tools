use anchor_lang::prelude::*;
//use crate::LineColumn;
//use crate::fallback::LineColumn;



declare_id!("BtSZDMEWUNjHi38dqbxf1qxBahf4vtUn8HurZxSE85px");

#[program]
pub mod hashvault {
    use super::*;

    pub fn create_ledger(
        ctx: Context<CreateLedger>,
        hashvalue: String,
        _seedstring: String
    ) -> Result<()> {

        let ledger_account = &mut ctx.accounts.ledger_account;
        ledger_account.hashvalue = hashvalue;
        
        Ok(())
    }

}

#[derive(Accounts)]
#[instruction(hashvalue: String, seedstring: String)]
pub struct CreateLedger<'info> {
    #[account(
        init,
        payer = wallet,
        space = 146, //82 for the original init plus 64 bytes for the sha512 hash
        seeds = [
            wallet.key().as_ref(),
            seedstring.as_ref(), //user input
            hashvalue.as_ref(),
        ],
        bump
    )]
    pub ledger_account: Account<'info, Ledger>,
    #[account(mut)]
    pub wallet: Signer<'info>,
    pub system_program: Program<'info, System>,
}
#[account]
pub struct Ledger {
    pub hashvalue: String,
    pub seedstring: String,
}