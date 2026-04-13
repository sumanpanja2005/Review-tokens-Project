#![no_std]

use soroban_sdk::{contract, contractimpl, Env, Symbol, Map, String, Address};

#[contract]
pub struct ReviewToken;

#[contractimpl]
impl ReviewToken {

    // Store review count per user
    pub fn submit_review(env: Env, user: Address, review: String) {
        let key = Symbol::new(&env, "reviews");

        let mut reviews: Map<Address, u32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let count = reviews.get(user.clone()).unwrap_or(0);
        reviews.set(user.clone(), count + 1);

        env.storage().instance().set(&key, &reviews);

        // Reward tokens
        Self::mint_token(env, user);
    }

    // Mint reward token (basic counter-based)
    fn mint_token(env: Env, user: Address) {
        let key = Symbol::new(&env, "tokens");

        let mut balances: Map<Address, u32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        let balance = balances.get(user.clone()).unwrap_or(0);
        balances.set(user.clone(), balance + 10); // reward: 10 tokens

        env.storage().instance().set(&key, &balances);
    }

    // Check token balance
    pub fn get_balance(env: Env, user: Address) -> u32 {
        let key = Symbol::new(&env, "tokens");

        let balances: Map<Address, u32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        balances.get(user).unwrap_or(0)
    }

    // Get total reviews submitted
    pub fn get_reviews(env: Env, user: Address) -> u32 {
        let key = Symbol::new(&env, "reviews");

        let reviews: Map<Address, u32> =
            env.storage().instance().get(&key).unwrap_or(Map::new(&env));

        reviews.get(user).unwrap_or(0)
    }
}