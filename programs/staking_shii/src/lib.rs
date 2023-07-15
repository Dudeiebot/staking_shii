use anchor_lang::prelude::*;

declare_id!("tUhgH4UM8cjTTRJo8TLAaPR5SikDAoMdfCms5ZsnSSK");

#[program]
pub mod staking_shii {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
