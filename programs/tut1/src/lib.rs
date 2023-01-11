use anchor_lang::prelude::*;

declare_id!("9tQ16Efw5TgKEYnx5qPxULB7kBBsA5jzRKQBNmP3h2bi");

#[program]
pub mod tut1 {
    use anchor_lang::solana_program::program::invoke;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Initialized ...");

        Ok(())
    }

    pub fn sol_transfer(ctx: Context<ASolTransfer>, amount: u64) -> Result<()> {
        let sender = ctx.accounts.sender.to_account_info();
        let receiver = ctx.accounts.receiver.to_account_info();
        let system_program = ctx.accounts.system_program.to_account_info();

        // let ix = system_instruction::transfer(&sender.key(), &receiver.key(), amount);
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &sender.key(),
            &receiver.key(),
            amount,
        );
        
        invoke(
            &ix,
            &[sender, receiver, system_program], // ).unwrap();
        )?;

        msg!("Sol tranfered : {}", amount as f64 / 1000_000_000.0);

        Ok(())
    }

    pub fn init_account(ctx: Context<InitAccount>) -> Result<()>{
        msg!("Account is initialized ....");
        Ok(())
    }

    pub fn add(ctx: Context<Add>, a: i32, b: i32) -> Result<()> {
        let account = &mut ctx.accounts.account;
        let tmp = a + b;
        account.res = tmp;

        msg!("Addition ....");
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct ASolTransfer<'info> {
    #[account(mut)]
    pub sender: Signer<'info>,

    ///CHECK:
    #[account(mut)]
    pub receiver: AccountInfo<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct InitAccount<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        init,
        signer,
        payer = user,
        space = 8 + Answer::MAX_SIZE,
    )]
    pub account: Account<'info, Answer>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Add<'info> {
    #[account(mut, signer)]
    pub account: Account<'info, Answer>,
}

#[account]
pub struct Answer {
    pub res: i32,
}
impl Answer {
    pub const MAX_SIZE: usize = std::mem::size_of::<Answer>();
}
