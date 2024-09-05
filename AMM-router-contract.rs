/*AMM Router Contract serves as an intermediary between users and liquidity pools, routing trades to the appropriate liquidity pool.*/

//Interface for AMM Router Contract 
pub trait AMMRouter {
    fn swap_exact_tokens_for_tokens(&mut self, amount_in: u64, min_amount_out: u64, path: Vec<Pubkey>, user: Pubkey) -> Result<u64, ProgramError>;
}

//Procedures for Implementing AMM Router Contract
//1. Define State: The router does not need to store much state, just facilitate trade routing. 
pub struct Router {}

//Implement swap_exact_tokens_for_tokens: Execute swaps across multiple liquidity pools. 
impl AMMRouter for Router {
    fn swap_exact_tokens_for_tokens(&mut self, amount_in: u64, min_amount_out: u64, path: Vec<Pubkey>, user: Pubkey) -> Result<u64, ProgramError> {
        let mut current_amount_in = amount_in;
        for i in 0..(path.len() - 1) {
            let pair_address = get_pair(path[i], path[i+1])?;
            let pool = get_liquidity_pool(pair_address)?;
            current_amount_in = pool.swap(path[i], current_amount_in, user)?;
        }
        if current_amount_in < min_amount_out {
            return Err(ProgramError::Custom(3)); // Slippage exceeded
        }
        Ok(current_amount_in)
    }
}

