# Increment Counter

The Increment Counter contract demonstrates how smart contracts can store and manage persistent state on the blockchain. This example builds on the Hello World contract by adding data persistence and state management capabilities.

## What This Example Teaches

- **State Management** - How to store data on the blockchain
- **Data Persistence** - State that survives between function calls
- **State Updates** - Modifying stored data safely
- **Access Control** - Controlling who can modify state
- **Event Emission** - Notifying when state changes

## Project Overview

This contract implements a simple counter that:
1. Starts at 0
2. Can be incremented by anyone
3. Can be reset by the owner
4. Maintains its value across transactions
5. Emits events when state changes

## Code Walkthrough

### **Complete Contract Code**

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, 
    Env, Symbol, Address, log
};

#[contract]
pub struct IncrementContract;

#[contractimpl]
impl IncrementContract {
    /// Initialize the contract with an owner
    pub fn initialize(env: Env, owner: Address) {
        env.storage().instance().set(&symbol_short!("OWNER"), &owner);
        env.storage().instance().set(&symbol_short!("COUNTER"), &0u32);
        
        log!(&env, "Counter initialized with owner: {}", owner);
    }
    
    /// Get the current counter value
    pub fn get_counter(env: Env) -> u32 {
        env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0)
    }
    
    /// Increment the counter by 1
    pub fn increment(env: Env) -> u32 {
        let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
        
        count += 1;
        env.storage().instance().set(&symbol_short!("COUNTER"), &count);
        
        // Emit event for transparency
        env.events().publish((
            symbol_short!("counter_incremented"),
            count
        ));
        
        log!(&env, "Counter incremented to: {}", count);
        count
    }
    
    /// Reset the counter to 0 (owner only)
    pub fn reset(env: Env, caller: Address) -> Result<(), Error> {
        let owner = env.storage().instance().get(&symbol_short!("OWNER"))
            .ok_or(Error::OwnerNotSet)?;
        
        if caller != owner {
            return Err(Error::Unauthorized);
        }
        
        env.storage().instance().set(&symbol_short!("COUNTER"), &0u32);
        
        // Emit event for transparency
        env.events().publish((
            symbol_short!("counter_reset"),
            caller
        ));
        
        log!(&env, "Counter reset by owner: {}", caller);
        Ok(())
    }
    
    /// Get the contract owner
    pub fn get_owner(env: Env) -> Address {
        env.storage().instance().get(&symbol_short!("OWNER"))
            .expect("Owner not set")
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    OwnerNotSet,
    Unauthorized,
}

impl From<Error> for soroban_sdk::Error {
    fn from(err: Error) -> Self {
        soroban_sdk::Error::from_type_and_code(err.into(), 0)
    }
}
```

### **Line-by-Line Explanation**

#### **Contract Structure**
```rust
#[contract]
pub struct IncrementContract;
```
- **Purpose**: Defines the contract structure
- **Note**: This is a unit struct (no fields) since all data is stored on-chain

#### **Constants and Storage Keys**
```rust
// Storage keys are defined as constants for efficiency
symbol_short!("OWNER")    // Stores the contract owner address
symbol_short!("COUNTER")  // Stores the current counter value
```

#### **Initialize Function**
```rust
pub fn initialize(env: Env, owner: Address) {
    env.storage().instance().set(&symbol_short!("OWNER"), &owner);
    env.storage().instance().set(&symbol_short!("COUNTER"), &0u32);
    
    log!(&env, "Counter initialized with owner: {}", owner);
}
```
- **Purpose**: Sets up initial contract state
- **Owner**: Sets who can reset the counter
- **Counter**: Initializes counter to 0
- **Logging**: Records initialization for debugging

#### **Get Counter Function**
```rust
pub fn get_counter(env: Env) -> u32 {
    env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0)
}
```
- **Purpose**: Read-only function to get current counter value
- **Return**: Current counter value or 0 if not set
- **Gas**: Very cheap since it only reads data

#### **Increment Function**
```rust
pub fn increment(env: Env) -> u32 {
    let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
    
    count += 1;
    env.storage().instance().set(&symbol_short!("COUNTER"), &count);
    
    // Emit event for transparency
    env.events().publish((
        symbol_short!("counter_incremented"),
        count
    ));
    
    log!(&env, "Counter incremented to: {}", count);
    count
}
```
- **Purpose**: Increments counter by 1
- **State Change**: Updates stored counter value
- **Event**: Emits event for transparency
- **Return**: New counter value

#### **Reset Function**
```rust
pub fn reset(env: Env, caller: Address) -> Result<(), Error> {
    let owner = env.storage().instance().get(&symbol_short!("OWNER"))
        .ok_or(Error::OwnerNotSet)?;
    
    if caller != owner {
        return Err(Error::Unauthorized);
    }
    
    env.storage().instance().set(&symbol_short!("COUNTER"), &0u32);
    
    // Emit event for transparency
    env.events().publish((
        symbol_short!("counter_reset"),
        caller
    ));
    
    log!(&env, "Counter reset by owner: {}", caller);
    Ok(())
}
```
- **Purpose**: Resets counter to 0 (owner only)
- **Access Control**: Only owner can call this function
- **Error Handling**: Returns Result for proper error handling
- **Event**: Emits reset event for transparency

#### **Error Handling**
```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    OwnerNotSet,
    Unauthorized,
}

impl From<Error> for soroban_sdk::Error {
    fn from(err: Error) -> Self {
        soroban_sdk::Error::from_type_and_code(err.into(), 0)
    }
}
```
- **Custom Errors**: Define specific error types
- **Error Conversion**: Convert to Soroban SDK errors
- **Debugging**: Errors include meaningful information

## Building and Testing

### **Step 1: Navigate to Project**
```bash
cd increment/contracts/increment
```

### **Step 2: Build Contract**
```bash
stellar contract build
```

**Expected Output:**
```
✅ Contract wasm built successfully
📁 Output: target/wasm32v1-none/release/increment.wasm
```

### **Step 3: Run Tests**
```bash
cargo test
```

**Expected Output:**
```
running 3 tests
test test::test_initialize ... ok
test test::test_increment ... ok
test test::test_reset ... ok

test result: ok. 3 tests passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
```

### **Step 4: Understanding Tests**

The test file (`src/test.rs`) contains comprehensive tests:

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Address, Env, Symbol};

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, IncrementContract);
        let client = IncrementContractClient::new(&env, &contract_id);
        
        let owner = Address::random(&env);
        client.initialize(&owner);
        
        assert_eq!(client.get_owner(), owner);
        assert_eq!(client.get_counter(), 0);
    }
    
    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, IncrementContract);
        let client = IncrementContractClient::new(&env, &contract_id);
        
        let owner = Address::random(&env);
        client.initialize(&owner);
        
        assert_eq!(client.increment(), 1);
        assert_eq!(client.get_counter(), 1);
        
        assert_eq!(client.increment(), 2);
        assert_eq!(client.get_counter(), 2);
    }
    
    #[test]
    fn test_reset() {
        let env = Env::default();
        let contract_id = env.register_contract(None, IncrementContract);
        let client = IncrementContractClient::new(&env, &contract_id);
        
        let owner = Address::random(&env);
        client.initialize(&owner);
        
        client.increment();
        client.increment();
        assert_eq!(client.get_counter(), 2);
        
        client.reset(&owner);
        assert_eq!(client.get_counter(), 0);
    }
}
```

## Deployment

### **Step 1: Deploy to Testnet**
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/increment.wasm \
  --source alice \
  --network testnet \
  --alias increment-counter
```

### **Step 2: Initialize Contract**
```bash
# Get your address for initialization
stellar keys address alice

# Initialize the contract (replace with your address)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  --send=yes \
  -- \
  initialize \
  --owner <YOUR_ADDRESS>
```

### **Step 3: Test Functions**
```bash
# Get current counter
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  get_counter

# Increment counter
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  --send=yes \
  -- \
  increment

# Check counter again
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  get_counter

# Reset counter (owner only)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  --send=yes \
  -- \
  reset \
  --caller <YOUR_ADDRESS>
```

## Understanding State Management

### **How State Persistence Works**

1. **Storage Types**: Uses `instance` storage for contract-specific data
2. **Key-Value Pairs**: Data stored as key-value pairs
3. **Persistence**: Data survives between function calls
4. **Global Access**: Anyone can read the state
5. **Controlled Writes**: Only authorized functions can modify state

### **State Flow Diagram**

```
Initialize → Set Owner & Counter → Contract Ready
    ↓
User Calls Increment → Read Current Value → Add 1 → Store New Value → Emit Event
    ↓
User Calls Get Counter → Read Stored Value → Return Value
    ↓
Owner Calls Reset → Verify Ownership → Set Counter to 0 → Emit Event
```

### **Storage Operations**

```rust
// Reading state
let value = env.storage().instance().get(&key).unwrap_or(default);

// Writing state
env.storage().instance().set(&key, &value);

// Checking if key exists
if env.storage().instance().has(&key) {
    // Key exists
}
```

## Advanced Features

### **1. Event Emission**

```rust
// Emit event when counter increments
env.events().publish((
    symbol_short!("counter_incremented"),
    count
));

// Emit event when counter resets
env.events().publish((
    symbol_short!("counter_reset"),
    caller
));
```

**Benefits of Events:**
- **Transparency**: All state changes are visible
- **Frontend Integration**: Frontends can listen for updates
- **Analytics**: Track contract usage patterns
- **Debugging**: Understand what happened when

### **2. Access Control**

```rust
pub fn reset(env: Env, caller: Address) -> Result<(), Error> {
    let owner = env.storage().instance().get(&symbol_short!("OWNER"))
        .ok_or(Error::OwnerNotSet)?;
    
    if caller != owner {
        return Err(Error::Unauthorized);
    }
    
    // Only owner can reset
    // ... reset logic ...
}
```

**Access Control Patterns:**
- **Owner-Only**: Single address has full control
- **Multi-Signature**: Multiple addresses must approve
- **Role-Based**: Different roles have different permissions
- **Time-Based**: Functions only available at certain times

### **3. Error Handling**

```rust
pub fn reset(env: Env, caller: Address) -> Result<(), Error> {
    let owner = env.storage().instance().get(&symbol_short!("OWNER"))
        .ok_or(Error::OwnerNotSet)?;  // Early return on error
    
    if caller != owner {
        return Err(Error::Unauthorized);  // Return error
    }
    
    // ... success logic ...
    Ok(())
}
```

**Error Handling Best Practices:**
- **Early Returns**: Check conditions first
- **Specific Errors**: Use meaningful error types
- **Error Conversion**: Convert to SDK errors
- **User Feedback**: Errors should be actionable

## Customization Ideas

### **1. Add Decrement Function**
```rust
pub fn decrement(env: Env) -> Result<u32, Error> {
    let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
    
    if count == 0 {
        return Err(Error::CannotDecrement);
    }
    
    count -= 1;
    env.storage().instance().set(&symbol_short!("COUNTER"), &count);
    
    Ok(count)
}
```

### **2. Add Maximum Value**
```rust
const MAX_COUNTER: u32 = 1000;

pub fn increment(env: Env) -> Result<u32, Error> {
    let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
    
    if count >= MAX_COUNTER {
        return Err(Error::CounterAtMaximum);
    }
    
    count += 1;
    env.storage().instance().set(&symbol_short!("COUNTER"), &count);
    
    Ok(count)
}
```

### **3. Add Multiple Counters**
```rust
pub fn increment_named(env: Env, name: Symbol) -> u32 {
    let key = (symbol_short!("NAMED_COUNTER"), name);
    let mut count: u32 = env.storage().instance().get(&key).unwrap_or(0);
    
    count += 1;
    env.storage().instance().set(&key, &count);
    
    count
}

pub fn get_named_counter(env: Env, name: Symbol) -> u32 {
    let key = (symbol_short!("NAMED_COUNTER"), name);
    env.storage().instance().get(&key).unwrap_or(0)
}
```

### **4. Add Counter History**
```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CounterEvent {
    pub timestamp: u64,
    pub action: String,
    pub value: u32,
}

pub fn increment_with_history(env: Env) -> u32 {
    let mut count: u32 = env.storage().instance().get(&symbol_short!("COUNTER")).unwrap_or(0);
    
    count += 1;
    env.storage().instance().set(&symbol_short!("COUNTER"), &count);
    
    // Store history
    let event = CounterEvent {
        timestamp: env.ledger().timestamp(),
        action: "increment".to_string(),
        value: count,
    };
    
    let history_key = (symbol_short!("HISTORY"), count);
    env.storage().instance().set(&history_key, &event);
    
    count
}
```

## Common Issues and Solutions

### **1. "Owner not set" Error**
```bash
# Error: Owner not set
# Solution: Initialize contract first
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet --send=yes -- initialize --owner <YOUR_ADDRESS>
```

### **2. "Unauthorized" Error**
```bash
# Error: Unauthorized
# Solution: Only owner can reset counter
# Use the address that was set during initialization
```

### **3. Counter Not Incrementing**
```bash
# Problem: Counter stays at 0
# Solution: Check if contract was initialized
stellar contract invoke --id <CONTRACT_ID> --source alice --network testnet -- get_owner
```

## What You've Learned

### **Core Concepts**
- ✅ **State Persistence**: Data survives between calls
- ✅ **Storage Operations**: Reading and writing to blockchain
- ✅ **Access Control**: Controlling who can modify state
- ✅ **Event Emission**: Notifying when state changes
- ✅ **Error Handling**: Proper error management

### **Technical Skills**
- ✅ **Storage Keys**: Using Symbols for efficient storage
- ✅ **State Updates**: Modifying stored data safely
- ✅ **Function Visibility**: Public vs. private functions
- ✅ **Input Validation**: Checking parameters and permissions
- ✅ **Logging**: Recording important events

### **Best Practices**
- ✅ **Initialize Pattern**: Set up contract state
- ✅ **Access Control**: Restrict sensitive operations
- ✅ **Event Logging**: Maintain transparency
- ✅ **Error Handling**: Graceful failure management
- ✅ **Gas Optimization**: Efficient storage operations

## Next Steps

### **Immediate Next Steps**
1. **Modify the Contract**: Add new functions or change behavior
2. **Test Variations**: Try different input values and edge cases
3. **Deploy and Test**: Deploy to testnet and interact with it

### **Learning Path**
1. **[Token Contract](token-contract.md)** - Learn about asset management
2. **[Contract State Management](smart-contracts/contract-state.md)** - Master advanced state patterns
3. **[Testing Strategies](smart-contracts/testing-contracts.md)** - Improve your testing skills

### **Advanced Topics**
- [Security Best Practices](advanced/security-best-practices.md)
- [Contract Optimization](advanced/contract-optimization.md)
- [Frontend Integration](frontend/README.md)

## Summary

Congratulations! You've successfully built a stateful smart contract that demonstrates:

- **Data Persistence**: Counter value survives between transactions
- **State Management**: Safe reading and writing of blockchain data
- **Access Control**: Owner-only reset functionality
- **Event Emission**: Transparent state change notifications
- **Error Handling**: Proper error management and user feedback

The Increment Counter contract may be simple, but it demonstrates all the fundamental patterns you'll use in every stateful smart contract you build. The concepts you've learned here—storage, state updates, access control, and events—are the building blocks of complex blockchain applications.

Remember: Every complex application starts with simple building blocks. You're now ready to build much more sophisticated smart contracts!

---

**Next**: [Token Contract](token-contract.md) - Learn about asset creation and management
