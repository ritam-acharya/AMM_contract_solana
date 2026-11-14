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

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }
}
