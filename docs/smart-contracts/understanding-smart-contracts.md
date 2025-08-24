# Understanding Smart Contracts

Smart contracts are the foundation of programmable blockchain applications. This section will teach you the core concepts, architecture patterns, and design principles that make smart contracts powerful and secure.

## What Are Smart Contracts?

Smart contracts are self-executing programs that run on the blockchain. Think of them as digital agreements that automatically execute when certain conditions are met, without requiring human intervention or trust in a third party.

### **Key Characteristics**

- **Self-Executing**: Code runs automatically when conditions are met
- **Trustless**: No need to trust a central authority
- **Immutable**: Code cannot be changed once deployed (with some exceptions)
- **Transparent**: All code and execution is visible on the blockchain
- **Deterministic**: Same inputs always produce the same outputs

## How Smart Contracts Work

### **Execution Model**

```
User Request → Contract Function → State Change → Transaction → Blockchain
     ↑              ↓              ↓              ↓            ↓
Response ←    Result Return ←   Data Update ←  Validation ← Consensus
```

### **Lifecycle of a Smart Contract**

1. **Development**: Write code in Rust
2. **Compilation**: Convert to WebAssembly (WASM)
3. **Deployment**: Upload to the blockchain
4. **Initialization**: Set up initial state
5. **Execution**: Users call functions
6. **State Changes**: Data is updated on-chain
7. **Events**: Notifications are emitted

## Core Architecture Patterns

### **1. Contract Structure**

Every Soroban contract follows this basic pattern:

```rust
#![no_std]
use soroban_sdk::{contract, contractimpl, Env};

#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    // Contract functions go here
}
```

**Components Explained**:
- **`#![no_std]`**: Excludes standard library (required for WASM)
- **`#[contract]`**: Marks the struct as a smart contract
- **`#[contractimpl]`**: Marks the implementation as contract functions
- **`Env`**: Provides access to blockchain features

### **2. Function Patterns**

#### **Read-Only Functions**
```rust
pub fn get_balance(env: Env, user: Address) -> u32 {
    // Read data without changing state
    env.storage().instance().get(&user).unwrap_or(0)
}
```

#### **State-Changing Functions**
```rust
pub fn transfer(env: Env, from: Address, to: Address, amount: u32) -> Result<(), Error> {
    // Validate inputs
    if amount == 0 {
        return Err(Error::InvalidAmount);
    }
    
    // Update state
    let from_balance = env.storage().instance().get(&from).unwrap_or(0);
    if from_balance < amount {
        return Err(Error::InsufficientBalance);
    }
    
    // Perform transfer
    env.storage().instance().set(&from, &(from_balance - amount));
    let to_balance = env.storage().instance().get(&to).unwrap_or(0);
    env.storage().instance().set(&to, &(to_balance + amount));
    
    Ok(())
}
```

### **3. State Management Patterns**

#### **Instance Storage**
```rust
// Store data specific to this contract instance
const COUNTER: Symbol = symbol_short!("COUNTER");

pub fn increment(env: Env) -> u32 {
    let current = env.storage().instance().get(&COUNTER).unwrap_or(0);
    let new_value = current + 1;
    env.storage().instance().set(&COUNTER, &new_value);
    new_value
}
```

#### **Persistent Storage**
```rust
// Store data that persists across contract upgrades
const USER_DATA: Symbol = symbol_short!("USER_DATA");

pub fn store_user_data(env: Env, user: Address, data: String) {
    let key = (USER_DATA, user);
    env.storage().persistent().set(&key, &data);
}
```

## Design Principles

### **1. Security First**

#### **Input Validation**
```rust
pub fn safe_function(env: Env, input: String) -> Result<String, Error> {
    // Always validate inputs
    if input.is_empty() {
        return Err(Error::InvalidInput);
    }
    
    if input.len() > 100 {
        return Err(Error::InputTooLong);
    }
    
    // Process validated input
    Ok(input.to_uppercase())
}
```

#### **Access Control**
```rust
const ADMIN: Symbol = symbol_short!("ADMIN");

pub fn admin_only_function(env: Env, caller: Address) -> Result<(), Error> {
    let admin = env.storage().instance().get(&ADMIN)
        .ok_or(Error::AdminNotSet)?;
    
    if caller != admin {
        return Err(Error::Unauthorized);
    }
    
    // Admin-only logic here
    Ok(())
}
```

### **2. Gas Optimization**

#### **Efficient Data Structures**
```rust
// Use Symbols instead of Strings for better performance
const USER_BALANCE: Symbol = symbol_short!("BALANCE");

// Use appropriate integer types
pub fn get_balance(env: Env, user: Address) -> u32 {
    env.storage().instance().get(&(USER_BALANCE, user)).unwrap_or(0)
}
```

#### **Minimize Storage Operations**
```rust
// Batch operations when possible
pub fn batch_update(env: Env, updates: Vec<(Address, u32)>) -> Result<(), Error> {
    for (user, amount) in updates {
        let key = (USER_BALANCE, user);
        env.storage().instance().set(&key, &amount);
    }
    Ok(())
}
```

### **3. Error Handling**

#### **Custom Error Types**
```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    InsufficientBalance,
    InvalidAmount,
    UserNotFound,
    Unauthorized,
    InvalidInput,
}

impl From<Error> for soroban_sdk::Error {
    fn from(err: Error) -> Self {
        soroban_sdk::Error::from_type_and_code(err.into(), 0)
    }
}
```

#### **Graceful Degradation**
```rust
pub fn safe_operation(env: Env, user: Address) -> Result<u32, Error> {
    match env.storage().instance().get(&user) {
        Some(value) => Ok(value),
        None => {
            // Log the missing user for debugging
            log!(&env, "User not found: {}", user);
            Err(Error::UserNotFound)
        }
    }
}
```

## Common Patterns

### **1. Factory Pattern**
```rust
pub fn create_token(env: Env, name: String, symbol: String) -> Result<Address, Error> {
    // Deploy a new token contract
    let wasm_hash = env.deployer().upload_contract_wasm(&TOKEN_WASM);
    let contract_id = env.deployer().with_current_contract(&env.current_contract_id())
        .create_contract(&wasm_hash);
    
    // Initialize the new contract
    let client = TokenClient::new(&env, &contract_id);
    client.initialize(&name, &symbol);
    
    Ok(contract_id)
}
```

### **2. Proxy Pattern**
```rust
pub fn upgrade_contract(env: Env, new_wasm_hash: Bytes) -> Result<(), Error> {
    // Only admin can upgrade
    let admin = env.storage().instance().get(&ADMIN)
        .ok_or(Error::AdminNotSet)?;
    
    if env.current_contract().invoker() != admin {
        return Err(Error::Unauthorized);
    }
    
    // Perform the upgrade
    env.deployer().update_current_contract_wasm(&new_wasm_hash);
    Ok(())
}
```

### **3. Event Emission**
```rust
pub fn transfer(env: Env, from: Address, to: Address, amount: u32) -> Result<(), Error> {
    // ... transfer logic ...
    
    // Emit event for transparency
    env.events().publish((
        symbol_short!("transfer"),
        (from, to, amount)
    ));
    
    Ok(())
}
```

## Best Practices

### **1. Code Organization**
- Keep functions small and focused
- Use clear, descriptive names
- Group related functionality together
- Separate concerns (validation, business logic, storage)

### **2. Testing Strategy**
- Test all code paths
- Include edge cases and error conditions
- Use property-based testing for complex logic
- Test with realistic data sizes

### **3. Documentation**
- Document all public functions
- Explain complex business logic
- Provide usage examples
- Document error conditions

### **4. Security Considerations**
- Validate all inputs
- Check access permissions
- Handle edge cases gracefully
- Use safe math operations
- Avoid reentrancy vulnerabilities

## Common Pitfalls

### **1. Reentrancy Attacks**
```rust
// ❌ Dangerous - can be re-entered
pub fn unsafe_withdraw(env: Env, user: Address) -> Result<(), Error> {
    let balance = get_balance(&env, &user);
    // State change happens after external call
    env.storage().instance().set(&(BALANCE, user), &0);
    // External call could re-enter this function
    send_tokens(&env, &user, &balance);
    Ok(())
}

// ✅ Safe - state changes before external calls
pub fn safe_withdraw(env: Env, user: Address) -> Result<(), Error> {
    let balance = get_balance(&env, &user);
    // Update state first
    env.storage().instance().set(&(BALANCE, user), &0);
    // Then make external call
    send_tokens(&env, &user, &balance);
    Ok(())
}
```

### **2. Integer Overflow**
```rust
// ❌ Dangerous - can overflow
pub fn unsafe_add(a: u32, b: u32) -> u32 {
    a + b  // Could overflow
}

// ✅ Safe - check for overflow
pub fn safe_add(a: u32, b: u32) -> Result<u32, Error> {
    a.checked_add(b).ok_or(Error::Overflow)
}
```

## What's Next?

Now that you understand smart contract fundamentals, you're ready to:

1. **[Build your first contract](your-first-contract.md)** - Apply these concepts
2. **[Learn state management](contract-state.md)** - Master data persistence
3. **[Explore testing strategies](testing-contracts.md)** - Ensure reliability
4. **[Deploy to networks](deploying-contracts.md)** - Get contracts running

Remember: Smart contract development requires careful attention to security, gas optimization, and user experience. Start simple and build complexity gradually!

---

**Next**: [Your First Contract](your-first-contract.md) - Building and deploying your first smart contract
