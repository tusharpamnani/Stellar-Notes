# Token Contract

The Token Contract example demonstrates how to create and manage custom tokens on the Stellar network. This is a fundamental building block for DeFi applications, NFT marketplaces, and any system that requires custom asset management.

## What This Example Teaches

- **Token Creation** - How to mint new tokens
- **Token Transfer** - Moving tokens between accounts
- **Balance Management** - Tracking token balances
- **Access Control** - Managing who can mint/burn tokens
- **Event Emission** - Notifying when token events occur

## Project Overview

This contract implements a standard ERC-20-like token with:
1. **Configurable Supply** - Total token supply
2. **Mint Function** - Create new tokens (owner only)
3. **Burn Function** - Destroy tokens (owner only)
4. **Transfer Function** - Move tokens between accounts
5. **Balance Queries** - Check token balances
6. **Event Logging** - Track all token operations

## Code Walkthrough

### **Complete Contract Code**

```rust
#![no_std]
use soroban_sdk::{
    contract, contractimpl, symbol_short, 
    Env, Symbol, Address, log, 
    Map, Vec, IntoVal, TryFromVal
};

#[contract]
pub struct TokenContract;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenInfo {
    pub name: Symbol,
    pub symbol: Symbol,
    pub decimals: u8,
    pub total_supply: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferEvent {
    pub from: Address,
    pub to: Address,
    pub amount: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MintEvent {
    pub to: Address,
    pub amount: i128,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BurnEvent {
    pub from: Address,
    pub amount: i128,
}

#[contractimpl]
impl TokenContract {
    /// Initialize the token contract
    pub fn initialize(
        env: Env,
        owner: Address,
        name: Symbol,
        symbol: Symbol,
        decimals: u8,
        initial_supply: i128
    ) -> Result<(), Error> {
        // Check if already initialized
        if env.storage().instance().has(&symbol_short!("INITIALIZED")) {
            return Err(Error::AlreadyInitialized);
        }
        
        // Validate parameters
        if decimals > 18 {
            return Err(Error::InvalidDecimals);
        }
        
        if initial_supply < 0 {
            return Err(Error::InvalidSupply);
        }
        
        // Store token info
        let token_info = TokenInfo {
            name,
            symbol,
            decimals,
            total_supply: initial_supply,
        };
        
        env.storage().instance().set(&symbol_short!("TOKEN_INFO"), &token_info);
        env.storage().instance().set(&symbol_short!("OWNER"), &owner);
        env.storage().instance().set(&symbol_short!("INITIALIZED"), &true);
        
        // Mint initial supply to owner
        if initial_supply > 0 {
            Self::mint_internal(&env, &owner, initial_supply)?;
        }
        
        log!(&env, "Token initialized: {} ({})", name, symbol);
        Ok(())
    }
    
    /// Get token information
    pub fn get_token_info(env: Env) -> Result<TokenInfo, Error> {
        env.storage().instance().get(&symbol_short!("TOKEN_INFO"))
            .ok_or(Error::NotInitialized)
    }
    
    /// Get token balance for an address
    pub fn balance_of(env: Env, address: Address) -> Result<i128, Error> {
        let balance_key = (symbol_short!("BALANCE"), address);
        Ok(env.storage().instance().get(&balance_key).unwrap_or(0))
    }
    
    /// Get total token supply
    pub fn total_supply(env: Env) -> Result<i128, Error> {
        let token_info: TokenInfo = Self::get_token_info(env)?;
        Ok(token_info.total_supply)
    }
    
    /// Transfer tokens from caller to recipient
    pub fn transfer(
        env: Env,
        from: Address,
        to: Address,
        amount: i128
    ) -> Result<bool, Error> {
        // Validate parameters
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        if from == to {
            return Ok(true); // No-op transfer
        }
        
        // Check sender balance
        let from_balance = Self::balance_of(env.clone(), from.clone())?;
        if from_balance < amount {
            return Err(Error::InsufficientBalance);
        }
        
        // Update balances
        let from_balance_key = (symbol_short!("BALANCE"), from.clone());
        let to_balance_key = (symbol_short!("BALANCE"), to.clone());
        
        env.storage().instance().set(&from_balance_key, &(from_balance - amount));
        env.storage().instance().set(&to_balance_key, &(Self::balance_of(env.clone(), to.clone())? + amount));
        
        // Emit transfer event
        let transfer_event = TransferEvent { from, to, amount };
        env.events().publish((
            symbol_short!("transfer"),
            transfer_event
        ));
        
        log!(&env, "Transfer: {} tokens from {} to {}", amount, from, to);
        Ok(true)
    }
    
    /// Mint new tokens (owner only)
    pub fn mint(
        env: Env,
        caller: Address,
        to: Address,
        amount: i128
    ) -> Result<bool, Error> {
        // Check authorization
        let owner = Self::get_owner(&env)?;
        if caller != owner {
            return Err(Error::Unauthorized);
        }
        
        // Validate amount
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        // Mint tokens
        Self::mint_internal(&env, &to, amount)?;
        
        // Emit mint event
        let mint_event = MintEvent { to, amount };
        env.events().publish((
            symbol_short!("mint"),
            mint_event
        ));
        
        log!(&env, "Minted {} tokens to {}", amount, to);
        Ok(true)
    }
    
    /// Burn tokens (owner only)
    pub fn burn(
        env: Env,
        caller: Address,
        from: Address,
        amount: i128
    ) -> Result<bool, Error> {
        // Check authorization
        let owner = Self::get_owner(&env)?;
        if caller != owner {
            return Err(Error::Unauthorized);
        }
        
        // Validate amount
        if amount <= 0 {
            return Err(Error::InvalidAmount);
        }
        
        // Check balance
        let balance = Self::balance_of(env.clone(), from.clone())?;
        if balance < amount {
            return Err(Error::InsufficientBalance);
        }
        
        // Burn tokens
        Self::burn_internal(&env, &from, amount)?;
        
        // Emit burn event
        let burn_event = BurnEvent { from, amount };
        env.events().publish((
            symbol_short!("burn"),
            burn_event
        ));
        
        log!(&env, "Burned {} tokens from {}", amount, from);
        Ok(true)
    }
    
    /// Get contract owner
    pub fn get_owner(env: Env) -> Result<Address, Error> {
        env.storage().instance().get(&symbol_short!("OWNER"))
            .ok_or(Error::NotInitialized)
    }
    
    // Internal helper functions
    
    fn mint_internal(env: &Env, to: &Address, amount: i128) -> Result<(), Error> {
        // Update balance
        let balance_key = (symbol_short!("BALANCE"), to.clone());
        let current_balance = env.storage().instance().get(&balance_key).unwrap_or(0);
        env.storage().instance().set(&balance_key, &(current_balance + amount));
        
        // Update total supply
        let mut token_info: TokenInfo = Self::get_token_info(env.clone())?;
        token_info.total_supply += amount;
        env.storage().instance().set(&symbol_short!("TOKEN_INFO"), &token_info);
        
        Ok(())
    }
    
    fn burn_internal(env: &Env, from: &Address, amount: i128) -> Result<(), Error> {
        // Update balance
        let balance_key = (symbol_short!("BALANCE"), from.clone());
        let current_balance = env.storage().instance().get(&balance_key).unwrap_or(0);
        env.storage().instance().set(&balance_key, &(current_balance - amount));
        
        // Update total supply
        let mut token_info: TokenInfo = Self::get_token_info(env.clone())?;
        token_info.total_supply -= amount;
        env.storage().instance().set(&symbol_short!("TOKEN_INFO"), &token_info);
        
        Ok(())
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Error {
    AlreadyInitialized,
    NotInitialized,
    InvalidDecimals,
    InvalidSupply,
    InvalidAmount,
    InsufficientBalance,
    Unauthorized,
}

impl From<Error> for soroban_sdk::Error {
    fn from(err: Error) -> Self {
        soroban_sdk::Error::from_type_and_code(err.into(), 0)
    }
}
```

### **Key Components Explained**

#### **Token Information Structure**
```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TokenInfo {
    pub name: Symbol,        // Token name (e.g., "My Token")
    pub symbol: Symbol,      // Token symbol (e.g., "MTK")
    pub decimals: u8,        // Decimal places (e.g., 6 for 6 decimals)
    pub total_supply: i128,  // Total tokens in existence
}
```

#### **Event Structures**
```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TransferEvent {
    pub from: Address,       // Sender address
    pub to: Address,         // Recipient address
    pub amount: i128,        // Amount transferred
}
```

#### **Storage Keys**
```rust
symbol_short!("TOKEN_INFO")     // Stores token metadata
symbol_short!("OWNER")          // Contract owner address
symbol_short!("BALANCE")        // Individual token balances
symbol_short!("INITIALIZED")    // Initialization flag
```

## Building and Testing

### **Step 1: Create Project Structure**
```bash
mkdir token-contract
cd token-contract
cargo init --lib
```

### **Step 2: Add Dependencies**
```toml
# Cargo.toml
[package]
name = "token-contract"
version = "0.1.0"
edition = "2021"

[lib]
crate-type = ["cdylib"]

[dependencies]
soroban-sdk = "20.0.0"

[dev_dependencies]
soroban-sdk = { version = "20.0.0", features = ["testutils"] }

[profile.release]
opt-level = "z"
overflow-checks = true
```

### **Step 3: Build Contract**
```bash
stellar contract build
```

### **Step 4: Run Tests**
```bash
cargo test
```

## Deployment and Usage

### **Step 1: Deploy Contract**
```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/token_contract.wasm \
  --source alice \
  --network testnet \
  --alias my-token
```

### **Step 2: Initialize Token**
```bash
# Get your address
stellar keys address alice

# Initialize token (replace with your address)
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  --send=yes \
  -- \
  initialize \
  --owner <YOUR_ADDRESS> \
  --name "My Token" \
  --symbol "MTK" \
  --decimals 6 \
  --initial_supply 1000000000
```

### **Step 3: Check Token Info**
```bash
# Get token information
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  get_token_info

# Check your balance
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  -- \
  balance_of \
  --address <YOUR_ADDRESS>
```

### **Step 4: Transfer Tokens**
```bash
# Create another identity for testing
stellar keys generate --global bob --network testnet --fund

# Get bob's address
stellar keys address bob

# Transfer tokens to bob
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  --send=yes \
  -- \
  transfer \
  --from <YOUR_ADDRESS> \
  --to <BOB_ADDRESS> \
  --amount 100000
```

### **Step 5: Mint More Tokens**
```bash
# Mint additional tokens
stellar contract invoke \
  --id <CONTRACT_ID> \
  --source alice \
  --network testnet \
  --send=yes \
  -- \
  mint \
  --caller <YOUR_ADDRESS> \
  --to <YOUR_ADDRESS> \
  --amount 500000
```

## Advanced Features

### **1. Batch Operations**
```rust
/// Transfer tokens to multiple recipients
pub fn batch_transfer(
    env: Env,
    from: Address,
    recipients: Vec<Address>,
    amounts: Vec<i128>
) -> Result<bool, Error> {
    if recipients.len() != amounts.len() {
        return Err(Error::InvalidBatch);
    }
    
    for (i, recipient) in recipients.iter().enumerate() {
        Self::transfer(env.clone(), from.clone(), recipient.clone(), amounts[i])?;
    }
    
    Ok(true)
}
```

### **2. Allowance System**
```rust
/// Approve spender to spend tokens
pub fn approve(
    env: Env,
    owner: Address,
    spender: Address,
    amount: i128
) -> Result<bool, Error> {
    let allowance_key = (symbol_short!("ALLOWANCE"), owner, spender);
    env.storage().instance().set(&allowance_key, &amount);
    
    // Emit approval event
    env.events().publish((
        symbol_short!("approval"),
        owner, spender, amount
    ));
    
    Ok(true)
}

/// Transfer tokens using allowance
pub fn transfer_from(
    env: Env,
    spender: Address,
    from: Address,
    to: Address,
    amount: i128
) -> Result<bool, Error> {
    let allowance_key = (symbol_short!("ALLOWANCE"), from.clone(), spender.clone());
    let allowance = env.storage().instance().get(&allowance_key).unwrap_or(0);
    
    if allowance < amount {
        return Err(Error::InsufficientAllowance);
    }
    
    // Update allowance
    env.storage().instance().set(&allowance_key, &(allowance - amount));
    
    // Perform transfer
    Self::transfer(env, from, to, amount)
}
```

### **3. Pausable Token**
```rust
/// Pause all token operations
pub fn pause(env: Env, caller: Address) -> Result<bool, Error> {
    let owner = Self::get_owner(&env)?;
    if caller != owner {
        return Err(Error::Unauthorized);
    }
    
    env.storage().instance().set(&symbol_short!("PAUSED"), &true);
    
    // Emit pause event
    env.events().publish((symbol_short!("paused"), caller));
    
    Ok(true)
}

/// Unpause token operations
pub fn unpause(env: Env, caller: Address) -> Result<bool, Error> {
    let owner = Self::get_owner(&env)?;
    if caller != owner {
        return Err(Error::Unauthorized);
    }
    
    env.storage().instance().set(&symbol_short!("PAUSED"), &false);
    
    // Emit unpause event
    env.events().publish((symbol_short!("unpaused"), caller));
    
    Ok(true)
}

/// Check if token is paused
fn is_paused(env: &Env) -> bool {
    env.storage().instance().get(&symbol_short!("PAUSED")).unwrap_or(false)
}
```

### **4. Token Metadata**
```rust
/// Set token metadata URI
pub fn set_metadata_uri(
    env: Env,
    caller: Address,
    uri: Symbol
) -> Result<bool, Error> {
    let owner = Self::get_owner(&env)?;
    if caller != owner {
        return Err(Error::Unauthorized);
    }
    
    env.storage().instance().set(&symbol_short!("METADATA_URI"), &uri);
    
    Ok(true)
}

/// Get token metadata URI
pub fn get_metadata_uri(env: Env) -> Result<Symbol, Error> {
    env.storage().instance().get(&symbol_short!("METADATA_URI"))
        .ok_or(Error::MetadataNotFound)
}
```

## Integration with Frontend

### **Generate TypeScript Bindings**
```bash
stellar contract bindings typescript \
  --network testnet \
  --contract-id <CONTRACT_ID> \
  --output-dir src/contracts
```

### **Frontend Usage Example**
```typescript
import { Client, networks } from './src/contracts';

// Create client
const client = new Client({
    ...networks.testnet,
    rpcUrl: 'https://soroban-testnet.stellar.org:443'
});

// Get token info
const tokenInfo = await client.getTokenInfo();
console.log(`Token: ${tokenInfo.name} (${tokenInfo.symbol})`);

// Check balance
const balance = await client.balanceOf({ address: userAddress });
console.log(`Balance: ${balance}`);

// Transfer tokens
const result = await client.transfer({
    from: userAddress,
    to: recipientAddress,
    amount: 100000
});
```

## Common Use Cases

### **1. DeFi Tokens**
- **Liquidity Pool Tokens** - Representing LP positions
- **Governance Tokens** - Voting rights in DAOs
- **Yield Farming Tokens** - Rewards for providing liquidity

### **2. NFT Marketplaces**
- **Platform Tokens** - Used for fees and rewards
- **Creator Tokens** - Supporting specific creators
- **Utility Tokens** - Access to premium features

### **3. Gaming Platforms**
- **In-Game Currency** - Purchasing items and upgrades
- **Reward Tokens** - Achievements and milestones
- **Staking Tokens** - Earning rewards over time

## Security Considerations

### **1. Access Control**
- **Owner-Only Functions** - Mint, burn, pause
- **Role-Based Access** - Different permission levels
- **Multi-Signature** - Multiple approvals required

### **2. Input Validation**
- **Amount Checks** - Positive values only
- **Address Validation** - Valid Stellar addresses
- **Decimal Limits** - Reasonable decimal places

### **3. Reentrancy Protection**
- **State Updates First** - Update state before external calls
- **Checks-Effects-Interactions** - Follow CEI pattern
- **Guard Modifiers** - Prevent reentrant calls

## What You've Learned

### **Core Concepts**
- ✅ **Token Creation** - How to mint new tokens
- ✅ **Balance Management** - Tracking token balances
- ✅ **Transfer Logic** - Moving tokens between accounts
- ✅ **Access Control** - Managing permissions
- ✅ **Event Emission** - Tracking token operations

### **Technical Skills**
- ✅ **Complex State Management** - Multiple data structures
- ✅ **Batch Operations** - Processing multiple operations
- ✅ **Error Handling** - Comprehensive error management
- ✅ **Gas Optimization** - Efficient storage patterns
- ✅ **Frontend Integration** - TypeScript bindings

### **Best Practices**
- ✅ **Initialization Pattern** - One-time setup
- ✅ **Event Logging** - Complete audit trail
- ✅ **Input Validation** - Parameter checking
- ✅ **Authorization Checks** - Permission verification
- ✅ **Modular Design** - Reusable components

## Next Steps

### **Immediate Next Steps**
1. **Customize the Token** - Add new features and functions
2. **Test Extensively** - Try edge cases and error conditions
3. **Deploy and Test** - Get real experience with the network

### **Learning Path**
1. **[Advanced Topics](advanced/README.md)** - Security and optimization
2. **[Frontend Integration](frontend/README.md)** - Building user interfaces
3. **[Production Deployment](smart-contracts/deploying-contracts.md)** - Going live

### **Advanced Topics**
- [Security Best Practices](advanced/security-best-practices.md)
- [Contract Optimization](advanced/contract-optimization.md)
- [Testing Strategies](advanced/testing-strategies.md)

## Summary

Congratulations! You've successfully built a comprehensive token contract that demonstrates:

- **Token Economics** - Supply management and distribution
- **Access Control** - Secure permission management
- **Event System** - Complete operation tracking
- **Error Handling** - Robust error management
- **Integration Ready** - Frontend-friendly design

The Token Contract is a fundamental building block for many blockchain applications. The patterns you've learned here—state management, access control, event emission, and error handling—are essential for building secure and scalable smart contracts.

Remember: Tokens represent real value, so security and testing are paramount. Always test thoroughly on testnet before deploying to mainnet!

---

**Next**: [Advanced Topics](advanced/README.md) - Security, optimization, and production readiness
