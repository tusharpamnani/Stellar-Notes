# Contract Optimization

Optimizing your smart contracts is crucial for cost efficiency and user experience. This section will teach you how to reduce gas costs, improve performance, and create contracts that scale effectively on the Stellar network.

## What You'll Learn

- **Gas Optimization** - Reducing transaction costs
- **Storage Efficiency** - Optimizing data storage patterns
- **Execution Optimization** - Faster contract execution
- **Batch Operations** - Processing multiple operations efficiently
- **Performance Monitoring** - Measuring and improving performance

## Gas Optimization Fundamentals

### **Understanding Gas Costs**

On Stellar, gas costs are primarily determined by:
- **Storage Operations** - Reading and writing data
- **Computational Complexity** - CPU-intensive operations
- **Network Calls** - External contract interactions
- **Data Size** - Amount of data processed

### **Gas Cost Breakdown**

```rust
// Example gas costs for common operations
pub fn gas_analysis(env: Env) {
    // Storage read: ~100 gas
    let value = env.storage().instance().get(&symbol_short!("KEY")).unwrap_or(0);
    
    // Storage write: ~20,000 gas
    env.storage().instance().set(&symbol_short!("KEY"), &value);
    
    // Simple computation: ~1-10 gas per operation
    let result = value + 1;
    
    // Event emission: ~3,750 gas
    env.events().publish((
        symbol_short!("event"),
        result
    ));
}
```

## Storage Optimization Techniques

### **1. Efficient Data Types**

Choose the most gas-efficient data types for your use case.

```rust
// ❌ INEFFICIENT: Using large types unnecessarily
pub struct InefficientUser {
    pub id: u128,           // 16 bytes - overkill for user ID
    pub balance: u128,       // 16 bytes - could use u64
    pub is_active: bool,     // 1 byte
    pub created_at: u128,    // 16 bytes - timestamp could be u64
}

// ✅ EFFICIENT: Optimized data types
pub struct EfficientUser {
    pub id: u32,            // 4 bytes - sufficient for user count
    pub balance: u64,        // 8 bytes - sufficient for token amounts
    pub is_active: bool,     // 1 byte
    pub created_at: u64,     // 8 bytes - sufficient for timestamp
}

// Total: 21 bytes vs 49 bytes (57% reduction)
```

### **2. Packed Storage**

Group related data into single storage slots when possible.

```rust
// ❌ INEFFICIENT: Separate storage for related data
pub fn store_user_data(env: &Env, user: &Address) {
    env.storage().instance().set(&(symbol_short!("USER_BALANCE"), user), &1000u64);
    env.storage().instance().set(&(symbol_short!("USER_ACTIVE"), user), &true);
    env.storage().instance().set(&(symbol_short!("USER_LEVEL"), user), &5u8);
}

// ✅ EFFICIENT: Packed storage structure
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserData {
    pub balance: u64,    // 8 bytes
    pub is_active: bool, // 1 byte
    pub level: u8,       // 1 byte
    // Total: 10 bytes (packed efficiently)
}

pub fn store_user_data_optimized(env: &Env, user: &Address) {
    let user_data = UserData {
        balance: 1000,
        is_active: true,
        level: 5,
    };
    
    // Single storage operation
    env.storage().instance().set(&(symbol_short!("USER_DATA"), user), &user_data);
}
```

### **3. Storage Layout Optimization**

Organize storage keys for efficient access patterns.

```rust
// ❌ INEFFICIENT: Scattered storage keys
pub fn inefficient_storage(env: &Env) {
    env.storage().instance().set(&symbol_short!("CONFIG_1"), &"value1");
    env.storage().instance().set(&symbol_short!("CONFIG_2"), &"value2");
    env.storage().instance().set(&symbol_short!("CONFIG_3"), &"value3");
    env.storage().instance().set(&symbol_short!("USER_COUNT"), &100);
    env.storage().instance().set(&symbol_short!("TOTAL_SUPPLY"), &1000000);
}

// ✅ EFFICIENT: Grouped storage keys
pub fn efficient_storage(env: &Env) {
    // Group configuration data
    let config = Config {
        value1: "value1",
        value2: "value2",
        value3: "value3",
    };
    env.storage().instance().set(&symbol_short!("CONFIG"), &config);
    
    // Group statistics data
    let stats = Stats {
        user_count: 100,
        total_supply: 1000000,
    };
    env.storage().instance().set(&symbol_short!("STATS"), &stats);
}
```

### **4. Sparse Storage**

Only store non-zero values and use efficient default handling.

```rust
// ❌ INEFFICIENT: Storing all values including zeros
pub fn store_all_balances(env: &Env, balances: &[u64]) {
    for (i, &balance) in balances.iter().enumerate() {
        // Stores even zero balances
        env.storage().instance().set(&(symbol_short!("BALANCE"), i as u32), &balance);
    }
}

// ✅ EFFICIENT: Sparse storage (only non-zero values)
pub fn store_sparse_balances(env: &Env, balances: &[u64]) {
    for (i, &balance) in balances.iter().enumerate() {
        if balance > 0 {
            // Only store non-zero balances
            env.storage().instance().set(&(symbol_short!("BALANCE"), i as u32), &balance);
        }
    }
}

// Reading with efficient defaults
pub fn get_balance(env: &Env, user_id: u32) -> u64 {
    env.storage().instance().get(&(symbol_short!("BALANCE"), user_id)).unwrap_or(0)
}
```

## Computational Optimization

### **1. Loop Optimization**

Optimize loops for better gas efficiency.

```rust
// ❌ INEFFICIENT: Multiple storage operations in loop
pub fn inefficient_loop(env: &Env, users: &[Address]) -> u64 {
    let mut total_balance = 0u64;
    
    for user in users {
        // Storage read in every iteration
        let balance = env.storage().instance().get(&(symbol_short!("BALANCE"), user)).unwrap_or(0);
        total_balance += balance;
    }
    
    total_balance
}

// ✅ EFFICIENT: Batch storage operations
pub fn efficient_loop(env: &Env, users: &[Address]) -> u64 {
    let mut total_balance = 0u64;
    let mut batch_balances = Vec::new();
    
    // Collect all balances first
    for user in users {
        let balance = env.storage().instance().get(&(symbol_short!("BALANCE"), user)).unwrap_or(0);
        batch_balances.push(balance);
    }
    
    // Process in batch
    for balance in batch_balances {
        total_balance += balance;
    }
    
    total_balance
}
```

### **2. Early Returns**

Use early returns to avoid unnecessary computation.

```rust
// ❌ INEFFICIENT: Nested conditions
pub fn inefficient_validation(env: &Env, user: &Address, amount: u64) -> Result<(), Error> {
    let user_exists = env.storage().instance().has(&(symbol_short!("USER"), user));
    
    if user_exists {
        let user_data = env.storage().instance().get(&(symbol_short!("USER_DATA"), user));
        if let Some(data) = user_data {
            if data.is_active {
                if amount > 0 {
                    if amount <= data.balance {
                        // Process transaction
                        return Ok(());
                    } else {
                        return Err(Error::InsufficientBalance);
                    }
                } else {
                    return Err(Error::InvalidAmount);
                }
            } else {
                return Err(Error::UserInactive);
            }
        } else {
            return Err(Error::UserDataNotFound);
        }
    } else {
        return Err(Error::UserNotFound);
    }
}

// ✅ EFFICIENT: Early returns
pub fn efficient_validation(env: &Env, user: &Address, amount: u64) -> Result<(), Error> {
    // Early return for basic validation
    if amount == 0 {
        return Err(Error::InvalidAmount);
    }
    
    // Early return if user doesn't exist
    if !env.storage().instance().has(&(symbol_short!("USER"), user)) {
        return Err(Error::UserNotFound);
    }
    
    // Get user data once
    let user_data = env.storage().instance().get(&(symbol_short!("USER_DATA"), user))
        .ok_or(Error::UserDataNotFound)?;
    
    // Early return if user is inactive
    if !user_data.is_active {
        return Err(Error::UserInactive);
    }
    
    // Early return if insufficient balance
    if amount > user_data.balance {
        return Err(Error::InsufficientBalance);
    }
    
    // Process transaction
    Ok(())
}
```

### **3. Function Inlining**

Use inline functions for simple operations to reduce call overhead.

```rust
// ❌ INEFFICIENT: Separate function calls
fn get_user_balance(env: &Env, user: &Address) -> u64 {
    env.storage().instance().get(&(symbol_short!("BALANCE"), user)).unwrap_or(0)
}

fn update_user_balance(env: &Env, user: &Address, new_balance: u64) {
    env.storage().instance().set(&(symbol_short!("BALANCE"), user), &new_balance);
}

pub fn transfer_tokens(env: &Env, from: &Address, to: &Address, amount: u64) -> Result<(), Error> {
    let from_balance = get_user_balance(env, from);
    let to_balance = get_user_balance(env, to);
    
    if from_balance < amount {
        return Err(Error::InsufficientBalance);
    }
    
    update_user_balance(env, from, from_balance - amount);
    update_user_balance(env, to, to_balance + amount);
    
    Ok(())
}

// ✅ EFFICIENT: Inline operations
pub fn transfer_tokens_optimized(env: &Env, from: &Address, to: &Address, amount: u64) -> Result<(), Error> {
    // Inline balance reads
    let from_balance = env.storage().instance().get(&(symbol_short!("BALANCE"), from)).unwrap_or(0);
    
    if from_balance < amount {
        return Err(Error::InsufficientBalance);
    }
    
    let to_balance = env.storage().instance().get(&(symbol_short!("BALANCE"), to)).unwrap_or(0);
    
    // Inline balance updates
    env.storage().instance().set(&(symbol_short!("BALANCE"), from), &(from_balance - amount));
    env.storage().instance().set(&(symbol_short!("BALANCE"), to), &(to_balance + amount));
    
    Ok(())
}
```

## Batch Operations

### **1. Batch Processing**

Process multiple operations in a single transaction to reduce gas costs.

```rust
// ❌ INEFFICIENT: Individual operations
pub fn process_individual_transfers(
    env: &Env,
    transfers: &[(Address, Address, u64)]
) -> Result<(), Error> {
    for (from, to, amount) in transfers {
        Self::transfer_tokens(env, from, to, *amount)?;
    }
    Ok(())
}

// ✅ EFFICIENT: Batch processing
pub fn process_batch_transfers(
    env: &Env,
    transfers: &[(Address, Address, u64)]
) -> Result<(), Error> {
    // Validate all transfers first
    for (from, to, amount) in transfers {
        if *amount == 0 {
            return Err(Error::InvalidAmount);
        }
    }
    
    // Process all transfers
    for (from, to, amount) in transfers {
        let from_balance = env.storage().instance().get(&(symbol_short!("BALANCE"), from)).unwrap_or(0);
        
        if from_balance < *amount {
            return Err(Error::InsufficientBalance);
        }
    }
    
    // Execute all transfers
    for (from, to, amount) in transfers {
        let from_balance = env.storage().instance().get(&(symbol_short!("BALANCE"), from)).unwrap_or(0);
        let to_balance = env.storage().instance().get(&(symbol_short!("BALANCE"), to)).unwrap_or(0);
        
        env.storage().instance().set(&(symbol_short!("BALANCE"), from), &(from_balance - amount));
        env.storage().instance().set(&(symbol_short!("BALANCE"), to), &(to_balance + amount));
    }
    
    Ok(())
}
```

### **2. Merkle Tree Operations**

Use Merkle trees for efficient batch verification.

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MerkleProof {
    pub leaf: Vec<u8>,
    pub siblings: Vec<Vec<u8>>,
    pub path: Vec<bool>,
}

impl MerkleProof {
    pub fn verify(&self, root: &[u8]) -> bool {
        let mut current = self.leaf.clone();
        
        for (i, &is_right) in self.path.iter().enumerate() {
            let sibling = &self.siblings[i];
            
            if is_right {
                current = Self::hash_pair(sibling, &current);
            } else {
                current = Self::hash_pair(&current, sibling);
            }
        }
        
        current == root
    }
    
    fn hash_pair(left: &[u8], right: &[u8]) -> Vec<u8> {
        // Simple hash function for demonstration
        let mut combined = Vec::new();
        combined.extend_from_slice(left);
        combined.extend_from_slice(right);
        
        // In production, use proper cryptographic hash
        combined
    }
}

// Efficient batch verification
pub fn verify_batch_claims(
    env: &Env,
    root: &[u8],
    proofs: &[MerkleProof]
) -> Result<bool, Error> {
    for proof in proofs {
        if !proof.verify(root) {
            return Err(Error::InvalidProof);
        }
    }
    
    Ok(true)
}
```

## Memory and Data Structure Optimization

### **1. Efficient Data Structures**

Choose the right data structures for your use case.

```rust
// ❌ INEFFICIENT: Using Vec for small, fixed-size data
pub struct InefficientUser {
    pub permissions: Vec<bool>, // 32 bytes for 32 permissions
}

// ✅ EFFICIENT: Using bit flags for permissions
pub struct EfficientUser {
    pub permissions: u32, // 4 bytes for 32 permissions
}

impl EfficientUser {
    pub fn has_permission(&self, permission: u8) -> bool {
        if permission >= 32 {
            return false;
        }
        
        (self.permissions & (1 << permission)) != 0
    }
    
    pub fn grant_permission(&mut self, permission: u8) {
        if permission < 32 {
            self.permissions |= 1 << permission;
        }
    }
    
    pub fn revoke_permission(&mut self, permission: u8) {
        if permission < 32 {
            self.permissions &= !(1 << permission);
        }
    }
}
```

### **2. Memory Pooling**

Reuse memory allocations when possible.

```rust
// ❌ INEFFICIENT: Creating new vectors for each operation
pub fn process_data(env: &Env, data: &[u64]) -> Vec<u64> {
    let mut result = Vec::new();
    
    for &value in data {
        if value > 100 {
            result.push(value * 2);
        }
    }
    
    result
}

// ✅ EFFICIENT: Pre-allocate memory
pub fn process_data_optimized(env: &Env, data: &[u64]) -> Vec<u64> {
    // Pre-allocate with estimated capacity
    let estimated_size = data.len() / 2; // Rough estimate
    let mut result = Vec::with_capacity(estimated_size);
    
    for &value in data {
        if value > 100 {
            result.push(value * 2);
        }
    }
    
    result
}
```

## Performance Monitoring and Profiling

### **1. Gas Usage Tracking**

Monitor gas usage to identify optimization opportunities.

```rust
pub struct GasTracker {
    start_gas: u64,
    operations: Vec<(String, u64)>,
}

impl GasTracker {
    pub fn new() -> Self {
        Self {
            start_gas: 0, // Would be set from environment in real contract
            operations: Vec::new(),
        }
    }
    
    pub fn track_operation(&mut self, operation: &str, gas_used: u64) {
        self.operations.push((operation.to_string(), gas_used));
    }
    
    pub fn get_total_gas(&self) -> u64 {
        self.operations.iter().map(|(_, gas)| gas).sum()
    }
    
    pub fn get_operation_breakdown(&self) -> &[(String, u64)] {
        &self.operations
    }
}

// Usage in contract
pub fn optimized_function(env: &Env) -> Result<(), Error> {
    let mut tracker = GasTracker::new();
    
    // Track storage operations
    let start = env.ledger().timestamp();
    env.storage().instance().set(&symbol_short!("KEY"), &100u64);
    let storage_gas = env.ledger().timestamp() - start;
    tracker.track_operation("storage_write", storage_gas);
    
    // Track computation
    let start = env.ledger().timestamp();
    let result = (0..1000).sum::<u64>();
    let compute_gas = env.ledger().timestamp() - start;
    tracker.track_operation("computation", compute_gas);
    
    // Log gas usage
    env.events().publish((
        symbol_short!("gas_usage"),
        tracker.get_total_gas()
    ));
    
    Ok(())
}
```

### **2. Performance Benchmarks**

Create benchmarks to measure performance improvements.

```rust
#[cfg(test)]
mod benchmarks {
    use super::*;
    use soroban_sdk::{Address, Env};
    use std::time::Instant;

    #[test]
    fn benchmark_storage_operations() {
        let env = Env::default();
        let contract_id = env.register_contract(None, OptimizedContract);
        let client = OptimizedContractClient::new(&env, &contract_id);
        
        let iterations = 1000;
        
        // Benchmark inefficient approach
        let start = Instant::now();
        for i in 0..iterations {
            client.inefficient_storage_operation(&i);
        }
        let inefficient_time = start.elapsed();
        
        // Benchmark efficient approach
        let start = Instant::now();
        for i in 0..iterations {
            client.efficient_storage_operation(&i);
        }
        let efficient_time = start.elapsed();
        
        println!("Inefficient: {:?}", inefficient_time);
        println!("Efficient: {:?}", efficient_time);
        println!("Improvement: {:.2}x", inefficient_time.as_nanos() as f64 / efficient_time.as_nanos() as f64);
        
        // Ensure efficient is faster
        assert!(efficient_time < inefficient_time);
    }
}
```

## Optimization Checklist

### **Before Optimization**

- [ ] **Profile Current Performance** - Measure gas usage and execution time
- [ ] **Identify Bottlenecks** - Find the most expensive operations
- [ ] **Set Performance Goals** - Define target improvements
- [ ] **Benchmark Baseline** - Establish current performance metrics

### **During Optimization**

- [ ] **Storage Optimization** - Reduce storage operations and data size
- [ ] **Computational Optimization** - Optimize loops and algorithms
- [ ] **Batch Operations** - Combine multiple operations
- [ ] **Data Structure Optimization** - Choose efficient data types
- [ ] **Memory Management** - Minimize allocations and reuse memory

### **After Optimization**

- [ ] **Measure Improvements** - Compare before and after performance
- [ ] **Test Functionality** - Ensure optimizations don't break functionality
- [ ] **Document Changes** - Record optimization techniques used
- [ ] **Monitor Production** - Track performance in production environment

## What You've Learned

### **Optimization Techniques**
- ✅ **Storage Efficiency** - Reducing storage costs and data size
- ✅ **Computational Optimization** - Faster execution and reduced gas usage
- ✅ **Batch Processing** - Combining operations for efficiency
- ✅ **Memory Management** - Efficient data structures and memory usage
- ✅ **Performance Monitoring** - Measuring and tracking improvements

### **Best Practices**
- ✅ **Data Type Selection** - Choosing appropriate types for use cases
- ✅ **Storage Layout** - Organizing data for efficient access
- ✅ **Algorithm Optimization** - Efficient algorithms and data structures
- ✅ **Gas Tracking** - Monitoring and optimizing gas usage
- ✅ **Benchmarking** - Measuring performance improvements

### **Production Optimization**
- ✅ **Performance Profiling** - Identifying optimization opportunities
- ✅ **Continuous Monitoring** - Tracking performance over time
- ✅ **Optimization Iteration** - Continuous improvement process
- ✅ **Cost-Benefit Analysis** - Balancing optimization with complexity

## Next Steps

### **Immediate Actions**
1. **Profile Your Contracts** - Measure current gas usage
2. **Identify Bottlenecks** - Find expensive operations
3. **Implement Optimizations** - Apply techniques learned here
4. **Measure Improvements** - Compare before and after

### **Advanced Optimization**
1. **[Testing Strategies](testing-strategies.md)** - Performance testing approaches
2. **Algorithm Analysis** - Time and space complexity optimization
3. **Gas Modeling** - Predictive gas cost analysis
4. **Performance Engineering** - Systematic performance improvement

### **Tools and Resources**
- **Gas Profilers** - Tools for measuring gas usage
- **Performance Monitors** - Production performance tracking
- **Benchmarking Frameworks** - Performance testing tools
- **Optimization Libraries** - Pre-optimized components

## Summary

Contract optimization is a continuous process that requires understanding of gas costs, storage patterns, and computational efficiency. The techniques you've learned here will help you create contracts that are not only functional but also cost-effective and performant.

Remember: Optimization should never come at the expense of security or correctness. Always test thoroughly and measure improvements to ensure your optimizations are effective and safe.

The best optimizations are those that maintain code clarity while significantly improving performance. Focus on high-impact, low-complexity improvements first!

---

**Next**: [Testing Strategies](testing-strategies.md) - Advanced testing approaches and methodologies
