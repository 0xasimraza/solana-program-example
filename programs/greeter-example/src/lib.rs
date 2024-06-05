use anchor_lang::prelude::*;

declare_id!("E8XjDqjjAzxh8J8eaceFwFDRVBAgHjUtheMdJd44of2d");
#[program]
pub mod greeter_example {
    use anchor_lang::solana_program::{program::invoke, system_instruction};

    use super::*;

    pub fn initialize(ctx: Context<Initialize>, greeting: String) -> Result<()> {
        let account = &mut ctx.accounts.greeting_account;
        account.greeting = greeting;
        Ok(())
    }

    pub fn set_greeting(ctx: Context<SetGreeting>, greeting: String) -> Result<()> {
        let account = &mut ctx.accounts.greeting_account;
        account.greeting = greeting;
        Ok(())
    }

    pub fn get_greeting(ctx: Context<GetGreeting>) -> Result<String> {
        let account = &ctx.accounts.greeting_account;
        Ok(account.greeting.clone())
    }

    pub fn deposit_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        let ix = system_instruction::transfer(
            &ctx.accounts.user.key(),
            &ctx.accounts.greeting_account.key(),
            amount,
        );

        invoke(
            &ix,
            &[
                ctx.accounts.user.to_account_info(),
                ctx.accounts.greeting_account.to_account_info(),
                ctx.accounts.system_program.to_account_info(),
            ],
        )?;

        let account = &mut ctx.accounts.greeting_account;
        account.balance += amount;

        Ok(())
    }

    pub fn withdraw_sol(ctx: Context<DepositSol>, amount: u64) -> Result<()> {
        let account = &ctx.accounts.greeting_account;
        if account.balance < amount {
            return Err(ErrorCode::InsufficientFunds.into());
        }

        **ctx
            .accounts
            .greeting_account
            .to_account_info()
            .try_borrow_mut_lamports()? -= amount;

        **ctx
            .accounts
            .user
            .to_account_info()
            .try_borrow_mut_lamports()? += amount;

        let account = &mut ctx.accounts.greeting_account;

        account.balance -= amount;

        Ok(())
    }
}

#[account]
pub struct GreetingAccount {
    pub greeting: String,
    pub balance: u64,
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = user, space = 8 + 256)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetGreeting<'info> {
    #[account(mut)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[derive(Accounts)]
pub struct GetGreeting<'info> {
    pub greeting_account: Account<'info, GreetingAccount>,
}

#[derive(Accounts)]
pub struct DepositSol<'info> {
    #[account(mut)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct WithdrawSol<'info> {
    #[account(mut)]
    pub greeting_account: Account<'info, GreetingAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
}

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient funds.")]
    InsufficientFunds,
}
