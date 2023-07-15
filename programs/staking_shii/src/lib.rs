use anchor_lang::prelude::*;

declare_id!("tUhgH4UM8cjTTRJo8TLAaPR5SikDAoMdfCms5ZsnSSK");

#[program]
pub mod staking_shii {
    use super::*;

    pub fn stake(ctx: Context<Stake>) -> Result<()> {
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
pub struct Stake {}


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
pub struct UserInfo {
    owner: Pubkey,
    points: u64,
    stake_count: u16, //so be careful not to create a lot of data that you are not using because you pay per byte and you dont want have a lot of nft upto that, so therefore make the balance between keeping it low and also future proof

}