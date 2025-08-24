# Examples & Tutorials

Welcome to the Examples & Tutorials section! Here you'll find hands-on projects that demonstrate real-world Stellar development patterns. Each example builds on the previous ones, helping you understand increasingly complex concepts.

## What You'll Find Here

- **Complete Working Projects** - Full implementations you can run and modify
- **Step-by-Step Explanations** - Detailed breakdowns of how each example works
- **Best Practices** - Production-ready patterns and techniques
- **Common Use Cases** - Real-world scenarios you'll encounter

## Example Projects

### **1. [Hello World Contract](hello-world.md)**
**Difficulty**: Beginner  
**Time**: 30-45 minutes  
**Concepts**: Basic contract structure, deployment, interaction

A simple greeting contract that demonstrates:
- Basic contract architecture
- Function parameters and return values
- Deployment workflow
- Contract interaction

**What you'll learn**: How to write, build, test, and deploy your first smart contract.

### **2. [Increment Counter](increment-counter.md)**
**Difficulty**: Beginner-Intermediate  
**Time**: 45-60 minutes  
**Concepts**: State management, persistence, data storage

A counter contract that shows:
- How to store data on the blockchain
- State persistence between calls
- Basic data manipulation
- Contract lifecycle management

**What you'll learn**: How contracts maintain state and persist data across transactions.

### **3. [Token Contract](token-contract.md)**
**Difficulty**: Intermediate  
**Time**: 1-2 hours  
**Concepts**: Asset creation, transfer logic, access control

A complete token implementation featuring:
- Token minting and burning
- Transfer functionality
- Balance tracking
- Access control patterns

**What you'll learn**: How to create and manage digital assets on Stellar.

## How to Use These Examples

### **Learning Path**
1. **Start with Hello World** - Get comfortable with the basics
2. **Move to Increment Counter** - Learn about state management
3. **Tackle Token Contract** - Understand complex financial logic

### **Hands-On Approach**
- **Don't just read** - Actually build each example
- **Modify the code** - Experiment with changes
- **Break things** - Learn from errors and debugging
- **Build upon** - Use examples as starting points for your own projects

### **Code Along**
Each example includes:
- Complete source code
- Line-by-line explanations
- Testing instructions
- Deployment steps
- Common issues and solutions

## Project Structure

All examples follow a consistent structure:

```
example-name/
├── Cargo.toml          # Project configuration
├── contracts/
│   └── contract-name/
│       ├── Cargo.toml  # Contract dependencies
│       ├── src/
│       │   ├── lib.rs  # Contract implementation
│       │   └── test.rs # Tests
│       └── Makefile    # Build commands
└── README.md           # Detailed documentation
```

## Prerequisites

Before diving into these examples, make sure you have:
- ✅ [Environment setup complete](getting-started/environment-setup.md)
- ✅ [First contract tutorial completed](smart-contracts/your-first-contract.md)
- ✅ Basic understanding of Rust syntax
- ✅ Familiarity with Stellar CLI commands

## Customization Ideas

### **Hello World Variations**
- Add multiple greeting functions
- Implement different languages
- Add time-based greetings
- Create personalized responses

### **Counter Enhancements**
- Add decrement functionality
- Implement reset capability
- Add maximum/minimum bounds
- Create multiple counters

### **Token Extensions**
- Add metadata (name, symbol, decimals)
- Implement approval/allowance system
- Add minting restrictions
- Create upgradeable contracts

## Testing Your Examples

### **Local Testing**
```bash
cd contracts/contract-name
cargo test
```

### **Testnet Deployment**
```bash
stellar contract deploy --wasm target/.../contract.wasm --source alice --network testnet
```

### **Interaction Testing**
```bash
stellar contract invoke --id <contract-id> --source alice --network testnet -- <function> --<param> <value>
```

## Troubleshooting

### **Common Issues**
- **Build errors**: Check Rust version and WASM target
- **Deployment failures**: Verify identity has sufficient balance
- **Function errors**: Double-check parameter names and types
- **Network issues**: Ensure you're on the correct network (testnet)

### **Getting Help**
- Check the [Stellar Discord](https://discord.gg/stellar)
- Review [official documentation](https://soroban.stellar.org/docs)
- Search [GitHub issues](https://github.com/stellar/soroban-examples)

## Beyond the Examples

### **Next Steps**
After completing these examples, explore:
- [Advanced Topics](advanced/README.md) - Security, optimization, testing
- [Frontend Integration](frontend/README.md) - Building user interfaces
- [Production Deployment](reference/network-config.md) - Mainnet considerations

### **Building Your Own Projects**
Use these examples as templates for:
- DeFi protocols
- NFT marketplaces
- Gaming platforms
- Supply chain applications
- Identity systems

## Contributing

### **Improving Examples**
- Found a bug? Report it!
- Have a better approach? Share it!
- Want to add examples? Contribute!

### **Sharing Your Work**
- Build upon these examples
- Share your modifications
- Help other developers learn
- Join the Stellar community

## Summary

These examples provide a solid foundation for Stellar development. By working through them:

- You'll understand core concepts
- You'll gain practical experience
- You'll learn best practices
- You'll be ready for real-world projects

Remember: The best way to learn is by doing. Don't just read the code—build it, modify it, break it, and fix it!

---

**Start with**: [Hello World Contract](hello-world.md) - Your first step into smart contract development
