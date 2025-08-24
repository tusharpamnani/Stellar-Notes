# Contract State Management

State management is the heart of smart contract functionality. This section will teach you how to store, retrieve, and manage data on the blockchain efficiently and securely.

## What is Contract State?

Contract state refers to the persistent data that your smart contract stores on the blockchain. This data persists between function calls and can be read by anyone, making it the foundation for building interactive and data-driven applications.

### **Key Characteristics**

- **Persistent**: Data survives between transactions
- **Public**: Anyone can read the state (though you control who can write)
- **Immutable**: Once written, data cannot be changed (only overwritten)
- **Costly**: Storage operations consume gas and cost money
- **Global**: State is accessible from anywhere on the network

## Storage Types in Soroban

### **1. Instance Storage**

Instance storage is the most common type, storing data specific to a single contract instance.

```rust
use soroban_sdk::{contract, contractimpl, symbol_short, Env, Symbol, Address};

const COUNTER: Symbol = symbol_short!("COUNTER");
const OWNER: Symbol = symbol_short!("OWNER");
const USER_BALANCES: Symbol = symbol_short!("BALANCES");

#[contract]
pub struct StateContract;

#[contractimpl]
impl StateContract {
    pub fn initialize(env: Env, owner: Address) {
        env.storage().instance().set(&OWNER, &owner);
        env.storage().instance().set(&COUNTER, &0u32);
    }
    
    pub fn get_counter(env: Env) -> u32 {
        env.storage().instance().get(&COUNTER).unwrap_or(0)
    }
    
    pub fn increment(env: Env) -> u32 {
        let current = env.storage().instance().get(&COUNTER).unwrap_or(0);
        let new_value = current + 1;
        env.storage().instance().set(&COUNTER, &new_value);
        new_value
    }
}
```

**When to use Instance Storage:**
- Contract-specific configuration
- Global counters and totals
- Owner/admin addresses
- Contract metadata

### **2. Persistent Storage**

Persistent storage survives contract upgrades and is useful for critical data that should never be lost.

```rust
const USER_PROFILES: Symbol = symbol_short!("PROFILES");
const CONTRACT_SETTINGS: Symbol = symbol_short!("SETTINGS");

pub fn store_user_profile(env: Env, user: Address, profile: String) {
    let key = (USER_PROFILES, user);
    env.storage().persistent().set(&key, &profile);
}

pub fn get_user_profile(env: Env, user: Address) -> Option<String> {
    let key = (USER_PROFILES, user);
    env.storage().persistent().get(&key)
}

pub fn store_setting(env: Env, setting_name: Symbol, value: String) {
    env.storage().persistent().set(&setting_name, &value);
}
```

**When to use Persistent Storage:**
- User data that should survive upgrades
- Critical configuration settings
- Historical records
- Upgradeable contract data

### **3. Temporary Storage**

Temporary storage is used for data that only needs to persist for the duration of a single transaction.

```rust
const TEMP_CALCULATION: Symbol = symbol_short!("TEMP_CALC");

pub fn complex_calculation(env: Env, input: u32) -> u32 {
    // Store intermediate result
    let intermediate = input * 2;
    env.storage().temporary().set(&TEMP_CALCULATION, &intermediate);
    
    // Use the stored value
    let stored = env.storage().temporary().get(&TEMP_CALCULATION).unwrap();
    stored + 10
}
```

**When to use Temporary Storage:**
- Intermediate calculations
- Transaction-scoped data
- Temporary state during complex operations

## Data Structures and Patterns

### **1. Simple Key-Value Storage**

```rust
const USER_BALANCE: Symbol = symbol_short!("BALANCE");
const USER_NAME: Symbol = symbol_short!("NAME");

pub fn set_user_balance(env: Env, user: Address, balance: u32) {
    env.storage().instance().set(&(USER_BALANCE, user), &balance);
}

pub fn get_user_balance(env: Env, user: Address) -> u32 {
    env.storage().instance().get(&(USER_BALANCE, user)).unwrap_or(0)
}

pub fn set_user_name(env: Env, user: Address, name: String) {
    env.storage().instance().set(&(USER_NAME, user), &name);
}
```

### **2. Complex Data Storage**

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserProfile {
    pub name: String,
    pub email: String,
    pub balance: u32,
    pub created_at: u64,
}

const USER_PROFILE: Symbol = symbol_short!("PROFILE");

pub fn store_user_profile(env: Env, user: Address, profile: UserProfile) {
    env.storage().instance().set(&(USER_PROFILE, user), &profile);
}

pub fn get_user_profile(env: Env, user: Address) -> Option<UserProfile> {
    env.storage().instance().get(&(USER_PROFILE, user))
}
```

### **3. Mapping Patterns**

```rust
// User to balance mapping
const BALANCES: Symbol = symbol_short!("BALANCES");

// User to permissions mapping
const PERMISSIONS: Symbol = symbol_short!("PERMS");

// Contract to user mapping
const CONTRACT_USERS: Symbol = symbol_short!("CONTRACT_USERS");

pub fn set_balance(env: Env, user: Address, balance: u32) {
    env.storage().instance().set(&(BALANCES, user), &balance);
}

pub fn set_permission(env: Env, user: Address, permission: String) {
    env.storage().instance().set(&(PERMISSIONS, user), &permission);
}

pub fn add_contract_user(env: Env, contract: Address, user: Address) {
    let key = (CONTRACT_USERS, contract, user);
    env.storage().instance().set(&key, &true);
}
```

## State Management Best Practices

### **1. Efficient Storage Keys**

```rust
// ✅ Good - Use Symbols for efficiency
const USER_BALANCE: Symbol = symbol_short!("BALANCE");
const USER_NAME: Symbol = symbol_short!("NAME");

// ❌ Avoid - Don't use strings directly
const USER_BALANCE: &str = "USER_BALANCE";
const USER_NAME: &str = "USER_NAME";
```

### **2. Batch Operations**

```rust
pub fn batch_update_balances(env: Env, updates: Vec<(Address, u32)>) -> Result<(), Error> {
    for (user, balance) in updates {
        if balance > MAX_BALANCE {
            return Err(Error::BalanceTooHigh);
        }
        env.storage().instance().set(&(USER_BALANCE, user), &balance);
    }
    Ok(())
}
```

### **3. Default Values**

```rust
pub fn get_user_balance(env: Env, user: Address) -> u32 {
    // Provide sensible defaults
    env.storage().instance().get(&(USER_BALANCE, user)).unwrap_or(0)
}

pub fn get_user_name(env: Env, user: Address) -> Option<String> {
    // Return None if no name is set
    env.storage().instance().get(&(USER_NAME, user))
}
```

### **4. State Validation**

```rust
pub fn set_user_balance(env: Env, user: Address, balance: u32) -> Result<(), Error> {
    // Validate input
    if balance > MAX_BALANCE {
        return Err(Error::BalanceTooHigh);
    }
    
    // Check current state
    let current = env.storage().instance().get(&(USER_BALANCE, user)).unwrap_or(0);
    if current + balance > MAX_BALANCE {
        return Err(Error::BalanceExceedsLimit);
    }
    
    // Update state
    env.storage().instance().set(&(USER_BALANCE, user), &(current + balance));
    Ok(())
}
```

## Advanced State Patterns

### **1. State Machines**

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OrderStatus {
    Pending,
    Confirmed,
    Shipped,
    Delivered,
    Cancelled,
}

const ORDER_STATUS: Symbol = symbol_short!("ORDER_STATUS");

pub fn update_order_status(env: Env, order_id: u32, new_status: OrderStatus) -> Result<(), Error> {
    let current_status = env.storage().instance().get(&(ORDER_STATUS, order_id))
        .ok_or(Error::OrderNotFound)?;
    
    // Validate state transition
    match (current_status, new_status) {
        (OrderStatus::Pending, OrderStatus::Confirmed) => {},
        (OrderStatus::Confirmed, OrderStatus::Shipped) => {},
        (OrderStatus::Shipped, OrderStatus::Delivered) => {},
        (OrderStatus::Pending, OrderStatus::Cancelled) => {},
        _ => return Err(Error::InvalidStatusTransition),
    }
    
    env.storage().instance().set(&(ORDER_STATUS, order_id), &new_status);
    Ok(())
}
```

### **2. Access Control Patterns**

```rust
const OWNER: Symbol = symbol_short!("OWNER");
const OPERATORS: Symbol = symbol_short!("OPERATORS");

pub fn add_operator(env: Env, caller: Address, operator: Address) -> Result<(), Error> {
    let owner = env.storage().instance().get(&OWNER)
        .ok_or(Error::OwnerNotSet)?;
    
    if caller != owner {
        return Err(Error::Unauthorized);
    }
    
    env.storage().instance().set(&(OPERATORS, operator), &true);
    Ok(())
}

pub fn is_operator(env: Env, user: Address) -> bool {
    env.storage().instance().get(&(OPERATORS, user)).unwrap_or(false)
}
```

### **3. State Events**

```rust
pub fn transfer_balance(env: Env, from: Address, to: Address, amount: u32) -> Result<(), Error> {
    // ... transfer logic ...
    
    // Emit state change event
    env.events().publish((
        symbol_short!("balance_transferred"),
        (from, to, amount)
    ));
    
    Ok(())
}
```

## Gas Optimization

### **1. Minimize Storage Operations**

```rust
// ❌ Expensive - Multiple storage operations
pub fn expensive_update(env: Env, user: Address, field1: String, field2: String) {
    env.storage().instance().set(&(USER_FIELD1, user), &field1);
    env.storage().instance().set(&(USER_FIELD2, user), &field2);
}

// ✅ Efficient - Single storage operation
pub fn efficient_update(env: Env, user: Address, field1: String, field2: String) {
    let user_data = UserData { field1, field2 };
    env.storage().instance().set(&(USER_DATA, user), &user_data);
}
```

### **2. Use Appropriate Data Types**

```rust
// Use u32 instead of u64 when possible
const COUNTER: Symbol = symbol_short!("COUNTER");

// Use Symbols instead of Strings for keys
const USER_BALANCE: Symbol = symbol_short!("BALANCE");
```

## Common Pitfalls

### **1. State Inconsistency**

```rust
// ❌ Dangerous - Can leave state inconsistent
pub fn unsafe_transfer(env: Env, from: Address, to: Address, amount: u32) -> Result<(), Error> {
    let from_balance = get_balance(&env, &from);
    if from_balance < amount {
        return Err(Error::InsufficientBalance);
    }
    
    // What if this fails?
    env.storage().instance().set(&(USER_BALANCE, from), &(from_balance - amount));
    
    // This might not execute, leaving state inconsistent
    let to_balance = get_balance(&env, &to);
    env.storage().instance().set(&(USER_BALANCE, to), &(to_balance + amount));
    
    Ok(())
}
```

### **2. Missing Validation**

```rust
// ❌ Dangerous - No validation
pub fn unsafe_set_balance(env: Env, user: Address, balance: u32) {
    env.storage().instance().set(&(USER_BALANCE, user), &balance);
}

// ✅ Safe - With validation
pub fn safe_set_balance(env: Env, user: Address, balance: u32) -> Result<(), Error> {
    if balance > MAX_BALANCE {
        return Err(Error::BalanceTooHigh);
    }
    
    env.storage().instance().set(&(USER_BALANCE, user), &balance);
    Ok(())
}
```

## Testing State Management

### **1. State Isolation**

```rust
#[test]
fn test_state_isolation() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StateContract);
    let client = StateContractClient::new(&env, &contract_id);
    
    // Test initial state
    assert_eq!(client.get_counter(), 0);
    
    // Test state changes
    assert_eq!(client.increment(), 1);
    assert_eq!(client.get_counter(), 1);
    
    // Test persistence
    assert_eq!(client.increment(), 2);
    assert_eq!(client.get_counter(), 2);
}
```

### **2. Edge Cases**

```rust
#[test]
fn test_edge_cases() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StateContract);
    let client = StateContractClient::new(&env, &contract_id);
    
    // Test with zero values
    client.set_balance(&Address::random(&env), &0);
    
    // Test with maximum values
    client.set_balance(&Address::random(&env), &u32::MAX);
    
    // Test with non-existent users
    let random_user = Address::random(&env);
    assert_eq!(client.get_balance(&random_user), 0);
}
```

## What's Next?

Now that you understand state management, you're ready to:

1. **[Test your contracts](testing-contracts.md)** - Ensure state changes work correctly
2. **[Deploy to networks](deploying-contracts.md)** - Get your stateful contracts running
3. **[Build frontend applications](frontend/README.md)** - Interact with contract state
4. **[Explore advanced patterns](advanced/README.md)** - Master complex state management

Remember: Good state management is the foundation of reliable smart contracts. Design your state carefully, validate everything, and test thoroughly!

---

**Next**: [Testing Contracts](testing-contracts.md) - Ensuring your contracts work correctly
