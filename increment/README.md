# Soroban Increment Contract Project


The `increment` contract stores a single counter value on-chain. Each time the `increment` function is called, the counter increases by one and the new value is returned. This demonstrates persistent contract state and basic contract logic.

---

## Project Structure

```
.
├── Cargo.toml                # Workspace manifest
├── Cargo.lock                # Dependency lockfile
├── README.md                 # This file
└── contracts/
    └── increment/
        ├── Cargo.toml        # Contract manifest
        ├── Makefile         # Build/test commands
        ├── src/
        │   ├── lib.rs       # Main contract code
        │   └── test.rs      # Contract tests
        └── test_snapshots/  # Test output (for snapshot testing)
```

- **contracts/increment/src/lib.rs**: The main contract logic.
- **contracts/increment/src/test.rs**: Unit tests for the contract.
- **contracts/increment/Makefile**: Helper commands for building, testing, formatting, and cleaning.
- **contracts/increment/test_snapshots/**: Stores test output for snapshot-based testing.

---

## Building the Contract

You can build the contract using the provided Makefile or directly with the Soroban CLI:

### Using Makefile
```sh
cd contracts/increment
make build
```
This will compile the contract to WASM and place the output in `target/wasm32v1-none/release/`.

### Using Soroban CLI
```sh
cd contracts/increment
soroban contract build
```

---

## Testing the Contract

Run the tests to verify the contract logic:

### Using Makefile
```sh
cd contracts/increment
make test
```

### Using Cargo Directly
```sh
cd contracts/increment
cargo test
```

The tests check that the counter increments as expected:
- First call returns 1
- Second call returns 2
- Third call returns 3

---

## How the Contract Works

### Main Logic (`src/lib.rs`)
```rust
const COUNTER: Symbol = symbol_short!("COUNTER");

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Increment increments an internal counter, and returns the value.
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&COUNTER).unwrap_or(0);
        log!(&env, "count: {}", count);

        count += 1;
        env.storage().instance().set(&COUNTER, &count);
        env.storage().instance().extend_ttl(50, 100);

        count
    }
}
```
- **State:** The counter is stored in contract instance storage under the key `COUNTER`.
- **Increment:** Each call reads the current value, increments it, stores it back, and returns the new value.
- **Persistence:** The counter persists between calls and is not reset unless the contract is redeployed.

---

## Interacting with the Contract

You can deploy and interact with the contract using the Soroban CLI. Here's a basic example:

1. **Build the contract:**
   ```sh
   soroban contract build
   ```
2. **Deploy the contract:**
   ```sh
   soroban contract deploy --wasm target/wasm32v1-none/release/increment.wasm
   ```
   Note the contract ID returned.
3. **Invoke the increment function:**
   ```sh
   soroban contract invoke \
     --id <contract-id> \
     --fn increment
   ```
   Each call will increment and return the counter value.

---

## Advanced: Test Snapshots

- The `test_snapshots` directory contains JSON files that record the state of the contract and ledger after tests run.
- These are used for snapshot testing, ensuring that contract behavior does not change unexpectedly.
- You usually do not need to edit these files manually.

---

## Extending the Project

- **Add new contracts:** Place each new contract in its own directory under `contracts/` and add it to the workspace in the top-level `Cargo.toml`.
- **Add frontend:** You can add a frontend app at the top level if desired (not included by default).

---

## References & Further Reading
- [Soroban Documentation](https://soroban.stellar.org/docs/)
- [Stellar Developers](https://developers.stellar.org/)
- [Rust Language](https://www.rust-lang.org/)
- [Soroban CLI Reference](https://soroban.stellar.org/docs/reference/cli)

---

## License
This project is for educational purposes and does not include a specific license. Please add one if you intend to use it in production or share it publicly.