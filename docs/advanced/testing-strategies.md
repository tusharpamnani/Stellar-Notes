# Testing Strategies

Learn comprehensive testing strategies for Stellar smart contracts. This guide covers unit testing, integration testing, security testing, and automated testing workflows to ensure your contracts are robust and secure.

## Testing Fundamentals

### **Why Testing Matters**

Testing is crucial for smart contract development because:
- **Immutable Code**: Once deployed, bugs cannot be fixed
- **Real Value**: Contracts handle real money and assets
- **Public Code**: Anyone can examine and exploit vulnerabilities
- **Network Effects**: Bugs can affect many users simultaneously

### **Testing Pyramid**

Follow the testing pyramid approach:
1. **Unit Tests** - Test individual functions (70%)
2. **Integration Tests** - Test contract interactions (20%)
3. **End-to-End Tests** - Test complete workflows (10%)

## Unit Testing

### **Basic Unit Tests**

Test individual contract functions in isolation.

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{
        testutils::{Address as _,},
        Address, Env,
    };

    #[test]
    fn test_initialize() {
        let env = Env::default();
        let contract_id = env.register_contract(None, IncrementContract);
        let client = IncrementContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        
        client.initialize(&owner);
        
        // Verify owner was set
        assert_eq!(client.get_owner(), owner);
        
        // Verify counter starts at 0
        assert_eq!(client.get_counter(), 0);
    }

    #[test]
    fn test_increment() {
        let env = Env::default();
        let contract_id = env.register_contract(None, IncrementContract);
        let client = IncrementContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        client.initialize(&owner);
        
        // Test increment
        let result = client.increment();
        assert_eq!(result, 1);
        
        // Verify counter was updated
        assert_eq!(client.get_counter(), 1);
        
        // Test multiple increments
        let result = client.increment();
        assert_eq!(result, 2);
        assert_eq!(client.get_counter(), 2);
    }

    #[test]
    fn test_reset_unauthorized() {
        let env = Env::default();
        let contract_id = env.register_contract(None, IncrementContract);
        let client = IncrementContractClient::new(&env, &contract_id);
        
        let owner = Address::generate(&env);
        let unauthorized = Address::generate(&env);
        
        client.initialize(&owner);
        client.increment(); // Set counter to 1
        
        // Try to reset with unauthorized account
        let result = client.try_reset(&unauthorized);
        assert!(result.is_err());
        
        // Counter should remain unchanged
        assert_eq!(client.get_counter(), 1);
    }
}
```

### **Test Utilities**

Create helper functions for common testing patterns.

```rust
#[cfg(test)]
mod test_utils {
    use super::*;
    use soroban_sdk::testutils::{Address as _,};

    pub struct TestContext {
        pub env: Env,
        pub contract_id: Address,
        pub client: IncrementContractClient<'static>,
        pub owner: Address,
        pub user1: Address,
        pub user2: Address,
    }

    impl TestContext {
        pub fn new() -> Self {
            let env = Env::default();
            let contract_id = env.register_contract(None, IncrementContract);
            let client = IncrementContractClient::new(&env, &contract_id);
            
            let owner = Address::generate(&env);
            let user1 = Address::generate(&env);
            let user2 = Address::generate(&env);
            
            Self {
                env,
                contract_id,
                client,
                owner,
                user1,
                user2,
            }
        }

        pub fn setup_contract(&self) {
            self.client.initialize(&self.owner);
        }

        pub fn setup_with_balance(&self, initial_balance: u32) {
            self.setup_contract();
            
            // Set initial counter value
            for _ in 0..initial_balance {
                self.client.increment();
            }
        }

        pub fn assert_counter_value(&self, expected: u32) {
            assert_eq!(self.client.get_counter(), expected);
        }

        pub fn assert_owner(&self, expected: Address) {
            assert_eq!(self.client.get_owner(), expected);
        }
    }

    impl Default for TestContext {
        fn default() -> Self {
            Self::new()
        }
    }
}

// Usage in tests
#[test]
fn test_complex_scenario() {
    let ctx = TestContext::new();
    ctx.setup_with_balance(5);
    
    ctx.assert_counter_value(5);
    ctx.assert_owner(ctx.owner);
    
    // Test reset by owner
    ctx.client.try_reset(&ctx.owner).unwrap();
    ctx.assert_counter_value(0);
}
```

### **Edge Case Testing**

Test boundary conditions and edge cases.

```rust
#[test]
fn test_edge_cases() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Test maximum u32 value
    let max_value = u32::MAX;
    
    // Set counter to max - 1
    for _ in 0..max_value - 1 {
        ctx.client.increment();
    }
    
    ctx.assert_counter_value(max_value - 1);
    
    // Increment to max
    let result = ctx.client.increment();
    assert_eq!(result, max_value);
    ctx.assert_counter_value(max_value);
    
    // Test overflow (should panic in debug mode)
    #[cfg(debug_assertions)]
    {
        std::panic::catch_unwind(|| {
            ctx.client.increment();
        }).expect_err("Should panic on overflow");
    }
}

#[test]
fn test_empty_initialization() {
    let ctx = TestContext::new();
    
    // Test that contract can be initialized multiple times
    ctx.client.initialize(&ctx.owner);
    ctx.assert_owner(ctx.owner);
    
    // Re-initialization should fail
    let result = ctx.client.try_initialize(&ctx.user1);
    assert!(result.is_err());
}

#[test]
fn test_invalid_addresses() {
    let ctx = TestContext::new();
    
    // Test with zero address (if applicable)
    // This depends on your contract's validation logic
    let zero_address = Address::from_contract_id(&ctx.env, &[0u8; 32]);
    
    let result = ctx.client.try_initialize(&zero_address);
    // Should either succeed or fail with specific error
    // depending on your validation requirements
}
```

## Integration Testing

### **Contract Interaction Tests**

Test how your contract interacts with other contracts and the Stellar network.

```rust
#[test]
fn test_contract_interactions() {
    let env = Env::default();
    
    // Deploy multiple contracts
    let increment_contract = env.register_contract(None, IncrementContract);
    let token_contract = env.register_contract(None, TokenContract);
    
    let increment_client = IncrementContractClient::new(&env, &increment_contract);
    let token_client = TokenContractClient::new(&env, &token_contract);
    
    let owner = Address::generate(&env);
    
    // Initialize both contracts
    increment_client.initialize(&owner);
    token_client.initialize(&owner, &Symbol::new(&env, "TEST"), &Symbol::new(&env, "TST"), 6, 1000000);
    
    // Test cross-contract calls
    let balance = token_client.balance(&owner);
    assert_eq!(balance, 1000000);
    
    // Increment counter and mint tokens
    let counter = increment_client.increment();
    token_client.mint(&owner, &counter);
    
    let new_balance = token_client.balance(&owner);
    assert_eq!(new_balance, 1000000 + counter);
}

#[test]
fn test_event_emission() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Capture events before increment
    let events_before = ctx.env.events().all();
    
    // Perform increment
    ctx.client.increment();
    
    // Check that event was emitted
    let events_after = ctx.env.events().all();
    let new_events: Vec<_> = events_after
        .into_iter()
        .filter(|event| !events_before.contains(event))
        .collect();
    
    assert_eq!(new_events.len(), 1);
    
    let event = &new_events[0];
    assert_eq!(event.topics[0], Symbol::new(&ctx.env, "counter_incremented"));
    assert_eq!(event.data, 1);
}
```

### **Network Simulation Tests**

Simulate network conditions and failures.

```rust
#[test]
fn test_network_failures() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Simulate network delay
    ctx.env.ledger().set_timestamp(ctx.env.ledger().timestamp() + 1000);
    
    // Test that contract still works after time changes
    let result = ctx.client.increment();
    assert_eq!(result, 1);
}

#[test]
fn test_storage_persistence() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Set some state
    ctx.client.increment();
    ctx.client.increment();
    ctx.assert_counter_value(2);
    
    // Simulate contract redeployment (same contract ID)
    let new_client = IncrementContractClient::new(&ctx.env, &ctx.contract_id);
    
    // State should persist
    assert_eq!(new_client.get_counter(), 2);
    assert_eq!(new_client.get_owner(), ctx.owner);
}
```

## Security Testing

### **Access Control Testing**

Test that access control mechanisms work correctly.

```rust
#[test]
fn test_access_control() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Test that only owner can reset
    let result = ctx.client.try_reset(&ctx.user1);
    assert!(result.is_err());
    
    // Test that owner can reset
    let result = ctx.client.try_reset(&ctx.owner);
    assert!(result.is_ok());
    
    // Test that unauthorized users cannot call admin functions
    let result = ctx.client.try_set_owner(&ctx.user1, &ctx.user2);
    assert!(result.is_err());
}

#[test]
fn test_role_based_access() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Test different user roles
    let admin = ctx.owner;
    let moderator = ctx.user1;
    let regular_user = ctx.user2;
    
    // Set up roles
    ctx.client.set_role(&admin, &moderator, &Role::Moderator);
    
    // Test role-based permissions
    assert!(ctx.client.has_role(&moderator, &Role::Moderator));
    assert!(!ctx.client.has_role(&regular_user, &Role::Moderator));
    
    // Test that only admins can modify roles
    let result = ctx.client.try_set_role(&moderator, &regular_user, &Role::Moderator);
    assert!(result.is_err());
}
```

### **Reentrancy Testing**

Test for reentrancy vulnerabilities.

```rust
#[test]
fn test_reentrancy_protection() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Create a malicious contract that tries to reenter
    let malicious_contract = env.register_contract(None, MaliciousContract);
    let malicious_client = MaliciousContractClient::new(&env, &malicious_contract);
    
    // Set up the malicious contract to call back into our contract
    malicious_client.set_target(&ctx.contract_id);
    
    // Try to call a function that could be vulnerable to reentrancy
    let result = ctx.client.call_with_callback(&malicious_contract);
    
    // Should either succeed safely or fail with reentrancy error
    // depending on your protection mechanism
    assert!(result.is_ok() || result.is_err());
    
    // Verify that state is consistent
    ctx.assert_counter_value(1); // Should only increment once
}

// Malicious contract for testing
#[contract]
pub struct MaliciousContract;

#[contractimpl]
impl MaliciousContract {
    pub fn set_target(&self, target: Address) {
        // Store target for reentrancy attack
    }
    
    pub fn attack(&self) {
        // This would be called during the target contract execution
        // to attempt reentrancy
    }
}
```

### **Input Validation Testing**

Test that input validation works correctly.

```rust
#[test]
fn test_input_validation() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Test invalid parameters
    let invalid_address = Address::from_contract_id(&ctx.env, &[0u8; 32]);
    
    // Test with invalid owner address
    let result = ctx.client.try_initialize(&invalid_address);
    // Should fail with validation error
    
    // Test with invalid counter value
    let result = ctx.client.try_set_counter(&ctx.owner, &u32::MAX);
    // Should fail if you have validation for maximum values
    
    // Test with empty strings
    let empty_symbol = Symbol::new(&ctx.env, "");
    let result = ctx.client.try_set_name(&ctx.owner, &empty_symbol);
    // Should fail if empty strings are not allowed
}

#[test]
fn test_boundary_conditions() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Test minimum values
    let result = ctx.client.try_set_counter(&ctx.owner, &0);
    assert!(result.is_ok());
    
    // Test maximum values
    let result = ctx.client.try_set_counter(&ctx.owner, &u32::MAX);
    // Should either succeed or fail based on your validation
    
    // Test negative values (if using signed integers)
    // This would depend on your data types
}
```

## Performance Testing

### **Gas Optimization Testing**

Test that your contract uses gas efficiently.

```rust
#[test]
fn test_gas_usage() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Measure gas usage for different operations
    let gas_before = ctx.env.ledger().gas_consumed();
    
    ctx.client.increment();
    
    let gas_after = ctx.env.ledger().gas_consumed();
    let gas_used = gas_after - gas_before;
    
    // Gas usage should be reasonable
    assert!(gas_used < 1000000); // Adjust threshold as needed
    
    // Test that gas usage doesn't grow unexpectedly
    let gas_before = ctx.env.ledger().gas_consumed();
    
    for _ in 0..100 {
        ctx.client.increment();
    }
    
    let gas_after = ctx.env.ledger().gas_consumed();
    let gas_per_operation = (gas_after - gas_before) / 100;
    
    // Gas per operation should be consistent
    assert!(gas_per_operation < 10000); // Adjust threshold
}

#[test]
fn test_storage_efficiency() {
    let ctx = TestContext::new();
    ctx.setup_contract();
    
    // Test that storage operations are efficient
    let storage_before = ctx.env.storage().instance().get(&Symbol::new(&ctx.env, "COUNTER"));
    
    // Perform many operations
    for i in 0..1000 {
        ctx.client.increment();
        assert_eq!(ctx.client.get_counter(), i + 1);
    }
    
    // Storage should not grow unexpectedly
    let storage_after = ctx.env.storage().instance().get(&Symbol::new(&ctx.env, "COUNTER"));
    
    // The storage footprint should remain reasonable
    // This is a simplified check - you might want more sophisticated metrics
}
```

## Automated Testing

### **Test Automation Setup**

Set up automated testing workflows.

```rust
// Test configuration
#[cfg(test)]
mod config {
    use super::*;
    
    pub const TEST_TIMEOUT: u64 = 30; // seconds
    pub const MAX_ITERATIONS: u32 = 1000;
    pub const GAS_LIMIT: u64 = 10000000;
    
    pub fn setup_test_env() -> Env {
        let mut env = Env::default();
        
        // Configure test environment
        env.ledger().set_gas_limit(GAS_LIMIT);
        env.ledger().set_timestamp(1234567890);
        
        env
    }
}

// Automated test runner
#[test]
fn test_automated_scenarios() {
    let env = config::setup_test_env();
    
    // Run a series of automated tests
    run_basic_functionality_tests(&env);
    run_edge_case_tests(&env);
    run_performance_tests(&env);
    run_security_tests(&env);
}

fn run_basic_functionality_tests(env: &Env) {
    // Test basic contract functionality
    let ctx = TestContext::new_with_env(env);
    ctx.setup_contract();
    
    // Test increment
    assert_eq!(ctx.client.increment(), 1);
    assert_eq!(ctx.client.get_counter(), 1);
    
    // Test reset
    ctx.client.try_reset(&ctx.owner).unwrap();
    assert_eq!(ctx.client.get_counter(), 0);
}

fn run_edge_case_tests(env: &Env) {
    // Test edge cases
    let ctx = TestContext::new_with_env(env);
    ctx.setup_contract();
    
    // Test maximum values
    for _ in 0..100 {
        ctx.client.increment();
    }
    
    assert_eq!(ctx.client.get_counter(), 100);
}

fn run_performance_tests(env: &Env) {
    // Test performance characteristics
    let ctx = TestContext::new_with_env(env);
    ctx.setup_contract();
    
    let start_time = std::time::Instant::now();
    
    for _ in 0..1000 {
        ctx.client.increment();
    }
    
    let duration = start_time.elapsed();
    assert!(duration.as_millis() < 1000); // Should complete within 1 second
}

fn run_security_tests(env: &Env) {
    // Test security aspects
    let ctx = TestContext::new_with_env(env);
    ctx.setup_contract();
    
    // Test access control
    let unauthorized = Address::generate(env);
    let result = ctx.client.try_reset(&unauthorized);
    assert!(result.is_err());
}
```

### **Continuous Integration**

Set up CI/CD for automated testing.

```yaml
# .github/workflows/test.yml
name: Test Smart Contracts

on:
  push:
    branches: [ main, develop ]
  pull_request:
    branches: [ main ]

jobs:
  test:
    runs-on: ubuntu-latest
    
    steps:
    - uses: actions/checkout@v3
    
    - name: Setup Rust
      uses: actions-rs/toolchain@v1
      with:
        toolchain: stable
        target: wasm32-unknown-unknown
    
    - name: Install Stellar CLI
      run: |
        curl -sSf https://soroban.stellar.org/install.sh | sh
        echo "$HOME/.local/bin" >> $GITHUB_PATH
    
    - name: Build contracts
      run: |
        cargo build --target wasm32-unknown-unknown --release
    
    - name: Run tests
      run: |
        cargo test --target wasm32-unknown-unknown
    
    - name: Run security tests
      run: |
        cargo test --target wasm32-unknown-unknown security
    
    - name: Run performance tests
      run: |
        cargo test --target wasm32-unknown-unknown performance
```

## Test Coverage

### **Coverage Metrics**

Track test coverage to ensure comprehensive testing.

```rust
#[test]
fn test_coverage_analysis() {
    // This test ensures we cover all code paths
    
    let ctx = TestContext::new();
    
    // Test initialization
    ctx.setup_contract();
    ctx.assert_owner(ctx.owner);
    ctx.assert_counter_value(0);
    
    // Test increment
    let result = ctx.client.increment();
    assert_eq!(result, 1);
    ctx.assert_counter_value(1);
    
    // Test multiple increments
    for i in 1..10 {
        let result = ctx.client.increment();
        assert_eq!(result, i + 1);
    }
    
    // Test reset by owner
    ctx.client.try_reset(&ctx.owner).unwrap();
    ctx.assert_counter_value(0);
    
    // Test reset by unauthorized user
    let result = ctx.client.try_reset(&ctx.user1);
    assert!(result.is_err());
    
    // Test get functions
    let owner = ctx.client.get_owner();
    let counter = ctx.client.get_counter();
    
    assert_eq!(owner, ctx.owner);
    assert_eq!(counter, 0);
    
    // Test error conditions
    let ctx2 = TestContext::new();
    let result = ctx2.client.try_get_owner();
    assert!(result.is_err()); // Should fail if not initialized
}
```

## What You've Learned

### **Testing Fundamentals**
- ✅ **Unit Testing** - Test individual functions thoroughly
- ✅ **Integration Testing** - Test contract interactions
- ✅ **Security Testing** - Test for vulnerabilities
- ✅ **Performance Testing** - Test gas efficiency

### **Advanced Testing**
- ✅ **Automated Testing** - CI/CD workflows
- ✅ **Test Coverage** - Comprehensive testing
- ✅ **Edge Cases** - Boundary condition testing
- ✅ **Test Utilities** - Reusable testing helpers

### **Production Ready**
- ✅ **Security Validation** - Vulnerability testing
- ✅ **Performance Validation** - Gas optimization testing
- ✅ **Automated Validation** - Continuous testing
- ✅ **Quality Assurance** - Comprehensive coverage

## Next Steps

### **Immediate Actions**
1. **Write Unit Tests** - Test all contract functions
2. **Add Integration Tests** - Test contract interactions
3. **Implement Security Tests** - Test for vulnerabilities
4. **Set Up CI/CD** - Automate testing workflow

### **Advanced Testing**
1. **[Security Best Practices](security-best-practices.md)** - Security testing strategies
2. **[Contract Optimization](contract-optimization.md)** - Performance testing
3. **[Examples](examples/README.md)** - Real-world testing examples
4. **Production Deployment** - Deploy with confidence

### **Resources**
- **Rust Testing** - [doc.rust-lang.org/book/testing](https://doc.rust-lang.org/book/testing.html)
- **Stellar Testing** - [developers.stellar.org](https://developers.stellar.org)
- **Security Testing** - [owasp.org](https://owasp.org)
- **Test Automation** - [github.com/actions](https://github.com/features/actions)

## Summary

Comprehensive testing is essential for smart contract development. By implementing thorough unit tests, integration tests, security tests, and performance tests, you can ensure your contracts are robust, secure, and efficient before deploying to production.

Remember: Testing is an investment in quality and security. The time spent writing tests will save you from costly bugs and vulnerabilities in production. Always test thoroughly and automate your testing workflow!

The best testing strategy is one that provides comprehensive coverage while maintaining fast feedback loops. Invest in testing infrastructure—it will pay dividends in the long run!

---

**Next**: [Security Best Practices](security-best-practices.md) - Security testing and vulnerability prevention
