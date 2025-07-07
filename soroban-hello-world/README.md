# Soroban Hello World Contract

This repository contains a minimal **Soroban smart contract workspace**, built in Rust, to help you get started with Stellar smart contracts development.
It includes a **Hello World contract** with a workspace layout, optimized build profiles, and inline documentation in the code.

---

## 📦 Project Structure

```
.
├── Cargo.lock
├── Cargo.toml
├── README.md
└── contracts
    └── hello-world
        ├── Cargo.toml
        ├── Makefile
        └── src
            ├── lib.rs
            └── test.rs
```

* `Cargo.toml` (root) — defines the Rust workspace and shared dependencies.
* `contracts/hello-world` — contains the Hello World contract implementation.
* `src/lib.rs` — the contract source code (with inline explanations).
* `src/test.rs` — example tests.

---

## 🧑‍💻 Build & Test

Navigate to the contract directory and build the `.wasm` file:

```bash
cd contracts/hello-world
stellar contract build
```

This command will compile your contract to WebAssembly (WASM), which is the format required for deployment on the Stellar Soroban platform.

Run tests:

```bash
cargo test
```
You'll see the output as
```bash
running 1 test
test test::test ... ok
```

Build the Contract (again, if needed):

```bash
stellar contract build
```

If you get an error like can't find crate for `'core'`, it means you didn't install the `wasm32 target` during the setup step. You can do so by running:

```bash
rustup target add wasm32v1-none
```

(Use `rustup target add wasm32-unknown-unknown` for Rust versions older than `v1.85.0`).

A `.wasm` file will be outputted in the `target` directory, at `target/wasm32v1-none/release/hello-world.wasm` or `target/wasm32-unknown-unknown/release/hello-world.wasm` (if you are using the older version). The `.wasm` file is the built contract that you will deploy to the blockchain.


### Optimizing Builds

Use `stellar contract optimize` to further minimize the size of the `.wasm`. First, re-install `stellar-cli` with the `opt` feature:

```bash
cargo install --locked stellar-cli --features opt
```

Then build an optimized `.wasm` file:

```bash
stellar contract optimize --wasm target/wasm32v1-none/release/hello-world.wasm
```

If you are using the older version:

```bash
stellar contract optimize --wasm target/wasm32-unknown-unknown/release/hello-world.wasm
```

This will optimize and output a new `hello-world.optimized.wasm` file in the same location as the input `.wasm`.

### Deploy to Testnet

To deploy your contract to the Stellar testnet, use the following command:

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello-world.wasm \
  --source alice \
  --network testnet \
  --alias hello-world
```

Or (if on an older version):

```bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/hello-world.wasm \
  --source alice \
  --network testnet \
  --alias hello-world
```

You'll see an output like this:

```bash
ℹ️  Skipping install because wasm already installed
ℹ️  Using wasm hash 8e61eb9cc3d9afd49d9aa295a579aaddf1f85855e5fc2f1148b3185debfe8c86
ℹ️  Simulating deploy transaction…
ℹ️  Transaction hash is e6f75debdabc95f0376185dc88db3aff62eb28e5899fcd8026ee69b0638ee758
🔗 https://stellar.expert/explorer/testnet/tx/e6f75debdabc95f0376185dc88db3aff62eb28e5899fcd8026ee69b0638ee758
ℹ️  Signing transaction: e6f75debdabc95f0376185dc88db3aff62eb28e5899fcd8026ee69b0638ee758
🌎 Submitting deploy transaction…
🔗 https://stellar.expert/explorer/testnet/contract/CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR
✅ Deployed!
CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR
```

### Interacting with the Contract

You can now interact with your deployed contract using the following command:

```bash
stellar contract invoke \
  --id CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to RPC
```

Replace `CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR` with the contract address you got from the deployment.

You'll see the output as

```bash
ℹ️  Simulation identified as read-only. Send by rerunning with `--send=yes`.
["Hello","RPC"]
```

---

## 🔍 Workspace Details

The root `Cargo.toml` sets up a Rust workspace:

```toml
[workspace]
resolver = "2"
members = [
  "contracts/*",
]

[workspace.dependencies]
soroban-sdk = "22"
```

### Optimized Build Profiles

Soroban contracts are limited to 64KB. To keep your `.wasm` small:

```toml
[profile.release]
opt-level = "z"
overflow-checks = true
debug = 0
strip = "symbols"
debug-assertions = false
panic = "abort"
codegen-units = 1
lto = true
```

For debugging with logs enabled:

```toml
[profile.release-with-logs]
inherits = "release"
debug-assertions = true
```

---

## 📄 Next Steps

Open `contracts/hello-world/src/lib.rs` to explore and implement the contract logic.
You'll find inline comments explaining:

* Why `#![no_std]` is used.
* How `#[contract]` and `#[contractimpl]` work.
* How to define and expose contract functions.

---

Happy building on Stellar! 🌟
