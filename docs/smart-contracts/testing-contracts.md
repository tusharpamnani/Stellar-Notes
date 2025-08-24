# Testing Contracts

Testing is crucial for smart contract development. Since deployed contracts are immutable and handle real value, thorough testing is your best defense against bugs and vulnerabilities. This section will teach you comprehensive testing strategies for Soroban contracts.

## Why Testing Matters

### **Smart Contract Risks**
- **Immutable Code**: Once deployed, bugs cannot be fixed
- **Real Value**: Contracts often handle real money and assets
- **Public Code**: Anyone can examine and exploit vulnerabilities
- **Network Effects**: Bugs can affect many users simultaneously

### **Testing Benefits**
- **Catch Bugs Early**: Find issues before deployment
- **Ensure Correctness**: Verify business logic works as intended
- **Document Behavior**: Tests serve as living documentation
- **Confidence**: Deploy with peace of mind
- **Cost Savings**: Avoid expensive bugs in production

## Testing Strategy

### **Testing Pyramid**

```
    /\
   /  \     E2E Tests (Few)
  /____\    Integration Tests (Some)
 /______\   Unit Tests (Many)
```

1. **Unit Tests**: Test individual functions in isolation
2. **Integration Tests**: Test contract interactions
3. **End-to-End Tests**: Test complete workflows

## Unit Testing

### **Basic Test Structure**

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Address, Env, Symbol};

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, CounterContract);
        let client = CounterContractClient::new(&env, &contract_id);
        
        // Test initial state
        assert_eq!(client.get_counter(), 0);
        
        // Test increment
        assert_eq!(client.increment(), 1);
        assert_eq!(client.get_counter(), 1);
        
        // Test multiple increments
        assert_eq!(client.increment(), 2);
        assert_eq!(client.increment(), 3);
        assert_eq!(client.get_counter(), 3);
    }
}
```

### **Test Environment Setup**

```rust
#[test]
fn test_with_environment() {
    // Create test environment
    let env = Env::default();
    
    // Register contract
    let contract_id = env.register_contract(None, MyContract);
    
    // Create client for easy interaction
    let client = MyContractClient::new(&env, &contract_id);
    
    // Test your contract
    let result = client.my_function(&"test".into());
    assert_eq!(result, "expected result");
}
```

### **Testing State Changes**

```rust
#[test]
fn test_state_persistence() {
    let env = Env::default();
    let contract_id = env.register_contract(None, StateContract);
    let client = StateContractClient::new(&env, &contract_id);
    
    let user = Address::random(&env);
    
    // Test initial state
    assert_eq!(client.get_balance(&user), 0);
    
    // Change state
    client.set_balance(&user, &100);
    
    // Verify state change
    assert_eq!(client.get_balance(&user), 100);
    
    // Test state persistence across calls
    assert_eq!(client.get_balance(&user), 100);
}
```

## Advanced Testing Patterns

### **1. Test Data Generation**

```rust
#[test]
fn test_with_multiple_users() {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenContract);
    let client = TokenContractClient::new(&env, &contract_id);
    
    // Generate test users
    let users: Vec<Address> = (0..5).map(|_| Address::random(&env)).collect();
    
    // Test with each user
    for user in &users {
        client.mint(&user, &100);
        assert_eq!(client.balance(&user), 100);
    }
}
```

### **2. Edge Case Testing**

```rust
#[test]
fn test_edge_cases() {
    let env = Env::default();
    let contract_id = env.register_contract(None, MathContract);
    let client = MathContractClient::new(&env, &contract_id);
    
    // Test zero values
    assert_eq!(client.add(&0, &0), 0);
    
    // Test maximum values
    assert_eq!(client.add(&u32::MAX, &0), u32::MAX);
    
    // Test overflow protection
    let result = client.checked_add(&u32::MAX, &1);
    assert!(result.is_err());
}
```

### **3. Error Testing**

```rust
#[test]
fn test_error_conditions() {
    let env = Env::default();
    let contract_id = env.register_contract(None, AccessControlContract);
    let client = AccessControlContractClient::new(&env, &contract_id);
    
    let user = Address::random(&env);
    let admin = Address::random(&env);
    
    // Test unauthorized access
    let result = client.admin_only_function(&user);
    assert!(result.is_err());
    
    // Test with admin
    client.set_admin(&admin);
    let result = client.admin_only_function(&admin);
    assert!(result.is_ok());
}
```

## Integration Testing

### **Contract Interaction Testing**

```rust
#[test]
fn test_contract_interaction() {
    let env = Env::default();
    
    // Deploy multiple contracts
    let token_id = env.register_contract(None, TokenContract);
    let exchange_id = env.register_contract(None, ExchangeContract);
    
    let token_client = TokenContractClient::new(&env, &token_id);
    let exchange_client = ExchangeContractClient::new(&env, &exchange_id);
    
    let user = Address::random(&env);
    
    // Setup: mint tokens
    token_client.mint(&user, &1000);
    
    // Test: exchange tokens
    exchange_client.swap_tokens(&user, &token_id, &500);
    
    // Verify: check balances
    assert_eq!(token_client.balance(&user), 500);
}
```

### **Cross-Contract Calls**

```rust
#[test]
fn test_cross_contract_calls() {
    let env = Env::default();
    
    let factory_id = env.register_contract(None, FactoryContract);
    let factory_client = FactoryContractClient::new(&env, &factory_id);
    
    let user = Address::random(&env);
    
    // Create new contract through factory
    let new_contract_id = factory_client.create_contract(&user);
    
    // Verify contract was created
    assert!(env.contracts().has(&new_contract_id));
    
    // Test interaction with new contract
    let new_client = NewContractClient::new(&env, &new_contract_id);
    let result = new_client.initialize(&user);
    assert!(result.is_ok());
}
```

## Property-Based Testing

### **Using QuickCheck for Property Testing**

```rust
use quickcheck::{Arbitrary, Gen};

#[derive(Clone, Debug)]
struct TransferOperation {
    from: Address,
    to: Address,
    amount: u32,
}

impl Arbitrary for TransferOperation {
    fn arbitrary(g: &mut Gen) -> Self {
        TransferOperation {
            from: Address::arbitrary(g),
            to: Address::arbitrary(g),
            amount: u32::arbitrary(g) % 1000, // Reasonable amounts
        }
    }
}

#[quickcheck]
fn test_transfer_properties(ops: Vec<TransferOperation>) -> bool {
    let env = Env::default();
    let contract_id = env.register_contract(None, TokenContract);
    let client = TokenContractClient::new(&env, &contract_id);
    
    // Setup initial balances
    for op in &ops {
        client.mint(&op.from, &1000);
    }
    
    // Execute transfers
    for op in &ops {
        if op.amount > 0 && op.from != op.to {
            let _ = client.transfer(&op.from, &op.to, &op.amount);
        }
    }
    
    // Verify total supply remains constant
    let total_supply: u32 = ops.iter()
        .map(|op| client.balance(&op.from) + client.balance(&op.to))
        .sum();
    
    total_supply == ops.len() as u32 * 1000
}
```

## Mocking and Stubbing

### **Mock External Dependencies**

```rust
#[test]
fn test_with_mock_oracle() {
    let env = Env::default();
    
    // Mock price oracle
    let mock_oracle_id = env.register_contract(None, MockPriceOracle);
    let mock_oracle = MockPriceOracleClient::new(&env, &mock_oracle_id);
    
    // Set mock price
    mock_oracle.set_price(&"ETH".into(), &2000);
    
    // Test contract that uses oracle
    let contract_id = env.register_contract(None, TradingContract);
    let client = TradingContractClient::new(&env, &contract_id);
    
    let price = client.get_asset_price(&"ETH".into());
    assert_eq!(price, 2000);
}
```

## Performance Testing

### **Gas Usage Testing**

```rust
#[test]
fn test_gas_usage() {
    let env = Env::default();
    let contract_id = env.register_contract(None, OptimizedContract);
    let client = OptimizedContractClient::new(&env, &contract_id);
    
    // Measure gas usage for different operations
    let start_gas = env.ledger().gas_consumed();
    
    client.expensive_operation(&1000);
    
    let end_gas = env.ledger().gas_consumed();
    let gas_used = end_gas - start_gas;
    
    // Ensure gas usage is within acceptable limits
    assert!(gas_used < 1000000, "Gas usage too high: {}", gas_used);
}
```

## Security Testing

### **Access Control Testing**

```rust
#[test]
fn test_access_control() {
    let env = Env::default();
    let contract_id = env.register_contract(None, SecureContract);
    let client = SecureContractClient::new(&env, &contract_id);
    
    let owner = Address::random(&env);
    let user = Address::random(&env);
    let attacker = Address::random(&env);
    
    // Setup: set owner
    client.initialize(&owner);
    
    // Test: only owner can call admin functions
    let result = client.admin_function(&user);
    assert!(result.is_err());
    
    let result = client.admin_function(&attacker);
    assert!(result.is_err());
    
    let result = client.admin_function(&owner);
    assert!(result.is_ok());
}
```

### **Reentrancy Testing**

```rust
#[test]
fn test_reentrancy_protection() {
    let env = Env::default();
    let contract_id = env.register_contract(None, ReentrancyContract);
    let client = ReentrancyContractClient::new(&env, &contract_id);
    
    let user = Address::random(&env);
    
    // Setup: fund user
    client.fund(&user, &1000);
    
    // Test: attempt reentrancy attack
    let result = client.withdraw(&user, &500);
    
    // Verify: attack was prevented
    assert!(result.is_ok());
    assert_eq!(client.balance(&user), 500);
    
    // Verify: no double-spending occurred
    let result = client.withdraw(&user, &500);
    assert!(result.is_ok());
    assert_eq!(client.balance(&user), 0);
}
```

## Test Organization

### **Test Module Structure**

```rust
#[cfg(test)]
mod test {
    use super::*;
    
    mod unit {
        use super::*;
        
        #[test]
        fn test_basic_functionality() { /* ... */ }
        
        #[test]
        fn test_edge_cases() { /* ... */ }
    }
    
    mod integration {
        use super::*;
        
        #[test]
        fn test_contract_interaction() { /* ... */ }
        
        #[test]
        fn test_complex_workflows() { /* ... */ }
    }
    
    mod security {
        use super::*;
        
        #[test]
        fn test_access_control() { /* ... */ }
        
        #[test]
        fn test_reentrancy() { /* ... */ }
    }
}
```

## Running Tests

### **Basic Test Commands**

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_increment

# Run tests with output
cargo test -- --nocapture

# Run tests in parallel
cargo test -- --test-threads=4

# Run tests with coverage
cargo test --coverage
```

### **Test Configuration**

```toml
# Cargo.toml
[dev-dependencies]
quickcheck = "1.0"
proptest = "1.0"

[profile.test]
opt-level = 0  # Faster compilation for tests
```

## Continuous Integration

### **GitHub Actions Example**

```yaml
name: Test Contracts

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      - uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
          target: wasm32v1-none
      
      - name: Install wasm target
        run: rustup target add wasm32v1-none
      
      - name: Run tests
        run: cargo test --target wasm32v1-none
      
      - name: Build contracts
        run: stellar contract build
```

## Best Practices

### **1. Test Coverage**
- Aim for 90%+ code coverage
- Test all code paths (happy path, error cases, edge cases)
- Test boundary conditions
- Test with realistic data

### **2. Test Isolation**
- Each test should be independent
- Use fresh environment for each test
- Avoid shared state between tests
- Clean up after tests

### **3. Test Naming**
- Use descriptive test names
- Follow pattern: `test_[scenario]_[expected_result]`
- Group related tests together
- Use consistent naming conventions

### **4. Test Data**
- Use realistic test data
- Test with edge cases (0, max values, negative numbers)
- Use random data when appropriate
- Avoid hardcoded magic numbers

### **5. Error Testing**
- Test all error conditions
- Verify error messages are helpful
- Test error recovery mechanisms
- Ensure errors don't leave state inconsistent

## Common Testing Mistakes

### **1. Insufficient Coverage**
```rust
// ❌ Only testing happy path
#[test]
fn test_transfer() {
    // Only tests successful transfer
    let result = client.transfer(&from, &to, &100);
    assert!(result.is_ok());
}

// ✅ Testing all scenarios
#[test]
fn test_transfer_scenarios() {
    // Test successful transfer
    let result = client.transfer(&from, &to, &100);
    assert!(result.is_ok());
    
    // Test insufficient balance
    let result = client.transfer(&from, &to, &10000);
    assert!(result.is_err());
    
    // Test zero amount
    let result = client.transfer(&from, &to, &0);
    assert!(result.is_err());
}
```

### **2. Testing Implementation Details**
```rust
// ❌ Testing internal implementation
#[test]
fn test_internal_state() {
    // Don't test private fields or internal state
    assert_eq!(client.internal_counter, 5);
}

// ✅ Testing public behavior
#[test]
fn test_public_behavior() {
    // Test what users can observe
    assert_eq!(client.get_counter(), 5);
}
```

## What's Next?

Now that you understand testing strategies, you're ready to:

1. **[Deploy your contracts](deploying-contracts.md)** - Get tested contracts on the network
2. **[Build frontend applications](frontend/README.md)** - Create user interfaces
3. **[Explore advanced topics](advanced/README.md)** - Master complex testing scenarios
4. **[Contribute to the ecosystem](resources/README.md)** - Help improve testing tools

Remember: Good testing is the foundation of reliable smart contracts. Test early, test often, and test thoroughly!

---

**Next**: [Deploying Contracts](deploying-contracts.md) - Getting your contracts on the network
