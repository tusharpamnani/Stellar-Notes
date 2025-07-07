// This attribute tells Rust to only compile and run this module when running tests (e.g., with `cargo test`).
// It ensures that test code is not included in the final WASM contract, keeping it small and efficient.
#![cfg(test)]

// Bring all items from the parent module (the contract) into scope, so we can test them directly.
use super::*;
// Import Soroban SDK helpers for testing: `vec` for vectors, `Env` for the environment, and `String` for Soroban-compatible strings.
use soroban_sdk::{vec, Env, String};

// The `#[test]` attribute marks this function as a test case.
#[test]
fn test() {
    // Create a new default Soroban environment for testing.
    // This simulates the blockchain context in which the contract runs.
    let env = Env::default();

    // Register the contract in the test environment.
    // This returns a unique contract ID that can be used to interact with the contract.
    let contract_id = env.register(Contract, ());

    // Create a client for the contract, allowing us to call its functions as if we were an external user.
    let client = ContractClient::new(&env, &contract_id);

    // Call the `hello` function on the contract, passing in the string "Tushar".
    // The result is a vector of strings returned by the contract.
    let words = client.hello(&String::from_str(&env, "Tushar"));

    // Assert that the returned vector matches our expectation: ["Hi There", "Tushar"].
    // If the assertion fails, the test will fail, indicating a problem with the contract logic.
    assert_eq!(
        words,
        vec![
            &env,
            String::from_str(&env, "Hi There"),
            String::from_str(&env, "Tushar"),
        ]
    );
}
