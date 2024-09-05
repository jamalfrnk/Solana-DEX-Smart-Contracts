//Test lp_reward_calculations.rs

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::clock;

    #[test]
    fn test_reward_calculation() {
        // Simulate adding liquidity and calculating rewards for multiple users
        let mut pool = LiquidityPool {
            total_liquidity: 0,
            reward_pool: 1000,
        };
        let mut user1 = User {
            liquidity: 0,
            time_factor: 1,
            reward: 0,
        };
        let mut user2 = User {
            liquidity: 0,
            time_factor: 2,
            reward: 0,
        };

        // User1 adds liquidity
        pool.total_liquidity += 500;
        user1.liquidity = 500;

        // User2 adds liquidity
        pool.total_liquidity += 300;
        user2.liquidity = 300;

        // Calculate rewards for both users
        let reward1 = (user1.liquidity as f64 / pool.total_liquidity as f64) * pool.reward_pool as f64 * user1.time_factor as f64;
        let reward2 = (user2.liquidity as f64 / pool.total_liquidity as f64) * pool.reward_pool as f64 * user2.time_factor as f64;

        assert_eq!(reward1 as u64, 555);
        assert_eq!(reward2 as u64, 444);
    }
}
