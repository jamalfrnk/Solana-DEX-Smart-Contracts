/*Pair Factory Contract is responsible for deploying and managing multiple token pairs. It ensures that liquidity pools for specific pairs are created only once.*/

//Interface for Pair Factory Contract
pub trait PairFactory {
    fn create_pair(&mut self, token_a: Pubkey, token_b: Pubkey) -> Result<Pubkey, ProgramError>;
    fn get_pair(&self, token_a: Pubkey, token_b: Pubkey) -> Result<Pubkey, ProgramError>;
}

//Procedures for Implementing Pair Factory Contract 
//1. Define State: Track deployed token pairs. 
pub struct Factory {
    pub pairs: HashMap<(Pubkey, Pubkey), Pubkey>, // Maps token pair to the address of the liquidity pool
}

//2. Implement create_pair: Create a new pair if one doesn't exist. 
impl PairFactory for Factory {
    fn create_pair(&mut self, token_a: Pubkey, token_b: Pubkey) -> Result<Pubkey, ProgramError> {
        let pair_key = (token_a, token_b);
        if self.pairs.contains_key(&pair_key) {
            return Err(ProgramError::Custom(1)); // Pair already exists
        }
        let pool_address = create_liquidity_pool(token_a, token_b)?;
        self.pairs.insert(pair_key, pool_address);
        Ok(pool_address)
    }
}

//3. Implement get_pair: Return the address of the liquidity pool for the given pair. 
impl PairFactory for Factory {
    fn get_pair(&self, token_a: Pubkey, token_b: Pubkey) -> Result<Pubkey, ProgramError> {
        let pair_key = (token_a, token_b);
        self.pairs.get(&pair_key).cloned().ok_or(ProgramError::Custom(2)) // Pair does not exist
    }
}
