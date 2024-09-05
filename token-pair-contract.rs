/*Token Pair Contract is responsible for managing two tokens in a liquidity pool and handling trades between them. The interface defines functions to add liquidity, remove liquidity, and execute swaps*/

//Interface for Token Pair Contract
pub trait TokenPair {
    fn add_liquidity(&mut self, token_a_amount: u64, token_b_amount: u64, user: Pubkey) -> Result<(), ProgramError>;
    fn remove_liquidity(&mut self, lp_tokens: u64, user: Pubkey) -> Result<(u64, u64), ProgramError>;
    fn swap(&mut self, token_in: Pubkey, amount_in: u64, user: Pubkey) -> Result<u64, ProgramError>;
    fn get_reserves(&self) -> (u64, u64);
}

//Procedures for Implementing Token Pair Contract
//1. Define State: Create a struct to store the reserve amounts for both tokens and track the liquidity provided by users.
pub struct LiquidityPool {
    pub token_a_reserve: u64,
    pub token_b_reserve: u64,
    pub lp_token_supply: u64,
}

//2. Implement add_liquidity: Allow users to add liquidity to the pool by depositing both tokens.
impl TokenPair for LiquidityPool {
    fn add_liquidity(&mut self, token_a_amount: u64, token_b_amount: u64, user: Pubkey) -> Result<(), ProgramError> {
        self.token_a_reserve += token_a_amount;
        self.token_b_reserve += token_b_amount;
        // Mint LP tokens proportional to liquidity added.
        Ok(())
    }
}

//3. Implement remove_liquidity: Users can burn their LP tokens and withdraw the underlying assets. 
impl TokenPair for LiquidityPool {
    fn remove_liquidity(&mut self, lp_tokens: u64, user: Pubkey) -> Result<(u64, u64), ProgramError> {
        let proportion = lp_tokens as f64 / self.lp_token_supply as f64;
        let token_a_withdrawn = (self.token_a_reserve as f64 * proportion) as u64;
        let token_b_withdrawn = (self.token_b_reserve as f64 * proportion) as u64;
        self.token_a_reserve -= token_a_withdrawn;
        self.token_b_reserve -= token_b_withdrawn;
        // Burn LP tokens from the user
        Ok((token_a_withdrawn, token_b_withdrawn))
    }
}



 //4. Implement swap: Allow users to swap one token for another using contant product AMM. 
 impl TokenPair for LiquidityPool {
    fn swap(&mut self, token_in: Pubkey, amount_in: u64, user: Pubkey) -> Result<u64, ProgramError> {
        let token_out_amount;
        if token_in == token_a_pubkey {
            let new_reserve_a = self.token_a_reserve + amount_in;
            token_out_amount = self.token_b_reserve - (self.token_a_reserve * self.token_b_reserve) / new_reserve_a;
            self.token_a_reserve = new_reserve_a;
            self.token_b_reserve -= token_out_amount;
        } else {
            let new_reserve_b = self.token_b_reserve + amount_in;
            token_out_amount = self.token_a_reserve - (self.token_a_reserve * self.token_b_reserve) / new_reserve_b;
            self.token_b_reserve = new_reserve_b;
            self.token_a_reserve -= token_out_amount;
        }
        Ok(token_out_amount)
    }
}

//5. Implement get_reserves: Returns the current reserves of both tokens. 
impl TokenPair for LiquidityPool {
    fn get_reserves(&self) -> (u64, u64) {
        (self.token_a_reserve, self.token_b_reserve)
    }
}




