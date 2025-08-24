# Your First Contract

Congratulations! You're about to build your first smart contract on Stellar. This tutorial will walk you through creating a simple "Hello World" contract, building it, testing it, and deploying it to the testnet.

## What We're Building

A simple smart contract that:
- Takes a name as input
- Returns a personalized greeting
- Demonstrates basic contract structure
- Shows how to deploy and interact with contracts

## Project Structure

We'll be working with the existing `soroban-hello-world` project in your workspace. The structure looks like this:

```
soroban-hello-world/
├── Cargo.toml              # Project configuration
├── contracts/
│   └── hello-world/
│       ├── Cargo.toml      # Contract dependencies
│       ├── src/
│       │   ├── lib.rs      # Contract code
│       │   └── test.rs     # Tests
│       └── Makefile        # Build commands
```

## Step 1: Explore the Contract Code

Let's start by looking at the existing contract code:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol};

#[contract]
pub struct HelloContract;

#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![env, symbol_short!("Hello"), to]
    }
}
```

### **Understanding the Code**

Let's break down what each part does:

- **`#![no_std]`**: Tells Rust not to include the standard library (required for WASM)
- **`use soroban_sdk::...`**: Imports the Stellar SDK
- **`#[contract]`**: Marks this struct as a smart contract
- **`#[contractimpl]`**: Marks this implementation as contract functions
- **`pub fn hello(...)`**: A public function that anyone can call
- **`Vec<Symbol>`**: Returns a vector of symbols (Stellar's string type)

## Step 2: Build the Contract

Now let's build the contract into WebAssembly format:

```bash
cd soroban-hello-world/contracts/hello-world
stellar contract build
```

**What happens during build:**
1. Rust compiler converts your code to WebAssembly
2. Output is saved to `target/wasm32v1-none/release/hello-world.wasm`
3. The `.wasm` file is what gets deployed to the blockchain

### **If you get build errors:**

**"Can't find crate for 'core'" error:**
```bash
rustup target add wasm32v1-none
```

**For older Rust versions:**
```bash
rustup target add wasm32-unknown-unknown
```

## Step 3: Test the Contract

Before deploying, let's make sure the contract works correctly:

```bash
cargo test
```

You should see output like:
```
running 1 test
test test::test ... ok

test result: ok. 1 test passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### **Understanding the Test**

The test file (`src/test.rs`) contains:

```rust
#[test]
fn test() {
    let env = Env::default();
    let contract_id = env.register_contract(None, HelloContract);
    let client = HelloContractClient::new(&env, &contract_id);
    
    let words = client.hello(&symbol_short!("Dev"));
    assert_eq!(words, vec![symbol_short!("Hello"), symbol_short!("Dev")]);
}
```

This test:
1. Creates a test environment
2. Registers the contract
3. Calls the `hello` function
4. Verifies the response

## Step 4: Deploy to Testnet

Now for the exciting part—deploying your contract to the Stellar testnet!

### **Make sure you have an identity:**
```bash
stellar keys list
```

If you don't see any identities, create one:
```bash
stellar keys generate --global alice --network testnet --fund
```

### **Deploy the contract:**
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello-world.wasm \
  --source alice \
  --network testnet \
  --alias hello-world
```

**What this command does:**
- `--wasm`: Points to your compiled contract
- `--source alice`: Uses your "alice" identity to pay for deployment
- `--network testnet`: Deploys to the test network (free)
- `--alias hello-world`: Gives your contract a friendly name

### **Deployment Output**

You'll see output like this:
```
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

**Important**: Save the contract ID (`CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR`) - you'll need it to interact with your contract!

## Step 5: Interact with Your Contract

Now let's test your deployed contract:

```bash
stellar contract invoke \
  --id CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to RPC
```

**Replace the contract ID** with the one you got from deployment.

### **Understanding the Command**

- `--id`: Your contract's address on the blockchain
- `--source alice`: Your identity (who's calling the contract)
- `--network testnet`: Which network to use
- `--`: Separates CLI options from function arguments
- `hello`: The function to call
- `--to RPC`: The parameter to pass to the function

### **Expected Output**

```
ℹ️  Simulation identified as read-only. Send by rerunning with `--send=yes`.
["Hello","RPC"]
```

The first call is a simulation. To actually execute the transaction:

```bash
stellar contract invoke \
  --id CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR \
  --source alice \
  --network testnet \
  --send=yes \
  -- \
  hello \
  --to YourName
```

## Step 6: Explore Your Contract

### **View on Stellar Expert**

Visit the contract explorer link from your deployment output:
```
https://stellar.expert/explorer/testnet/contract/CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR
```

This shows:
- Contract details
- Transaction history
- Current state
- Network information

### **Try Different Inputs**

Test your contract with various names:

```bash
# Test with different names
stellar contract invoke --id <your-contract-id> --source alice --network testnet --send=yes -- hello --to "Alice"
stellar contract invoke --id <your-contract-id> --source alice --network testnet --send=yes -- hello --to "Bob"
stellar contract invoke --id <your-contract-id> --source alice --network testnet --send=yes -- hello --to "World"
```

## Understanding What Happened

### **The Complete Flow**

1. **Development**: You wrote Rust code
2. **Compilation**: Rust compiled it to WebAssembly
3. **Deployment**: Contract was uploaded to the Stellar testnet
4. **Execution**: Users can now call your contract functions
5. **Persistence**: Contract runs on the blockchain forever

### **Key Concepts Demonstrated**

- **Smart Contracts**: Self-executing code on the blockchain
- **Deployment**: Getting your code onto the network
- **Interaction**: Calling contract functions from the outside
- **Testnet**: Safe environment for development and testing

## Common Issues and Solutions

### **"Contract not found" error**
- Double-check the contract ID
- Make sure you're on the right network (testnet)
- Verify the contract was deployed successfully

### **"Insufficient balance" error**
- Your identity needs testnet Lumens
- Run: `stellar keys generate --global alice --network testnet --fund`

### **"Function not found" error**
- Check the function name spelling
- Make sure you're calling the right function
- Verify the contract code matches what you deployed

## What's Next?

Congratulations! You've successfully:
- ✅ Built a smart contract
- ✅ Deployed it to the testnet
- ✅ Interacted with it
- ✅ Understood the basic workflow

### **Next Steps**

1. **Modify the Contract**: Change the greeting message or add new functions
2. **Learn State Management**: Move to [Contract State Management](contract-state.md)
3. **Build a Frontend**: Create a web app to interact with your contract
4. **Explore Advanced Patterns**: Learn about more complex contract structures

### **Experiment Ideas**

- Add a function that counts how many times `hello` has been called
- Create a function that stores the last name used
- Add validation to ensure names aren't empty
- Create a function that returns different greetings based on time

## Summary

You've taken your first step into the world of smart contract development! You now understand:

- How to structure a Soroban contract
- The build, test, and deploy workflow
- How to interact with deployed contracts
- Basic contract architecture patterns

The foundation you've built here will serve you well as you move on to more complex contracts and applications.

---

**Next**: [Contract State Management](contract-state.md) - Learning how to store and manage data in your contracts
