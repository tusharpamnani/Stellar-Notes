#![cfg(test)]

// Import the contract and its client for testing.
use crate::{IncrementContract, IncrementContractClient};
use soroban_sdk::Env;

// Define a test function. The #[test] attribute marks this as a test case.
#[test]
fn test() {
    // Create a new default environment for testing. This simulates the blockchain environment.
    let env = Env::default();
    // Register the contract in the environment, returning a contract ID.
    let contract_id = env.register(IncrementContract, ());
    // Create a client to interact with the contract in tests.
    let client = IncrementContractClient::new(&env, &contract_id);

    // Call the increment function and check that it returns 1 (first increment).
    assert_eq!(client.increment(), 1);
    // Call again and check that it returns 2 (second increment).
    assert_eq!(client.increment(), 2);
    // Call again and check that it returns 3 (third increment).
    assert_eq!(client.increment(), 3);
}