// This attribute tells the Rust compiler not to use the standard library, which is required for smart contracts on Soroban.
// Soroban contracts must be as small as possible and compatible with WASM environments that do not have the full Rust standard library.
#![no_std]

// Import necessary types and macros from the Soroban SDK.
// - `contract` and `contractimpl` are procedural macros that help define and expose contract logic.
// - `vec`, `Env`, `String`, and `Vec` are types and helpers for working with the Soroban environment and contract data.
use soroban_sdk::{contract, contractimpl, vec, Env, String, Vec};

// The `#[contract]` macro marks this struct as the main contract object.
// Soroban uses this to identify the contract's entry point.
#[contract]
pub struct Contract;

// The implementation block below defines the contract's public functions.
// The `#[contractimpl]` macro exposes these functions to the blockchain, making them callable by users or other contracts.
#[contractimpl]
impl Contract {
    // This is a simple contract function named `hello`.
    //
    // Parameters:
    // - `env`: The Soroban environment, which provides access to blockchain context and utilities.
    // - `to`: A `String` parameter, representing the name or entity to greet.
    //
    // Returns:
    // - A `Vec<String>` containing two elements: "Hello" and the value of `to`.
    //
    // Example:
    //   If called with `to = "World"`, it returns `["Hello", "World"]`.
    //
    // This demonstrates how to return data from a contract and how to use Soroban's types.
    pub fn hello(env: Env, to: String) -> Vec<String> {
        // `vec!` is a macro to create a new vector in the Soroban environment.
        // `String::from_str(&env, "Hello")` creates a Soroban-compatible string.
        // The vector is returned as the function's result.
        vec![&env, String::from_str(&env, "Hi There"), to]
    }
}

// The `mod test;` line includes the test module, which is defined in a separate file (`test.rs`).
// This allows you to keep your tests organized and separate from your contract logic.
mod test;
