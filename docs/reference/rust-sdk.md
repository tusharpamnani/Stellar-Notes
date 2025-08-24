# Rust SDK Reference

Complete reference for the Soroban Rust SDK. This page provides comprehensive documentation for all SDK components, types, and functions used in smart contract development.

## SDK Overview

The Soroban SDK provides the core functionality needed to build smart contracts on the Stellar network. It includes types, storage operations, environment access, and utility functions.

## Core Types

### **Environment (Env)**

The `Env` type provides access to the contract's execution environment.

```rust
use soroban_sdk::Env;

pub fn example_function(env: Env) {
    // Access ledger information
    let timestamp = env.ledger().timestamp();
    let sequence = env.ledger().sequence();
    
    // Access storage
    let storage = env.storage();
    
    // Access events
    let events = env.events();
    
    // Access current contract
    let contract_id = env.current_contract();
}
```

### **Address**

Represents a Stellar account address.

```rust
use soroban_sdk::Address;

pub fn address_operations(env: &Env) {
    // Create random address for testing
    let random_address = Address::random(env);
    
    // Get current contract address
    let contract_address = env.current_contract();
    
    // Check if address is valid
    if contract_address != Address::default() {
        // Address is valid
    }
}
```

### **Symbol**

Efficient string representation for contract storage and events.

```rust
use soroban_sdk::{symbol_short, Symbol};

pub fn symbol_operations(env: &Env) {
    // Create symbol using macro (recommended)
    let key = symbol_short!("USER_BALANCE");
    
    // Create symbol from string
    let custom_key = Symbol::new(env, "custom_key");
    
    // Convert to string
    let key_string = key.to_string();
}
```

### **Vec**

Dynamic array type for contract data.

```rust
use soroban_sdk::Vec;

pub fn vector_operations(env: &Env) {
    // Create empty vector
    let mut vec = Vec::new(env);
    
    // Add elements
    vec.push_back(1);
    vec.push_back(2);
    vec.push_back(3);
    
    // Access elements
    let first = vec.get(0).unwrap();
    
    // Iterate
    for i in 0..vec.len() {
        let element = vec.get(i).unwrap();
        // Process element
    }
}
```

### **Map**

Key-value storage for contract data.

```rust
use soroban_sdk::Map;

pub fn map_operations(env: &Env) {
    // Create empty map
    let mut map = Map::new(env);
    
    // Insert key-value pairs
    map.set(&1, &"value1");
    map.set(&2, &"value2");
    
    // Get values
    let value1 = map.get(&1);
    let value2 = map.get(&2);
    
    // Check if key exists
    if map.contains_key(&1) {
        // Key exists
    }
    
    // Remove key
    map.remove(&1);
}
```

## Storage Operations

### **Instance Storage**

Contract-specific persistent storage.

```rust
use soroban_sdk::symbol_short;

pub fn instance_storage_operations(env: &Env) {
    // Set value
    env.storage().instance().set(&symbol_short!("KEY"), &"value");
    
    // Get value
    let value = env.storage().instance().get(&symbol_short!("KEY"));
    
    // Get with default
    let value_with_default = env.storage().instance().get(&symbol_short!("KEY")).unwrap_or("default");
    
    // Check if key exists
    let exists = env.storage().instance().has(&symbol_short!("KEY"));
    
    // Remove key
    env.storage().instance().remove(&symbol_short!("KEY"));
    
    // Extend TTL
    env.storage().instance().extend_ttl(50, 100);
}
```

### **Temporary Storage**

Short-lived storage for transaction duration.

```rust
pub fn temporary_storage_operations(env: &Env) {
    // Set temporary value
    env.storage().temporary().set(&symbol_short!("TEMP_KEY"), &"temp_value");
    
    // Get temporary value
    let temp_value = env.storage().temporary().get(&symbol_short!("TEMP_KEY"));
    
    // Check if temporary key exists
    let exists = env.storage().temporary().has(&symbol_short!("TEMP_KEY"));
}
```

### **Persistent Storage**

Long-term storage across contract instances.

```rust
pub fn persistent_storage_operations(env: &Env) {
    // Set persistent value
    env.storage().persistent().set(&symbol_short!("PERSISTENT_KEY"), &"persistent_value");
    
    // Get persistent value
    let value = env.storage().persistent().get(&symbol_short!("PERSISTENT_KEY"));
    
    // Check if persistent key exists
    let exists = env.storage().persistent().has(&symbol_short!("PERSISTENT_KEY"));
}
```

## Environment Access

### **Ledger Information**

Access to blockchain ledger data.

```rust
pub fn ledger_operations(env: &Env) {
    // Get current timestamp
    let timestamp = env.ledger().timestamp();
    
    // Get current sequence number
    let sequence = env.ledger().sequence();
    
    // Get network passphrase
    let passphrase = env.ledger().network_passphrase();
    
    // Get base reserve
    let base_reserve = env.ledger().base_reserve();
}
```

### **Contract Information**

Access to contract-specific data.

```rust
pub fn contract_operations(env: &Env) {
    // Get current contract ID
    let contract_id = env.current_contract();
    
    // Get contract address
    let contract_address = env.current_contract_address();
    
    // Get invocation count
    let invocation_count = env.invocation_count();
}
```

### **Random Number Generation**

Cryptographically secure random numbers.

```rust
pub fn random_operations(env: &Env) {
    // Generate random u32
    let random_u32 = env.prng().u32_in_range(0..100);
    
    // Generate random u64
    let random_u64 = env.prng().u64_in_range(0..1000);
    
    // Generate random bytes
    let random_bytes = env.prng().bytes(32);
}
```

## Event System

### **Publishing Events**

Emit events for transparency and frontend integration.

```rust
pub fn event_operations(env: &Env) {
    // Publish simple event
    env.events().publish((
        symbol_short!("user_created"),
        "user_id"
    ));
    
    // Publish complex event
    env.events().publish((
        symbol_short!("transfer"),
        TransferEvent {
            from: Address::default(),
            to: Address::default(),
            amount: 1000,
        }
    ));
    
    // Publish multiple events
    env.events().publish((
        symbol_short!("batch_operation"),
        "operation_type",
        "result"
    ));
}
```

### **Event Structures**

Define structured events for better organization.

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub amount: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserEvent {
    pub user_id: Address,
    pub action: Symbol,
    pub timestamp: u64,
}

pub fn structured_events(env: &Env) {
    let transfer_event = TransferEvent {
        from: Address::default(),
        to: Address::default(),
        amount: 1000,
    };
    
    env.events().publish((
        symbol_short!("transfer"),
        transfer_event
    ));
    
    let user_event = UserEvent {
        user_id: Address::default(),
        action: symbol_short!("login"),
        timestamp: env.ledger().timestamp(),
    };
    
    env.events().publish((
        symbol_short!("user_action"),
        user_event
    ));
}
```

## Error Handling

### **Custom Error Types**

Define specific error types for your contract.

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractError {
    InsufficientBalance,
    UserNotFound,
    InvalidAmount,
    Unauthorized,
    ContractPaused,
    InvalidInput,
}

impl From<ContractError> for soroban_sdk::Error {
    fn from(err: ContractError) -> Self {
        match err {
            ContractError::InsufficientBalance => {
                soroban_sdk::Error::from_type_and_code(1, 0)
            }
            ContractError::UserNotFound => {
                soroban_sdk::Error::from_type_and_code(2, 0)
            }
            ContractError::InvalidAmount => {
                soroban_sdk::Error::from_type_and_code(3, 0)
            }
            ContractError::Unauthorized => {
                soroban_sdk::Error::from_type_and_code(4, 0)
            }
            ContractError::ContractPaused => {
                soroban_sdk::Error::from_type_and_code(5, 0)
            }
            ContractError::InvalidInput => {
                soroban_sdk::Error::from_type_and_code(6, 0)
            }
        }
    }
}
```

### **Error Handling Patterns**

Common error handling approaches.

```rust
pub fn error_handling_examples(env: &Env) -> Result<(), ContractError> {
    // Early return pattern
    let user_balance = env.storage().instance()
        .get(&symbol_short!("USER_BALANCE"))
        .ok_or(ContractError::UserNotFound)?;
    
    // Validation with custom error
    if user_balance < 100 {
        return Err(ContractError::InsufficientBalance);
    }
    
    // Result chaining
    let result = env.storage().instance()
        .get(&symbol_short!("USER_DATA"))
        .ok_or(ContractError::UserNotFound)
        .and_then(|data| {
            if data.is_active {
                Ok(data)
            } else {
                Err(ContractError::UserNotFound)
            }
        })?;
    
    Ok(())
}
```

## Utility Functions

### **Logging**

Debug and informational logging.

```rust
use soroban_sdk::log;

pub fn logging_examples(env: &Env) {
    // Basic logging
    log!(&env, "Function called with parameter: {}", 42);
    
    // Multiple parameters
    log!(&env, "User {} has balance {}", "alice", 1000);
    
    // Conditional logging
    if env.ledger().timestamp() > 1000000 {
        log!(&env, "High timestamp detected: {}", env.ledger().timestamp());
    }
    
    // Error logging
    log!(&env, "Error occurred: {}", "insufficient balance");
}
```

### **Data Conversion**

Convert between different data types.

```rust
use soroban_sdk::{IntoVal, TryFromVal};

pub fn conversion_examples(env: &Env) {
    // Convert to contract value
    let value: soroban_sdk::Val = 42u64.into_val(env);
    let address_val: soroban_sdk::Val = Address::default().into_val(env);
    
    // Convert from contract value
    let number: u64 = value.try_from_val(env, &value).unwrap();
    let address: Address = address_val.try_from_val(env, &address_val).unwrap();
    
    // Safe conversion with default
    let safe_number: u64 = value.try_from_val(env, &value).unwrap_or(0);
}
```

### **Validation Functions**

Common validation utilities.

```rust
pub fn validation_examples(env: &Env) -> Result<(), ContractError> {
    // Validate address
    let user_address = Address::default();
    if user_address == Address::default() {
        return Err(ContractError::InvalidInput);
    }
    
    // Validate amount
    let amount = 1000u64;
    if amount == 0 {
        return Err(ContractError::InvalidAmount);
    }
    
    // Validate string length
    let input = "test";
    if input.len() < 3 || input.len() > 100 {
        return Err(ContractError::InvalidInput);
    }
    
    // Validate numeric range
    let value = 50u64;
    if value < 1 || value > 1000 {
        return Err(ContractError::InvalidInput);
    }
    
    Ok(())
}
```

## Advanced Patterns

### **Batch Operations**

Efficiently process multiple operations.

```rust
pub fn batch_operations(env: &Env, operations: &[u64]) -> Result<Vec<u64>, ContractError> {
    let mut results = Vec::new(env);
    
    // Pre-allocate capacity
    results.reserve(operations.len());
    
    // Process all operations
    for &operation in operations {
        let result = process_operation(env, operation)?;
        results.push_back(result);
    }
    
    Ok(results)
}

fn process_operation(env: &Env, operation: u64) -> Result<u64, ContractError> {
    // Process individual operation
    Ok(operation * 2)
}
```

### **Caching Patterns**

Optimize repeated operations.

```rust
pub fn caching_example(env: &Env, user_id: &Address) -> Result<u64, ContractError> {
    // Check cache first
    let cache_key = (symbol_short!("CACHE"), user_id);
    if let Some(cached_value) = env.storage().instance().get(&cache_key) {
        return Ok(cached_value);
    }
    
    // Calculate value if not cached
    let calculated_value = calculate_expensive_operation(env, user_id)?;
    
    // Cache the result
    env.storage().instance().set(&cache_key, &calculated_value);
    
    Ok(calculated_value)
}

fn calculate_expensive_operation(env: &Env, user_id: &Address) -> Result<u64, ContractError> {
    // Simulate expensive operation
    let mut result = 0u64;
    for i in 0..1000 {
        result += i;
    }
    Ok(result)
}
```

### **State Machine Patterns**

Implement complex state transitions.

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ContractState {
    Initialized,
    Active,
    Paused,
    Terminated,
}

pub fn state_machine_example(env: &Env, new_state: ContractState) -> Result<(), ContractError> {
    let current_state = env.storage().instance()
        .get(&symbol_short!("STATE"))
        .unwrap_or(ContractState::Initialized);
    
    // Validate state transition
    match (current_state, new_state) {
        (ContractState::Initialized, ContractState::Active) => {
            // Valid transition
        }
        (ContractState::Active, ContractState::Paused) => {
            // Valid transition
        }
        (ContractState::Paused, ContractState::Active) => {
            // Valid transition
        }
        (ContractState::Active, ContractState::Terminated) => {
            // Valid transition
        }
        _ => {
            return Err(ContractError::InvalidInput);
        }
    }
    
    // Update state
    env.storage().instance().set(&symbol_short!("STATE"), &new_state);
    
    // Emit state change event
    env.events().publish((
        symbol_short!("state_changed"),
        current_state,
        new_state
    ));
    
    Ok(())
}
```

## Testing Utilities

### **Test Environment Setup**

Configure test environment for contract testing.

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Address, Env, Symbol};

    fn setup_test_env() -> (Env, Address) {
        let env = Env::default();
        let contract_id = env.register_contract(None, TestContract);
        (env, contract_id)
    }

    #[test]
    fn test_basic_functionality() {
        let (env, contract_id) = setup_test_env();
        let client = TestContractClient::new(&env, &contract_id);
        
        // Test implementation
        let result = client.test_function();
        assert!(result.is_ok());
    }
}
```

### **Mock Data Generation**

Create test data for comprehensive testing.

```rust
#[cfg(test)]
mod test_utils {
    use super::*;
    use soroban_sdk::{Address, Env, Symbol};

    pub fn create_test_user(env: &Env) -> Address {
        Address::random(env)
    }
    
    pub fn create_test_users(env: &Env, count: usize) -> Vec<Address> {
        (0..count).map(|_| Address::random(env)).collect()
    }
    
    pub fn create_test_symbol(env: &Env, value: &str) -> Symbol {
        Symbol::new(env, value)
    }
    
    pub fn create_test_data(env: &Env) -> TestData {
        TestData {
            user_id: Address::random(env),
            balance: 1000,
            is_active: true,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestData {
    pub user_id: Address,
    pub balance: u64,
    pub is_active: bool,
}
```

## Best Practices

### **Gas Optimization**

Write gas-efficient contracts.

```rust
pub fn gas_optimized_function(env: &Env) -> Result<(), ContractError> {
    // Use efficient storage keys
    let key = symbol_short!("KEY");
    
    // Batch storage operations
    let mut batch_data = Vec::new(env);
    for i in 0..10 {
        batch_data.push_back(i);
    }
    
    // Single storage operation instead of multiple
    env.storage().instance().set(&key, &batch_data);
    
    // Use early returns to avoid unnecessary computation
    if env.ledger().timestamp() > 1000000 {
        return Ok(());
    }
    
    Ok(())
}
```

### **Security Considerations**

Implement secure contract patterns.

```rust
pub fn secure_function(env: &Env, caller: &Address) -> Result<(), ContractError> {
    // Validate caller
    if caller == &Address::default() {
        return Err(ContractError::InvalidInput);
    }
    
    // Check authorization
    if !is_authorized(env, caller)? {
        return Err(ContractError::Unauthorized);
    }
    
    // Validate inputs
    let input = env.storage().instance().get(&symbol_short!("INPUT"));
    if let Some(input_val) = input {
        if !is_valid_input(input_val) {
            return Err(ContractError::InvalidInput);
        }
    }
    
    // Perform operation
    perform_secure_operation(env)?;
    
    Ok(())
}

fn is_authorized(env: &Env, caller: &Address) -> Result<bool, ContractError> {
    let auth_key = (symbol_short!("AUTH"), caller);
    Ok(env.storage().instance().get(&auth_key).unwrap_or(false))
}

fn is_valid_input(input: &str) -> bool {
    input.len() > 0 && input.len() <= 100
}

fn perform_secure_operation(env: &Env) -> Result<(), ContractError> {
    // Secure operation implementation
    Ok(())
}
```

## What You've Learned

### **SDK Fundamentals**
- ✅ **Core Types** - Environment, Address, Symbol, Vec, Map
- ✅ **Storage Operations** - Instance, temporary, and persistent storage
- ✅ **Environment Access** - Ledger, contract, and random number access
- ✅ **Event System** - Publishing structured events
- ✅ **Error Handling** - Custom error types and handling patterns

### **Advanced Features**
- ✅ **Utility Functions** - Logging, conversion, and validation
- ✅ **Design Patterns** - Batch operations, caching, and state machines
- ✅ **Testing Support** - Test environment setup and utilities
- ✅ **Best Practices** - Gas optimization and security considerations

### **Production Ready**
- ✅ **Comprehensive Coverage** - All major SDK functionality
- ✅ **Practical Examples** - Real-world usage patterns
- ✅ **Performance Tips** - Gas optimization techniques
- ✅ **Security Patterns** - Secure contract development

## Next Steps

### **Immediate Actions**
1. **Explore SDK Features** - Try different SDK components
2. **Build Examples** - Create contracts using various SDK features
3. **Optimize Usage** - Apply gas optimization techniques
4. **Test Thoroughly** - Use testing utilities for validation

### **Advanced Usage**
1. **[Network Configuration](network-config.md)** - Configure different networks
2. **[CLI Commands](cli-commands.md)** - Deploy and interact with contracts
3. **[Advanced Topics](advanced/README.md)** - Security and optimization
4. **Community Resources** - SDK updates and best practices

### **Resources**
- **Official Documentation** - [soroban.stellar.org/docs](https://soroban.stellar.org/docs)
- **GitHub Repository** - [github.com/stellar/soroban](https://github.com/stellar/soroban)
- **Community Discord** - [discord.gg/stellar](https://discord.gg/stellar)
- **Examples Repository** - [github.com/stellar/soroban-examples](https://github.com/stellar/soroban-examples)

## Summary

The Soroban Rust SDK provides a comprehensive toolkit for building smart contracts on the Stellar network. From basic types and storage operations to advanced patterns and testing utilities, the SDK covers all aspects of smart contract development.

Remember: The SDK is designed to be both powerful and user-friendly. Start with the basics and gradually explore advanced features as your contracts become more complex.

The best way to learn the SDK is through practice. Build contracts, experiment with different features, and always test thoroughly!

---

**Next**: [Network Configuration](network-config.md) - Network settings and endpoints
