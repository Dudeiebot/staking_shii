use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, TokenAccount, Token, Transfer};
use anchor_spl::associated_token::AssociatedToken;

declare_id!("tUhgH4UM8cjTTRJo8TLAaPR5SikDAoMdfCms5ZsnSSK");

#[program]
pub mod staking_shii {
    use anchor_spl::token::transfer;

    use super::*;

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        //inintialize value of user info

        if !ctx.accounts.user_info.is_intialized {
            ctx.accounts.user_info.is_intialized = true;
            ctx.accounts.user_info.points = 0;
            ctx.accounts.user_info.stake_count = 0;
            ctx.accounts.user_info.owner = ctx.accounts.owner.key();
        }

        //transfer nft from owner to escrow
        let cpi_program = ctx.accounts.token_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.user_token_account.to_account_info(),
            to: ctx.accounts.stake_token_account.to_account_info(),
            authority: ctx.accounts.owner.to_account_info(),
        };
        let cpi_context = CpiContext::new(cpi_program, cpi_accounts);
        transfer(cpi_context, 1)?;

        //set the appropriate data fields for the data account
        ctx.accounts.stake_info.owner = ctx.accounts.owner.key();
        ctx.accounts.stake_info.mint = ctx.accounts.mint.key(); 
        ctx.accounts.stake_info.start_timestamp = Clock::get().unwrap().unix_timestamp as u64;
        ctx.accounts.stake_info.last_redeem_timestamp = Clock::get().unwrap().unix_timestamp as u64;
        ctx.accounts.stake_info.is_staked = true;

        ctx.accounts.user_info.stake_count = ctx.accounts.user_info.stake_count.checked_add(1).unwrap();

        Ok(())
    }
    
    pub fn redeem(ctx: Context<Redeem>) -> Result<()> {
        Ok(())
    }

    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Stake<'info>{
    // Data Accounts
    #[account(
        init_if_needed,
        seeds = [b"USER", owner.key().as_ref()],
        bump,
        payer = owner,
        space = std::mem::size_of::<UserInfo>() + 8
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        init_if_needed,
        seeds = [b"STAKE", owner.key().as_ref(), mint.key().as_ref()],
        bump,
        payer = owner,
        space = std::mem::size_of::<StakeInfo>() + 8
    )]
    pub stake_info: Account<'info, StakeInfo>,

    //Token account
    pub mint: Account<'info, Mint>,
    
    #[account(
        init_if_needed,
        payer = owner,
        associated_token::mint = mint,
        associated_token::authority = stake_info,
        
    )]
    pub stake_token_account: Account<'info, TokenAccount>,

    #[account(
        mut,
        constraint = user_token_account.owner == owner.key(), //these is used to check all 
        constraint = user_token_account.mint == mint.key(),
        constraint = user_token_account.amount == 1
    )]
    pub user_token_account: Account<'info, TokenAccount>,
   
    //signer
    #[account(mut)] 
    pub owner: Signer<'info>,

    //program accounts
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub rent: Sysvar<'info, Rent>,
}


#[derive(Accounts)]
pub struct Redeem {}


#[derive(Accounts)]
pub struct Unstake {}

#[account]
pub struct StakeInfo {
    owner: Pubkey,
    mint: Pubkey,
    start_timestamp: u64, //unix timestamp
    last_redeem_timestamp: u64,
    is_staked: bool,
}

#[account]
pub struct UserInfo { //it all starts off at 0
    owner: Pubkey,
    points: u64,
    stake_count: u16, //so be careful not to create a lot of data that you are not using because you pay per byte and you dont want have a lot of nft upto that, so therefore make the balance between keeping it low and also future proof
    is_intialized: bool,
}