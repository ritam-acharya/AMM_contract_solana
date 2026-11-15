pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("JCxTbX93MmTzffYDKdF9AGJ4BDsKoXm1aER16jDRmyR7");

#[program]
pub mod turbin_week3_assignment {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>,  fee: u16, seed: u64, authority: Option<Pubkey>) -> Result<()> {
        initialize::initialize(ctx,  fee, seed, authority)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64, max_x: u64, max_y: u64) -> Result<()> {
        deposit::deposit(ctx, amount, max_x, max_y)
    }
}
