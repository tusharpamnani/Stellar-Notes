# Security Best Practices

Security is paramount in smart contract development. Since deployed contracts are immutable and often handle real value, vulnerabilities can lead to catastrophic losses. This section will teach you how to protect your contracts from common attacks and security pitfalls.

## What You'll Learn

- **Common Vulnerabilities** - Understanding attack vectors
- **Security Patterns** - Proven protection mechanisms
- **Access Control** - Managing permissions securely
- **Input Validation** - Preventing malicious inputs
- **Testing Strategies** - Security-focused testing

## Common Smart Contract Vulnerabilities

### **1. Reentrancy Attacks**

Reentrancy occurs when a contract calls an external contract before updating its own state, allowing the external contract to call back and exploit the incomplete state.

#### **Vulnerable Pattern**
```rust
// ❌ VULNERABLE: State update after external call
pub fn withdraw(env: Env, caller: Address, amount: i128) -> Result<(), Error> {
    let balance = Self::get_balance(&env, &caller)?;
    
    if balance < amount {
        return Err(Error::InsufficientBalance);
    }
    
    // ❌ External call before state update
    Self::transfer_tokens(&env, &caller, amount)?;
    
    // State update happens after external call
    Self::update_balance(&env, &caller, balance - amount)?;
    
    Ok(())
}
```

#### **Secure Pattern (Checks-Effects-Interactions)**
```rust
// ✅ SECURE: State update before external call
pub fn withdraw(env: Env, caller: Address, amount: i128) -> Result<(), Error> {
    let balance = Self::get_balance(&env, &caller)?;
    
    if balance < amount {
        return Err(Error::InsufficientBalance);
    }
    
    // ✅ Update state FIRST
    Self::update_balance(&env, &caller, balance - amount)?;
    
    // ✅ External call LAST
    Self::transfer_tokens(&env, &caller, amount)?;
    
    Ok(())
}
```

#### **Reentrancy Guard Pattern**
```rust
use soroban_sdk::symbol_short;

#[contract]
pub struct SecureContract {
    // Track reentrancy state
    reentrancy_guard: bool,
}

#[contractimpl]
impl SecureContract {
    pub fn secure_function(env: Env) -> Result<(), Error> {
        // Check reentrancy guard
        if env.storage().instance().get(&symbol_short!("REENTRANCY_GUARD")).unwrap_or(false) {
            return Err(Error::ReentrancyDetected);
        }
        
        // Set reentrancy guard
        env.storage().instance().set(&symbol_short!("REENTRANCY_GUARD"), &true);
        
        // Function logic here
        Self::perform_operation(&env)?;
        
        // Clear reentrancy guard
        env.storage().instance().set(&symbol_short!("REENTRANCY_GUARD"), &false);
        
        Ok(())
    }
}
```

### **2. Access Control Vulnerabilities**

Improper access control can allow unauthorized users to perform privileged operations.

#### **Vulnerable Pattern**
```rust
// ❌ VULNERABLE: No access control
pub fn mint_tokens(env: Env, to: Address, amount: i128) -> Result<(), Error> {
    // Anyone can mint tokens!
    Self::mint_internal(&env, &to, amount)?;
    Ok(())
}
```

#### **Secure Pattern (Role-Based Access Control)**
```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Role {
    Owner,
    Minter,
    Pauser,
    Upgrader,
}

#[contractimpl]
impl SecureContract {
    pub fn mint_tokens(
        env: Env, 
        caller: Address, 
        to: Address, 
        amount: i128
    ) -> Result<(), Error> {
        // ✅ Check if caller has minter role
        if !Self::has_role(&env, &caller, Role::Minter) {
            return Err(Error::Unauthorized);
        }
        
        Self::mint_internal(&env, &to, amount)?;
        Ok(())
    }
    
    fn has_role(env: &Env, user: &Address, role: Role) -> bool {
        let role_key = (symbol_short!("ROLE"), user, role);
        env.storage().instance().get(&role_key).unwrap_or(false)
    }
    
    pub fn grant_role(
        env: Env,
        caller: Address,
        user: Address,
        role: Role
    ) -> Result<(), Error> {
        // Only owner can grant roles
        if !Self::has_role(&env, &caller, Role::Owner) {
            return Err(Error::Unauthorized);
        }
        
        let role_key = (symbol_short!("ROLE"), user, role);
        env.storage().instance().set(&role_key, &true);
        
        // Emit event
        env.events().publish((
            symbol_short!("role_granted"),
            user, role
        ));
        
        Ok(())
    }
}
```

### **3. Integer Overflow/Underflow**

While Rust provides built-in protection, it's important to validate inputs and handle edge cases.

#### **Secure Integer Handling**
```rust
pub fn safe_add(a: i128, b: i128) -> Result<i128, Error> {
    // Check for overflow
    if b > 0 && a > i128::MAX - b {
        return Err(Error::Overflow);
    }
    
    // Check for underflow
    if b < 0 && a < i128::MIN - b {
        return Err(Error::Underflow);
    }
    
    Ok(a + b)
}

pub fn safe_multiply(a: i128, b: i128) -> Result<i128, Error> {
    // Check for overflow
    if a != 0 && b != 0 {
        if a > i128::MAX / b.abs() {
            return Err(Error::Overflow);
        }
    }
    
    Ok(a * b)
}
```

### **4. Input Validation**

Always validate inputs to prevent malicious data from causing unexpected behavior.

#### **Comprehensive Input Validation**
```rust
pub fn create_user(
    env: Env,
    caller: Address,
    username: Symbol,
    email: Symbol
) -> Result<(), Error> {
    // ✅ Validate username length
    let username_str = username.to_string();
    if username_str.len() < 3 || username_str.len() > 20 {
        return Err(Error::InvalidUsername);
    }
    
    // ✅ Validate username characters (alphanumeric only)
    if !username_str.chars().all(|c| c.is_alphanumeric()) {
        return Err(Error::InvalidUsername);
    }
    
    // ✅ Validate email format (basic)
    let email_str = email.to_string();
    if !email_str.contains('@') || !email_str.contains('.') {
        return Err(Error::InvalidEmail);
    }
    
    // ✅ Check if username already exists
    if Self::username_exists(&env, &username) {
        return Err(Error::UsernameTaken);
    }
    
    // Create user
    Self::create_user_internal(&env, &caller, &username, &email)?;
    
    Ok(())
}
```

## Security Patterns and Best Practices

### **1. Pausable Pattern**

Allow contracts to be paused in emergency situations.

```rust
#[contractimpl]
impl SecureContract {
    pub fn pause(env: Env, caller: Address) -> Result<(), Error> {
        // Only pauser can pause
        if !Self::has_role(&env, &caller, Role::Pauser) {
            return Err(Error::Unauthorized);
        }
        
        env.storage().instance().set(&symbol_short!("PAUSED"), &true);
        
        // Emit pause event
        env.events().publish((
            symbol_short!("paused"),
            caller
        ));
        
        Ok(())
    }
    
    pub fn unpause(env: Env, caller: Address) -> Result<(), Error> {
        // Only pauser can unpause
        if !Self::has_role(&env, &caller, Role::Pauser) {
            return Err(Error::Unauthorized);
        }
        
        env.storage().instance().set(&symbol_short!("PAUSED"), &false);
        
        // Emit unpause event
        env.events().publish((
            symbol_short!("unpaused"),
            caller
        ));
        
        Ok(())
    }
    
    fn when_not_paused(env: &Env) -> Result<(), Error> {
        if env.storage().instance().get(&symbol_short!("PAUSED")).unwrap_or(false) {
            return Err(Error::ContractPaused);
        }
        Ok(())
    }
    
    // Use in all state-changing functions
    pub fn transfer(env: Env, from: Address, to: Address, amount: i128) -> Result<(), Error> {
        // ✅ Check if contract is paused
        Self::when_not_paused(&env)?;
        
        // Transfer logic here
        Self::transfer_internal(&env, &from, &to, amount)?;
        
        Ok(())
    }
}
```

### **2. Upgradeable Pattern**

Design contracts that can be upgraded while maintaining state.

```rust
#[contract]
pub struct UpgradeableContract {
    // Implementation address
    implementation: Address,
    // Admin address
    admin: Address,
}

#[contractimpl]
impl UpgradeableContract {
    pub fn upgrade(env: Env, caller: Address, new_implementation: Address) -> Result<(), Error> {
        // Only admin can upgrade
        let admin = Self::get_admin(&env)?;
        if caller != admin {
            return Err(Error::Unauthorized);
        }
        
        // Validate new implementation
        if new_implementation == Address::default() {
            return Err(Error::InvalidImplementation);
        }
        
        // Update implementation
        env.storage().instance().set(&symbol_short!("IMPLEMENTATION"), &new_implementation);
        
        // Emit upgrade event
        env.events().publish((
            symbol_short!("upgraded"),
            new_implementation
        ));
        
        Ok(())
    }
    
    pub fn get_admin(env: Env) -> Result<Address, Error> {
        env.storage().instance().get(&symbol_short!("ADMIN"))
            .ok_or(Error::AdminNotSet)
    }
    
    pub fn change_admin(env: Env, caller: Address, new_admin: Address) -> Result<(), Error> {
        // Only current admin can change admin
        let admin = Self::get_admin(&env)?;
        if caller != admin {
            return Err(Error::Unauthorized);
        }
        
        // Validate new admin
        if new_admin == Address::default() {
            return Err(Error::InvalidAdmin);
        }
        
        // Update admin
        env.storage().instance().set(&symbol_short!("ADMIN"), &new_admin);
        
        // Emit admin change event
        env.events().publish((
            symbol_short!("admin_changed"),
            admin, new_admin
        ));
        
        Ok(())
    }
}
```

### **3. Emergency Stop Pattern**

Provide emergency mechanisms to halt critical operations.

```rust
#[contractimpl]
impl SecureContract {
    pub fn emergency_stop(env: Env, caller: Address) -> Result<(), Error> {
        // Only emergency admin can stop
        if !Self::has_role(&env, &caller, Role::EmergencyAdmin) {
            return Err(Error::Unauthorized);
        }
        
        env.storage().instance().set(&symbol_short!("EMERGENCY_STOP"), &true);
        
        // Emit emergency stop event
        env.events().publish((
            symbol_short!("emergency_stop"),
            caller
        ));
        
        Ok(())
    }
    
    pub fn resume_operations(env: Env, caller: Address) -> Result<(), Error> {
        // Only emergency admin can resume
        if !Self::has_role(&env, &caller, Role::EmergencyAdmin) {
            return Err(Error::Unauthorized);
        }
        
        env.storage().instance().set(&symbol_short!("EMERGENCY_STOP"), &false);
        
        // Emit resume event
        env.events().publish((
            symbol_short!("operations_resumed"),
            caller
        ));
        
        Ok(())
    }
    
    fn when_not_emergency_stopped(env: &Env) -> Result<(), Error> {
        if env.storage().instance().get(&symbol_short!("EMERGENCY_STOP")).unwrap_or(false) {
            return Err(Error::EmergencyStopActive);
        }
        Ok(())
    }
}
```

### **4. Rate Limiting Pattern**

Prevent abuse by limiting operation frequency.

```rust
#[contractimpl]
impl SecureContract {
    pub fn rate_limited_function(env: Env, caller: Address) -> Result<(), Error> {
        let now = env.ledger().timestamp();
        let last_call_key = (symbol_short!("LAST_CALL"), caller);
        let last_call = env.storage().instance().get(&last_call_key).unwrap_or(0);
        
        // Rate limit: 1 call per minute
        if now - last_call < 60 {
            return Err(Error::RateLimitExceeded);
        }
        
        // Update last call time
        env.storage().instance().set(&last_call_key, &now);
        
        // Function logic here
        Self::perform_function(&env)?;
        
        Ok(())
    }
}
```

## Security Testing Strategies

### **1. Unit Testing for Security**

```rust
#[cfg(test)]
mod test {
    use super::*;
    use soroban_sdk::{Address, Env, Symbol};

    #[test]
    fn test_reentrancy_protection() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SecureContract);
        let client = SecureContractClient::new(&env, &contract_id);
        
        // Test that reentrancy guard works
        let result = client.secure_function();
        assert!(result.is_ok());
        
        // Test that reentrancy is prevented
        // This would require a malicious contract to test properly
    }
    
    #[test]
    fn test_access_control() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SecureContract);
        let client = SecureContractClient::new(&env, &contract_id);
        
        let owner = Address::random(&env);
        let user = Address::random(&env);
        
        // Grant minter role to user
        client.grant_role(&owner, &user, &Role::Minter);
        
        // User should be able to mint
        let result = client.mint_tokens(&user, &user, &1000);
        assert!(result.is_ok());
        
        // Non-minter should not be able to mint
        let non_minter = Address::random(&env);
        let result = client.mint_tokens(&non_minter, &non_minter, &1000);
        assert!(result.is_err());
    }
    
    #[test]
    fn test_input_validation() {
        let env = Env::default();
        let contract_id = env.register_contract(None, SecureContract);
        let client = SecureContractClient::new(&env, &contract_id);
        
        // Test invalid username
        let result = client.create_user(
            &Address::random(&env),
            &Symbol::new(&env, "a"), // Too short
            &Symbol::new(&env, "test@example.com")
        );
        assert!(result.is_err());
        
        // Test invalid email
        let result = client.create_user(
            &Address::random(&env),
            &Symbol::new(&env, "validuser"),
            &Symbol::new(&env, "invalid-email") // No @ symbol
        );
        assert!(result.is_err());
    }
}
```

### **2. Fuzzing and Property-Based Testing**

```rust
#[cfg(test)]
mod fuzz_tests {
    use super::*;
    use soroban_sdk::{Address, Env, Symbol};
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_safe_math_properties(a: i64, b: i64) {
            let env = Env::default();
            let contract_id = env.register_contract(None, SecureContract);
            let client = SecureContractClient::new(&env, &contract_id);
            
            // Convert to i128 for contract
            let a_128 = a as i128;
            let b_128 = b as i128;
            
            // Test safe addition
            let result = client.safe_add(a_128, b_128);
            
            // If no overflow/underflow, result should be correct
            if a_128.checked_add(b_128).is_some() {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), a_128 + b_128);
            } else {
                assert!(result.is_err());
            }
        }
    }
}
```

## Security Checklist

### **Before Deployment**

- [ ] **Access Control**: All privileged functions have proper access checks
- [ ] **Input Validation**: All inputs are validated and sanitized
- [ ] **Reentrancy Protection**: External calls don't allow reentrancy
- [ ] **Integer Safety**: No potential for overflow/underflow
- [ ] **Emergency Mechanisms**: Pause and emergency stop functions
- [ ] **Event Logging**: All important state changes emit events
- [ ] **Error Handling**: Comprehensive error handling and user feedback
- [ ] **Testing**: Security-focused testing completed
- [ ] **Audit**: Professional security audit (recommended for mainnet)

### **After Deployment**

- [ ] **Monitoring**: Monitor for suspicious activity
- [ ] **Updates**: Keep dependencies updated
- [ ] **Incident Response**: Plan for security incidents
- [ ] **Community**: Engage with security community
- [ ] **Bounties**: Consider bug bounty programs

## What You've Learned

### **Security Fundamentals**
- ✅ **Vulnerability Recognition** - Identifying common attack vectors
- ✅ **Protection Patterns** - Implementing security mechanisms
- ✅ **Access Control** - Managing permissions securely
- ✅ **Input Validation** - Preventing malicious inputs
- ✅ **Testing Strategies** - Security-focused testing approaches

### **Best Practices**
- ✅ **Checks-Effects-Interactions** - Preventing reentrancy
- ✅ **Role-Based Access** - Granular permission management
- ✅ **Emergency Controls** - Pause and emergency stop mechanisms
- ✅ **Event Logging** - Complete audit trail
- ✅ **Comprehensive Testing** - Security validation

### **Production Security**
- ✅ **Security Patterns** - Proven protection mechanisms
- ✅ **Upgrade Strategies** - Maintaining security over time
- ✅ **Monitoring** - Detecting security issues
- ✅ **Incident Response** - Handling security incidents

## Next Steps

### **Immediate Actions**
1. **Audit Your Contracts** - Review existing code for vulnerabilities
2. **Implement Security Patterns** - Add protection mechanisms
3. **Test Security** - Comprehensive security testing
4. **Document Security** - Security documentation and procedures

### **Advanced Security**
1. **[Contract Optimization](contract-optimization.md)** - Performance and security
2. **[Testing Strategies](testing-strategies.md)** - Advanced testing approaches
3. **Formal Verification** - Mathematical proof of security
4. **Security Audits** - Professional security reviews

### **Community Resources**
- [Stellar Security](https://stellar.org/security/) - Official security resources
- [OpenZeppelin](https://openzeppelin.com/contracts/) - Security libraries
- [Consensys Diligence](https://consensys.net/diligence/) - Security tools
- [Trail of Bits](https://www.trailofbits.com/) - Security research

## Summary

Security in smart contract development is not optional—it's essential. The patterns and practices you've learned here will help you build contracts that are resistant to common attacks and vulnerabilities.

Remember: Security is a process, not a product. Always stay vigilant, keep learning, and never stop testing. Your users' funds and trust depend on it!

---

**Next**: [Contract Optimization](contract-optimization.md) - Performance and gas optimization
