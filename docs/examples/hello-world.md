# Hello World Contract

The Hello World contract is your gateway into Stellar smart contract development. This simple yet complete example demonstrates the fundamental concepts you'll use in every contract you build.

## What This Example Teaches

- **Contract Structure** - How to organize your smart contract code
- **Function Definition** - Creating callable functions
- **Parameter Handling** - Working with input data
- **Return Values** - Sending data back to callers
- **Deployment Process** - Getting your contract on the network
- **Interaction Patterns** - How users interact with your contract

## Project Overview

This is a minimal smart contract that:
1. Takes a name as input
2. Returns a personalized greeting
3. Demonstrates basic contract architecture
4. Shows the complete development workflow

## Code Walkthrough

### **Complete Contract Code**

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

### **Line-by-Line Explanation**

#### **`#![no_std]`**
- **Purpose**: Tells Rust not to include the standard library
- **Why needed**: WebAssembly targets don't support the full Rust standard library
- **Impact**: You can only use core Rust features and Soroban SDK

#### **`use soroban_sdk::{...}`**
- **Purpose**: Imports the Stellar SDK components we need
- **`contract`**: Macro for marking contract structs
- **`contractimpl`**: Macro for marking contract implementations
- **`symbol_short!`**: Macro for creating short symbols (efficient strings)
- **`Env`**: The contract environment (provides access to blockchain features)
- **`Symbol`**: Stellar's string type (more efficient than regular strings)

#### **`#[contract]`**
- **Purpose**: Marks this struct as a smart contract
- **What it does**: Generates necessary boilerplate code
- **Why needed**: Tells the Soroban runtime this is a contract, not regular code

#### **`pub struct HelloContract;`**
- **Purpose**: Defines the contract structure
- **`pub`**: Makes the struct public (accessible from outside)
- **`HelloContract`**: The name of your contract
- **`;`**: Unit struct (no fields, just a name)

#### **`#[contractimpl]`**
- **Purpose**: Marks this implementation as contract functions
- **What it does**: Generates client code and ABI information
- **Why needed**: Makes functions callable from outside the contract

#### **`pub fn hello(env: Env, to: Symbol) -> Vec<Symbol>`**
- **Purpose**: Defines the main contract function
- **`pub fn`**: Public function declaration
- **`hello`**: Function name (what users call)
- **`env: Env`**: Contract environment parameter (always first)
- **`to: Symbol`**: Input parameter (the name to greet)
- **`-> Vec<Symbol>`**: Return type (vector of symbols)

#### **`vec![env, symbol_short!("Hello"), to]`**
- **Purpose**: Creates and returns the greeting
- **`vec!`**: Macro to create a vector
- **`env`**: The environment (required for all contract functions)
- **`symbol_short!("Hello")`: Creates the word "Hello"
- **`to`**: The name parameter passed in

## Building the Contract

### **Step 1: Navigate to the Project**
```bash
cd soroban-hello-world/contracts/hello-world
```

### **Step 2: Build the Contract**
```bash
stellar contract build
```

### **What Happens During Build**

1. **Rust Compilation**: Your Rust code compiles to WebAssembly
2. **WASM Generation**: Output saved to `target/wasm32v1-none/release/hello-world.wasm`
3. **Size Optimization**: Contract is optimized for blockchain deployment
4. **Validation**: Basic checks ensure the contract is valid

### **Build Output**

Successful build shows:
```
✅ Contract wasm built successfully
📁 Output: target/wasm32v1-none/release/hello-world.wasm
```

## Testing the Contract

### **Run Tests**
```bash
cargo test
```

### **Test Output**
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

**What the test does**:
1. **Creates test environment**: `Env::default()` simulates the blockchain
2. **Registers contract**: `env.register_contract()` puts the contract in the test environment
3. **Creates client**: `HelloContractClient::new()` creates a way to call the contract
4. **Calls function**: `client.hello()` invokes the contract function
5. **Verifies result**: `assert_eq!` checks the response matches expectations

## Deploying to Testnet

### **Prerequisites**
- Stellar CLI installed and configured
- Testnet identity with Lumens
- Contract built successfully

### **Check Identity**
```bash
stellar keys list
```

### **Create Identity (if needed)**
```bash
stellar keys generate --global alice --network testnet --fund
```

### **Deploy Contract**
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello-world.wasm \
  --source alice \
  --network testnet \
  --alias hello-world
```

### **Deployment Output**
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

**Important**: Save the contract ID! You'll need it for all interactions.

## Interacting with Your Contract

### **Simulate a Call (Read-Only)**
```bash
stellar contract invoke \
  --id CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to RPC
```

**Output**:
```
ℹ️  Simulation identified as read-only. Send by rerunning with `--send=yes`.
["Hello","RPC"]
```

### **Execute a Transaction**
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

**Output**:
```
["Hello","YourName"]
```

## Understanding the Response

### **Response Format**
The contract returns a `Vec<Symbol>` containing:
- `env`: The environment (always included)
- `"Hello"`: The greeting word
- `to`: The name parameter you passed

### **Why This Format?**
- **Environment**: Required for all contract functions
- **Vector**: Allows returning multiple values
- **Symbols**: Stellar's efficient string type
- **Consistent**: Same pattern used in all contracts

## Exploring Your Contract

### **View on Stellar Expert**
Visit the contract explorer link from deployment:
```
https://stellar.expert/explorer/testnet/contract/CDO2UIEFLB4COWHHOKZDYPKBXL47WQXDHS2Z3NYCSDAH5OCARZPI54RR
```

### **What You'll See**
- Contract address and details
- Transaction history
- Network information
- Contract metadata

### **Try Different Names**
```bash
# Test various inputs
stellar contract invoke --id <contract-id> --source alice --network testnet --send=yes -- hello --to "Alice"
stellar contract invoke --id <contract-id> --source alice --network testnet --send=yes -- hello --to "Bob"
stellar contract invoke --id <contract-id> --source alice --network testnet --send=yes -- hello --to "World"
```

## Customization Ideas

### **Add More Functions**
```rust
#[contractimpl]
impl HelloContract {
    pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![env, symbol_short!("Hello"), to]
    }
    
    pub fn goodbye(env: Env, to: Symbol) -> Vec<Symbol> {
        vec![env, symbol_short!("Goodbye"), to]
    }
    
    pub fn greet(env: Env, greeting: Symbol, to: Symbol) -> Vec<Symbol> {
        vec![env, greeting, to]
    }
}
```

### **Add Validation**
```rust
pub fn hello(env: Env, to: Symbol) -> Vec<Symbol> {
    // Ensure name is not empty
    if to.len() == 0 {
        panic!("Name cannot be empty");
    }
    
    vec![env, symbol_short!("Hello"), to]
}
```

### **Add Multiple Languages**
```rust
pub fn hello_multilingual(env: Env, to: Symbol, language: Symbol) -> Vec<Symbol> {
    let greeting = match language.as_str() {
        "es" => symbol_short!("Hola"),
        "fr" => symbol_short!("Bonjour"),
        "de" => symbol_short!("Hallo"),
        _ => symbol_short!("Hello"),
    };
    
    vec![env, greeting, to]
}
```

## Common Issues and Solutions

### **Build Errors**

**"Can't find crate for 'core'"**
```bash
rustup target add wasm32v1-none
```

**"Function not found" during invocation**
- Check function name spelling
- Ensure you're calling the right function
- Verify contract code matches deployment

### **Deployment Issues**

**"Insufficient balance"**
```bash
stellar keys generate --global alice --network testnet --fund
```

**"Contract not found"**
- Double-check contract ID
- Verify you're on the right network
- Ensure deployment was successful

### **Interaction Problems**

**"Parameter not found"**
- Check parameter names (`--to`, not `--name`)
- Ensure parameter values are provided
- Verify parameter types match

## What You've Learned

### **Core Concepts**
- ✅ Smart contract structure and organization
- ✅ Function definition and parameters
- ✅ Return values and data types
- ✅ Contract deployment workflow
- ✅ Contract interaction patterns

### **Technical Skills**
- ✅ Writing Rust code for Soroban
- ✅ Building contracts to WebAssembly
- ✅ Testing contract functionality
- ✅ Deploying to Stellar testnet
- ✅ Using Stellar CLI commands

### **Development Workflow**
- ✅ Write → Build → Test → Deploy → Interact
- ✅ Iterative development process
- ✅ Error handling and debugging
- ✅ Contract lifecycle management

## Next Steps

### **Immediate Next Steps**
1. **Modify the contract** - Add new functions or change behavior
2. **Experiment with parameters** - Try different input types
3. **Test edge cases** - See what happens with invalid inputs

### **Learning Path**
1. **[Increment Counter](increment-counter.md)** - Learn about state management
2. **[Contract State Management](smart-contracts/contract-state.md)** - Understand data persistence
3. **[Testing Strategies](smart-contracts/testing-contracts.md)** - Improve your testing skills

### **Advanced Topics**
- [Security Best Practices](advanced/security-best-practices.md)
- [Contract Optimization](advanced/contract-optimization.md)
- [Frontend Integration](frontend/README.md)

## Summary

Congratulations! You've successfully:

- ✅ **Built** your first smart contract
- ✅ **Deployed** it to the Stellar testnet
- ✅ **Interacted** with it using the CLI
- ✅ **Understood** the complete development workflow

This Hello World contract may be simple, but it demonstrates all the fundamental patterns you'll use in every smart contract you build. The concepts you've learned here—structure, functions, deployment, and interaction—are the building blocks of complex blockchain applications.

Remember: Every expert was once a beginner. You've taken the first step into a world of possibilities!

---

**Next**: [Increment Counter](increment-counter.md) - Learn how contracts store and manage state
