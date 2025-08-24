# Deploying Contracts

Deployment is the final step in getting your smart contracts live on the blockchain. This section will teach you how to deploy contracts to testnet, mainnet, and handle the various considerations for each environment.

## Deployment Overview

### **What Happens During Deployment**

1. **Contract Compilation**: Rust code → WebAssembly (WASM)
2. **Network Selection**: Choose testnet, mainnet, or futurenet
3. **Transaction Creation**: Build deployment transaction
4. **Network Submission**: Send to blockchain network
5. **Confirmation**: Wait for network consensus
6. **Contract Activation**: Contract becomes callable

### **Deployment Networks**

| Network | Purpose | Cost | Risk | Lumens Required |
|---------|---------|------|------|-----------------|
| **Testnet** | Development & Testing | Free | None | 0 (Friendbot funded) |
| **Futurenet** | Advanced Testing | Free | None | 0 (Friendbot funded) |
| **Mainnet** | Production | Real XLM | High | 1 XLM minimum |

## Prerequisites

### **Before Deployment Checklist**

- ✅ [Environment setup complete](getting-started/environment-setup.md)
- ✅ Contract code written and tested
- ✅ All tests passing (`cargo test`)
- ✅ Contract builds successfully (`stellar contract build`)
- ✅ Network identity with sufficient Lumens
- ✅ Contract thoroughly tested locally

### **Required Tools**

```bash
# Verify Stellar CLI installation
stellar --version

# Verify Rust and WASM target
rustc --version
rustup target list | grep wasm32

# Verify contract builds
stellar contract build
```

## Building for Deployment

### **Step 1: Build the Contract**

```bash
cd your-contract-directory
stellar contract build
```

**Expected Output:**
```
✅ Contract wasm built successfully
📁 Output: target/wasm32v1-none/release/your-contract.wasm
```

### **Step 2: Verify WASM File**

```bash
# Check file size (should be reasonable)
ls -lh target/wasm32v1-none/release/your-contract.wasm

# Verify file integrity
file target/wasm32v1-none/release/your-contract.wasm
```

**File Size Guidelines:**
- **Small contracts**: < 100KB (simple logic)
- **Medium contracts**: 100KB - 500KB (moderate complexity)
- **Large contracts**: 500KB - 1MB (complex applications)
- **Very large**: > 1MB (consider optimization)

## Testnet Deployment

### **Step 1: Prepare Testnet Identity**

```bash
# Check existing identities
stellar keys list

# Create new identity if needed
stellar keys generate --global alice --network testnet --fund

# Verify identity has Lumens
stellar keys address alice
```

### **Step 2: Deploy to Testnet**

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/your-contract.wasm \
  --source alice \
  --network testnet \
  --alias my-contract
```

**Command Breakdown:**
- `--wasm`: Path to compiled contract
- `--source`: Identity paying for deployment
- `--network`: Target network (testnet)
- `--alias`: Friendly name for your contract

### **Step 3: Deployment Output**

```
ℹ️  Skipping install because wasm already installed
ℹ️  Using wasm hash abc123...
ℹ️  Simulating deploy transaction…
ℹ️  Transaction hash is def456...
🔗 https://stellar.expert/explorer/testnet/tx/def456...
ℹ️  Signing transaction: def456...
🌎 Submitting deploy transaction…
🔗 https://stellar.expert/explorer/testnet/contract/GHI789...
✅ Deployed!
GHI789...
```

**Important Information to Save:**
- **Contract ID**: `GHI789...` (needed for all interactions)
- **Transaction Hash**: `def456...` (for tracking)
- **Explorer Links**: For monitoring and verification

## Mainnet Deployment

### **⚠️ Mainnet Considerations**

**Before deploying to mainnet:**
- Contract thoroughly tested on testnet
- Security audit completed
- All edge cases handled
- Gas optimization complete
- Emergency procedures planned
- Legal compliance verified

### **Step 1: Prepare Mainnet Identity**

```bash
# Create mainnet identity (NO --fund flag)
stellar keys generate --global mainnet-alice --network mainnet

# Fund with real Lumens (minimum 1 XLM)
# Transfer from exchange or wallet
```

### **Step 2: Deploy to Mainnet**

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/your-contract.wasm \
  --source mainnet-alice \
  --network mainnet \
  --alias my-contract-production
```

**Mainnet Differences:**
- Higher transaction fees
- Real XLM costs
- Permanent deployment
- Real user interactions
- Higher security requirements

## Deployment Verification

### **Step 1: Verify on Explorer**

Visit the contract explorer link from deployment output:
```
https://stellar.expert/explorer/testnet/contract/GHI789...
```

**What to Verify:**
- Contract address matches deployment output
- Transaction history shows deployment
- Contract is active and callable
- No error messages or warnings

### **Step 2: Test Contract Interaction**

```bash
# Test basic functionality
stellar contract invoke \
  --id GHI789... \
  --source alice \
  --network testnet \
  -- \
  your_function \
  --param value
```

### **Step 3: Verify Contract State**

```bash
# Check contract state
stellar contract invoke \
  --id GHI789... \
  --source alice \
  --network testnet \
  -- \
  get_state
```

## Advanced Deployment Patterns

### **1. Contract Upgrades**

```rust
// In your contract
const ADMIN: Symbol = symbol_short!("ADMIN");

pub fn upgrade(env: Env, new_wasm_hash: Bytes) -> Result<(), Error> {
    let admin = env.storage().instance().get(&ADMIN)
        .ok_or(Error::AdminNotSet)?;
    
    if env.current_contract().invoker() != admin {
        return Err(Error::Unauthorized);
    }
    
    env.deployer().update_current_contract_wasm(&new_wasm_hash);
    Ok(())
}
```

**Upgrade Process:**
```bash
# 1. Deploy new contract version
stellar contract deploy --wasm new-version.wasm --source alice --network testnet

# 2. Call upgrade function on old contract
stellar contract invoke --id old-contract-id --source alice --network testnet -- upgrade --new_wasm_hash new-hash
```

### **2. Factory Pattern Deployment**

```rust
pub fn create_contract(env: Env, user: Address) -> Result<Address, Error> {
    let wasm_hash = env.deployer().upload_contract_wasm(&CONTRACT_WASM);
    let contract_id = env.deployer().with_current_contract(&env.current_contract_id())
        .create_contract(&wasm_hash);
    
    // Initialize the new contract
    let client = NewContractClient::new(&env, &contract_id);
    client.initialize(&user);
    
    Ok(contract_id)
}
```

### **3. Multi-Signature Deployment**

```bash
# Deploy with multiple signers
stellar contract deploy \
  --wasm contract.wasm \
  --source alice \
  --network testnet \
  --alias multi-sig-contract \
  --signers bob,carol
```

## Deployment Configuration

### **Environment-Specific Settings**

```rust
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Config {
    pub network: String,
    pub admin: Address,
    pub max_users: u32,
    pub fee_rate: u32,
}

const CONFIG: Symbol = symbol_short!("CONFIG");

pub fn initialize(env: Env, admin: Address) {
    let config = Config {
        network: env.current_contract().network().to_string(),
        admin,
        max_users: if env.current_contract().network() == "mainnet" { 10000 } else { 1000 },
        fee_rate: if env.current_contract().network() == "mainnet" { 100 } else { 10 },
    };
    
    env.storage().instance().set(&CONFIG, &config);
}
```

### **Network-Specific Constants**

```rust
// Different settings for different networks
const MAX_TRANSACTION_SIZE: u32 = if cfg!(feature = "mainnet") { 1000000 } else { 100000 };
const GAS_LIMIT: u32 = if cfg!(feature = "mainnet") { 10000000 } else { 1000000 };
```

## Deployment Scripts

### **Automated Deployment Script**

```bash
#!/bin/bash
# deploy.sh

set -e

CONTRACT_NAME="my-contract"
NETWORK=${1:-testnet}
IDENTITY=${2:-alice}

echo "🚀 Deploying $CONTRACT_NAME to $NETWORK..."

# Build contract
echo "📦 Building contract..."
stellar contract build

# Deploy contract
echo "🌐 Deploying to $NETWORK..."
DEPLOY_OUTPUT=$(stellar contract deploy \
  --wasm target/wasm32v1-none/release/$CONTRACT_NAME.wasm \
  --source $IDENTITY \
  --network $NETWORK \
  --alias $CONTRACT_NAME)

# Extract contract ID
CONTRACT_ID=$(echo "$DEPLOY_OUTPUT" | grep "✅ Deployed!" | awk '{print $2}')

echo "✅ Contract deployed successfully!"
echo "📋 Contract ID: $CONTRACT_ID"
echo "🔗 Explorer: https://stellar.expert/explorer/$NETWORK/contract/$CONTRACT_ID"

# Save contract ID to file
echo $CONTRACT_ID > .contract_id_$NETWORK
```

### **Usage:**
```bash
# Deploy to testnet
./deploy.sh testnet alice

# Deploy to mainnet
./deploy.sh mainnet mainnet-alice
```

## Post-Deployment Checklist

### **Immediate Actions**

- ✅ Save contract ID and transaction hash
- ✅ Verify contract on explorer
- ✅ Test basic functionality
- ✅ Verify contract state
- ✅ Check for any error logs

### **Documentation Updates**

- ✅ Update README with contract address
- ✅ Document deployment details
- ✅ Update frontend configuration
- ✅ Notify team members
- ✅ Update monitoring systems

### **Monitoring Setup**

```bash
# Monitor contract activity
stellar contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  get_stats

# Check contract balance
stellar contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  get_balance
```

## Troubleshooting Deployment

### **Common Issues**

#### **1. Insufficient Balance**
```bash
# Error: Insufficient balance
# Solution: Fund your identity
stellar keys generate --global alice --network testnet --fund
```

#### **2. Contract Build Failures**
```bash
# Error: Build failed
# Solution: Check Rust version and WASM target
rustup update
rustup target add wasm32v1-none
cargo clean
stellar contract build
```

#### **3. Network Connection Issues**
```bash
# Error: Network timeout
# Solution: Check internet connection and try again
stellar contract deploy --wasm contract.wasm --source alice --network testnet
```

#### **4. Contract Already Deployed**
```bash
# Error: Contract already exists
# Solution: Use different alias or remove old deployment
stellar contract deploy --wasm contract.wasm --source alice --network testnet --alias new-name
```

### **Recovery Procedures**

#### **Failed Deployment Recovery**
```bash
# 1. Check transaction status
stellar transaction show TRANSACTION_HASH --network testnet

# 2. If failed, retry deployment
stellar contract deploy --wasm contract.wasm --source alice --network testnet

# 3. If successful, verify contract
stellar contract invoke --id NEW_CONTRACT_ID --source alice --network testnet -- get_state
```

#### **Contract Rollback**
```bash
# 1. Deploy previous version
stellar contract deploy --wasm previous-version.wasm --source alice --network testnet

# 2. Update frontend to use new contract
# 3. Notify users of change
# 4. Investigate failure cause
```

## Security Considerations

### **Pre-Deployment Security**

- **Code Review**: Multiple developers review code
- **Security Audit**: Professional security audit
- **Test Coverage**: 90%+ test coverage
- **Gas Optimization**: Minimize attack surface
- **Access Control**: Proper permission systems

### **Post-Deployment Security**

- **Monitoring**: Monitor for suspicious activity
- **Emergency Procedures**: Plan for security incidents
- **Upgrade Path**: Plan for security updates
- **User Communication**: Transparent about issues
- **Incident Response**: Quick response to threats

## Cost Optimization

### **Deployment Costs**

| Network | Deployment Cost | Transaction Cost | Storage Cost |
|---------|----------------|------------------|--------------|
| **Testnet** | Free | Free | Free |
| **Mainnet** | ~0.01 XLM | ~0.00001 XLM | ~0.0001 XLM/KB |

### **Cost Reduction Strategies**

```rust
// 1. Minimize WASM size
#![no_std]  // Exclude standard library

// 2. Use efficient data types
const SMALL_CONSTANT: u8 = 255;  // Instead of u32

// 3. Batch operations
pub fn batch_update(env: Env, updates: Vec<(Address, u32)>) {
    for (user, amount) in updates {
        env.storage().instance().set(&(USER_BALANCE, user), &amount);
    }
}
```

## What's Next?

Now that you can deploy contracts, you're ready to:

1. **[Build frontend applications](frontend/README.md)** - Create user interfaces
2. **[Monitor production contracts](advanced/README.md)** - Keep contracts running smoothly
3. **[Scale your applications](advanced/README.md)** - Handle growth and optimization
4. **[Contribute to the ecosystem](resources/README.md)** - Help others deploy

Remember: Deployment is just the beginning. Monitor your contracts, gather user feedback, and continuously improve!

---

**Next**: [Frontend Integration](frontend/README.md) - Building user interfaces for your contracts
