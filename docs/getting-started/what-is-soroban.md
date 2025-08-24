# What is Soroban?

Soroban is Stellar's smart contract platform—think of it as the "programmable layer" that sits on top of the Stellar blockchain. While Stellar handles the basic financial transactions, Soroban allows developers to create complex, automated financial applications.

## The Analogy

Imagine Stellar as a highway system:
- **Stellar Core**: The roads, traffic lights, and basic infrastructure
- **Soroban**: The cars, delivery trucks, and automated vehicles that use those roads

Soroban gives you the tools to build "vehicles" (smart contracts) that can automatically execute financial logic on the Stellar network.

## What Makes Soroban Special?

### **1. Rust-Based Development**
Soroban smart contracts are written in Rust, a modern programming language known for:
- **Memory Safety**: Prevents common programming errors
- **Performance**: Fast execution with minimal overhead
- **Reliability**: Used in production systems like Firefox and Dropbox
- **Developer Experience**: Excellent tooling and error messages

### **2. WebAssembly (WASM)**
Soroban contracts compile to WebAssembly, which means:
- **Cross-Platform**: Runs on any device that supports WASM
- **Fast**: Near-native performance
- **Secure**: Sandboxed execution environment
- **Efficient**: Small file sizes and quick loading

### **3. Stellar Integration**
Unlike other smart contract platforms, Soroban is deeply integrated with Stellar:
- **Native Asset Support**: Works seamlessly with Stellar's built-in assets
- **Fast Settlement**: 3-5 second transaction finality
- **Low Costs**: Predictable, minimal transaction fees
- **Regulatory Compliance**: Built-in features for financial regulations

## How Soroban Works

### **Contract Lifecycle**

1. **Development**: Write your contract in Rust
2. **Compilation**: Convert to WebAssembly (WASM)
3. **Deployment**: Upload to the Stellar network
4. **Execution**: Users interact with your contract
5. **State Management**: Contract maintains data on-chain

### **Execution Model**

```
User Request → Soroban Runtime → Smart Contract → Stellar Network
     ↑              ↓              ↓              ↓
Response ←    Result Processing ← State Changes ← Transaction
```

## What Can You Build with Soroban?

### **Financial Applications**
- **Automated Trading**: Bots that execute trades based on market conditions
- **Lending Platforms**: Smart contracts that manage loans and interest
- **Insurance**: Automated claims processing and payout systems
- **Derivatives**: Complex financial instruments with automated settlement

### **DeFi (Decentralized Finance)**
- **DEXs**: Decentralized exchanges for trading assets
- **Yield Farming**: Automated strategies for earning returns
- **Staking**: Locking assets to earn rewards
- **Governance**: Voting systems for protocol decisions

### **Business Applications**
- **Supply Chain**: Tracking goods and automating payments
- **Real Estate**: Fractional ownership and automated rent collection
- **Gaming**: In-game economies and asset ownership
- **Identity**: Verifiable credentials and access control

## Soroban vs. Other Smart Contract Platforms

| Feature | Soroban | Ethereum | Solana |
|---------|---------|----------|---------|
| **Language** | Rust | Solidity | Rust |
| **Execution** | WASM | EVM | Native |
| **Speed** | 3-5s | 15s+ | <1s |
| **Cost** | Very Low | High | Low |
| **Learning Curve** | Moderate | Steep | Moderate |
| **Stellar Integration** | Native | None | None |

## Key Soroban Concepts

### **Contracts**
- Self-executing programs that run on the blockchain
- Can store data, process logic, and interact with other contracts
- Immutable once deployed (can be upgraded through specific patterns)

### **Functions**
- Public methods that users can call
- Can read and modify contract state
- Execute business logic automatically

### **State Management**
- Contracts can store persistent data
- Data is stored on-chain and accessible to all users
- Efficient storage with automatic cleanup

### **Events**
- Notifications that contracts can emit
- Useful for frontend applications to track contract activity
- Stored on-chain for transparency

## Development Workflow

### **1. Write Contract**
```rust
#[contract]
pub struct MyContract;

#[contractimpl]
impl MyContract {
    pub fn hello(env: Env, name: String) -> String {
        format!("Hello, {}!", name)
    }
}
```

### **2. Build & Test**
```bash
stellar contract build
cargo test
```

### **3. Deploy**
```bash
stellar contract deploy --wasm target/.../contract.wasm
```

### **4. Interact**
```bash
stellar contract invoke --id <contract-id> --fn hello --name "World"
```

## Why Choose Soroban?

### **For Developers**
- **Modern Language**: Rust is safer and more performant than older alternatives
- **Great Tooling**: Excellent development experience with comprehensive tooling
- **Active Community**: Growing ecosystem with helpful resources
- **Future-Proof**: Built on proven WebAssembly technology

### **For Businesses**
- **Regulatory Friendly**: Built-in compliance features
- **Cost Effective**: Much lower transaction costs than alternatives
- **Enterprise Ready**: Used by major companies and institutions
- **Scalable**: Can handle high transaction volumes

### **For Users**
- **Fast**: Transactions complete in seconds
- **Cheap**: Minimal fees for all operations
- **Reliable**: 99.9%+ uptime with robust infrastructure
- **Accessible**: Works on any device with internet access

## Getting Started with Soroban

Now that you understand what Soroban is, you're ready to:
1. [Set up your development environment](environment-setup.md)
2. [Write your first smart contract](smart-contracts/your-first-contract.md)
3. [Deploy and test your contract](smart-contracts/deploying-contracts.md)

Soroban makes Stellar programmable, opening up endless possibilities for financial innovation. Whether you're building the next big DeFi protocol or a simple automation tool, Soroban provides the foundation you need.

---

**Next**: [Prerequisites](prerequisites.md) - What you need to know before starting development
