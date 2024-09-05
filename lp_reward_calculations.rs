/* Use "bash" to install anchor framework: cargo install --git https://github.com/project-serum/anchor --tag v0.24.2 anchor-cli --locked*/

//1. Define State: Track liquidity pools and user information, such as their liquidity and time of participation. 
use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkgv2kMXWg5kF");

#[program]
pub mod solana_dex_rewards {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, reward_pool: u64) -> ProgramResult {
        let pool = &mut ctx.accounts.liquidity_pool;
        pool.total_liquidity = 0;
        pool.reward_pool = reward_pool;
        Ok(())
    }

    pub fn add_liquidity(ctx: Context<AddLiquidity>, amount: u64, time_factor: u64) -> ProgramResult {
        let pool = &mut ctx.accounts.liquidity_pool;
        let user = &mut ctx.accounts.user;

        // Update user liquidity and time factor
        user.liquidity += amount;
        user.time_factor = time_factor;

        // Update total liquidity in the pool
        pool.total_liquidity += amount;
        Ok(())
    }

    pub fn calculate_rewards(ctx: Context<CalculateRewards>) -> ProgramResult {
        let pool = &ctx.accounts.liquidity_pool;
        let user = &ctx.accounts.user;

        // Reward formula: (liquidity / total_liquidity) * reward_pool * time_factor
        let reward = (user.liquidity as u64 * user.time_factor as u64) / pool.total_liquidity as u64 * pool.reward_pool as u64;
        user.reward = reward;
        Ok(())
    }
}

//2. Define Accounts to store state about liquidity pools and users. 
#[account]
pub struct LiquidityPool {
    pub total_liquidity: u64,
    pub reward_pool: u64,
}
pub struct User {
    pub liquidity: u64,
    pub time_factor: u64, // how long they provided liquidity
    pub reward: u64,      // Calculated rewards for the user
}

//3. Define instructions for the users to interact with the contract. 
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = authority, space = 8 + 16)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AddLiquidity<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub user: Account<'info, User>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct CalculateRewards<'info> {
    #[account(mut)]
    pub liquidity_pool: Account<'info, LiquidityPool>,
    #[account(mut)]
    pub user: Account<'info, User>,
}

