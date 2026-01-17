use anchor_lang::prelude::*;

// This is your program's public key and it will update
// automatically when you build the project.
declare_id!("G686y4s1Ft8HmndMedTEoRZyvRRXipSubiBQzpqPRcXW");

#[program]
mod CRUD {
    use super::*;
    pub fn Make_Journal(ctx: Context<InitJournal>, title: String, msg: String) -> Result<()> {
        let journal = &mut ctx.accounts.journal;
        journal.title = title;
        journal.message = msg;

        msg!("Made journal with tile: {title}, message: {msg}");
        Ok(())
    }

    pub fn modify(ctx: Context<Modify>, msg: String) -> Result<()> {
        let journal = &mut ctx.accounts.journal;
        journal.message = msg;
        msg!("Modified journal's message: {msg}");
        Ok(())
    }

    pub fn delete(ctx: Context<Delete>) -> Result<()> {
        msg!("Journal Deleted");
        Ok(())
    }

}

#[account]
#[derive(InitSpace)]
pub struct Journal {
    pub owner: Pubkey,
    #[max_len(50)]
    pub title: String,
    #[max_len(250)]
    pub message: String,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct InitJournal<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,

    #[account(
        init,
        space = 8 + Journal::Init_Space,
        seeds = [owner.key().as_ref(), title.as_bytes()],
        bump,
        payer = owner
    )]
    pub journal: Account<'info, Journal>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Modify<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,

    #[account(
        mut,
        realloc = 8 + Journal::Init_Space,
        realloc::payer = owner, 
        realloc::zero = true, 
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump,
    )]
    pub journal: Account<'info, Journal>,
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct Delete<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account( 
        mut, 
        seeds = [title.as_bytes(), owner.key().as_ref()], 
        bump, 
        close = owner,
    )]
    pub journal_entry: Account<'info, Journal>,
}
