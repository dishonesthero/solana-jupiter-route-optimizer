use anchor_lang::prelude::*;

declare_id!("Optm111111111111111111111111111111111111111");

#[program]
pub mod route_optimizer {
    use super::*;

    pub fn initialize_optimizer(
        ctx: Context<InitializeOptimizer>,
        protocol_fee_bps: u16,
    ) -> Result<()> {
        require!(protocol_fee_bps <= 100, RouteError::FeeExceedsMaximum);

        let config = &mut ctx.accounts.config;
        config.authority = ctx.accounts.authority.key();
        config.protocol_fee_bps = protocol_fee_bps;
        config.total_swaps = 0;
        config.bump = ctx.bumps.config;

        emit!(OptimizerInitializedEvent {
            authority: config.authority,
            protocol_fee_bps,
        });

        Ok(())
    }

    pub fn verify_and_settle_swap(
        ctx: Context<SettleSwap>,
        min_output_amount: u64,
        actual_output_amount: u64,
    ) -> Result<()> {
        require!(
            actual_output_amount >= min_output_amount,
            RouteError::SlippageExceeded
        );

        let config = &mut ctx.accounts.config;
        config.total_swaps = config.total_swaps.saturating_add(1);

        emit!(SwapSettledEvent {
            user: ctx.accounts.user.key(),
            output_amount: actual_output_amount,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeOptimizer<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + OptimizerConfig::LEN,
        seeds = [b"optimizer-config", authority.key().as_ref()],
        bump
    )]
    pub config: Account<'info, OptimizerConfig>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SettleSwap<'info> {
    #[account(mut)]
    pub config: Account<'info, OptimizerConfig>,
    pub user: Signer<'info>,
}

#[account]
pub struct OptimizerConfig {
    pub authority: Pubkey,
    pub protocol_fee_bps: u16,
    pub total_swaps: u64,
    pub bump: u8,
}

impl OptimizerConfig {
    pub const LEN: usize = 32 + 2 + 8 + 1;
}

#[event]
pub struct OptimizerInitializedEvent {
    pub authority: Pubkey,
    pub protocol_fee_bps: u16,
}

#[event]
pub struct SwapSettledEvent {
    pub user: Pubkey,
    pub output_amount: u64,
}

#[error_code]
pub enum RouteError {
    #[msg("Protocol fee basis points exceeds max of 100 bps (1%).")]
    FeeExceedsMaximum,
    #[msg("Actual output amount is below minimum required slippage threshold.")]
    SlippageExceeded,
}
