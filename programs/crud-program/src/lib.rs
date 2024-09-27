use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod crud_program {
    use super::*;

    pub fn create(ctx: Context<Create>, data: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.data = data;
        Ok(())
    }

    pub fn update(ctx: Context<Update>, data: u64) -> Result<()> {
        let user_account = &mut ctx.accounts.user_account;
        user_account.data = data;
        Ok(())
    }

    // Read operation is handled by the client
    
    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        // The account will be closed and lamports returned to the user
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 8)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Update<'info> {
    #[account(mut)]
    pub user_account: Account<'info, UserAccount>,
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct Delete<'info> {
    #[account(mut, close = user)]
    pub user_account: Account<'info, UserAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[account]
pub struct UserAccount {
    pub data: u64,
}