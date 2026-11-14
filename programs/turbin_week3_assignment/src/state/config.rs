use anchor_lang::prelude::*;


#[account]
#[derive(InitSpace)]
pub struct Config {
    seed: u64,
    fee: u16,
    authority: Option<Pubkey>,
    is_locked: bool,
    config_bump: u8,
    lp_bump: u8,
    mint_x: Pubkey,
    mint_y: Pubkey,
}