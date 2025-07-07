#![no_std]

use soroban_sdk::{contract, contractimpl, log, symbol_short, Env, Symbol};

// Define a constant key for storing the counter in contract storage.
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increments an internal counter and returns the new value.
    ///
    /// # Arguments
    /// * `env` - The environment object, which provides access to storage and other blockchain features.
    pub fn increment(env: Env) -> u32 {
        // Try to get the current counter value from storage. If it doesn't exist, start at 0.
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        // Log the current count (before incrementing) for debugging and transparency.
        log!(&env, "count: {}", count);

        // Increment the counter by 1.
        count += 1;
        // Store the new counter value back into contract storage.
        env.storage().instance().set(&COUNTER, &count);
        // Extend the time-to-live (TTL) for this storage entry, so it persists for a while.
        env.storage().instance().extend_ttl(50, 100);

        // Return the new counter value.
        count
    }
}

// The test module is included below, but only compiled when running tests.
mod test;