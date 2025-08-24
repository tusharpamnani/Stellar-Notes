# CLI Commands

Complete reference for all Stellar CLI commands. Use this as a quick lookup while building and deploying your smart contracts.

## Command Overview

The Stellar CLI (`stellar`) provides commands for:
- **Contract Management** - Build, deploy, and interact with contracts
- **Key Management** - Create and manage identities
- **Network Operations** - Connect to different networks
- **Configuration** - Set up your development environment

## Basic Command Structure

```bash
stellar <command> [subcommand] [options] [arguments]
```

### **Global Options**
```bash
--help, -h          # Show help for command
--version, -v       # Show version information
--verbose           # Enable verbose output
--quiet             # Suppress output
--config <file>     # Use custom config file
```

## Contract Commands

### **Build Contract**

```bash
stellar contract build [options]
```

**Purpose**: Compile Rust contract to WebAssembly

**Options**:
```bash
--target <target>           # WASM target (default: wasm32v1-none)
--release                   # Build in release mode
--debug                     # Build in debug mode
--out-dir <directory>       # Output directory
--contract-name <name>      # Contract name
```

**Examples**:
```bash
# Basic build
stellar contract build

# Build with specific target
stellar contract build --target wasm32v1-none

# Build in release mode
stellar contract build --release
```

### **Deploy Contract**

```bash
stellar contract deploy [options]
```

**Purpose**: Deploy compiled contract to network

**Options**:
```bash
--wasm <file>              # Path to WASM file
--source <identity>         # Source identity
--network <network>         # Target network (testnet/mainnet)
--alias <name>              # Contract alias
--fee <amount>              # Transaction fee
--timeout <seconds>         # Transaction timeout
```

**Examples**:
```bash
# Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source alice \
  --network testnet \
  --alias my-contract

# Deploy to mainnet
stellar contract deploy \
  --wasm contract.wasm \
  --source mainnet-alice \
  --network mainnet \
  --alias production-contract
```

### **Invoke Contract**

```bash
stellar contract invoke [options] -- <function> [parameters]
```

**Purpose**: Call contract functions

**Options**:
```bash
--id <contract-id>          # Contract ID
--source <identity>          # Source identity
--network <network>          # Network
--send=yes                  # Execute transaction
--fee <amount>              # Transaction fee
--timeout <seconds>         # Transaction timeout
```

**Examples**:
```bash
# Simulate function call
stellar contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  hello \
  --to World

# Execute transaction
stellar contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  --send=yes \
  -- \
  increment

# Call with parameters
stellar contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  set_value \
  --value 42
```

### **Generate Bindings**

```bash
stellar contract bindings [language] [options]
```

**Purpose**: Generate client code for contracts

**Options**:
```bash
--network <network>         # Network
--contract-id <id>          # Contract ID
--output-dir <directory>    # Output directory
--format <format>           # Output format
```

**Examples**:
```bash
# Generate TypeScript bindings
stellar contract bindings typescript \
  --network testnet \
  --contract-id CONTRACT_ID \
  --output-dir src/contracts

# Generate JavaScript bindings
stellar contract bindings javascript \
  --network testnet \
  --contract-id CONTRACT_ID \
  --output-dir src/contracts
```

## Key Management Commands

### **Generate Keys**

```bash
stellar keys generate [options]
```

**Purpose**: Create new keypair

**Options**:
```bash
--global <name>             # Global identity name
--network <network>         # Network for identity
--fund                      # Request funding from Friendbot
--output-format <format>    # Output format (json/text)
```

**Examples**:
```bash
# Create testnet identity with funding
stellar keys generate --global alice --network testnet --fund

# Create mainnet identity (no funding)
stellar keys generate --global mainnet-alice --network mainnet

# Create multiple identities
stellar keys generate --global bob --network testnet --fund
stellar keys generate --global carol --network testnet --fund
```

### **List Keys**

```bash
stellar keys list [options]
```

**Purpose**: List all identities

**Options**:
```bash
--network <network>         # Filter by network
--output-format <format>    # Output format
```

**Examples**:
```bash
# List all identities
stellar keys list

# List testnet identities only
stellar keys list --network testnet

# List in JSON format
stellar keys list --output-format json
```

### **Show Key Address**

```bash
stellar keys address <identity> [options]
```

**Purpose**: Show public key for identity

**Options**:
```bash
--network <network>         # Network
--output-format <format>    # Output format
```

**Examples**:
```bash
# Show alice's address
stellar keys address alice

# Show in different format
stellar keys address alice --output-format json
```

### **Remove Key**

```bash
stellar keys remove <identity> [options]
```

**Purpose**: Remove identity

**Options**:
```bash
--network <network>         # Network
--force                     # Force removal without confirmation
```

**Examples**:
```bash
# Remove identity
stellar keys remove alice

# Force remove
stellar keys remove alice --force
```

## Network Commands

### **Show Network Info**

```bash
stellar network [subcommand] [options]
```

**Purpose**: Network information and configuration

**Subcommands**:
```bash
stellar network show         # Show current network
stellar network list         # List available networks
stellar network switch       # Switch networks
```

**Examples**:
```bash
# Show current network
stellar network show

# List all networks
stellar network list

# Switch to testnet
stellar network switch testnet
```

### **Configure Network**

```bash
stellar config [subcommand] [options]
```

**Purpose**: Configure CLI settings

**Subcommands**:
```bash
stellar config set <key> <value>     # Set configuration
stellar config get <key>              # Get configuration
stellar config list                   # List all settings
stellar config reset                  # Reset to defaults
```

**Examples**:
```bash
# Set default network
stellar config set network testnet

# Set RPC timeout
stellar config set rpc.timeout 30000

# Show all config
stellar config list
```

## Transaction Commands

### **Show Transaction**

```bash
stellar transaction show <hash> [options]
```

**Purpose**: Show transaction details

**Options**:
```bash
--network <network>         # Network
--output-format <format>    # Output format
```

**Examples**:
```bash
# Show transaction
stellar transaction show TRANSACTION_HASH

# Show in JSON format
stellar transaction show TRANSACTION_HASH --output-format json
```

### **Submit Transaction**

```bash
stellar transaction submit [options]
```

**Purpose**: Submit raw transaction

**Options**:
```bash
--transaction <file>        # Transaction file
--network <network>         # Network
--timeout <seconds>         # Timeout
```

**Examples**:
```bash
# Submit transaction from file
stellar transaction submit --transaction tx.xdr --network testnet
```

## Utility Commands

### **Version Information**

```bash
stellar --version
stellar -v
```

**Purpose**: Show CLI version

### **Help**

```bash
stellar --help
stellar -h
stellar <command> --help
```

**Purpose**: Show help information

### **Completion**

```bash
stellar completion [shell]
```

**Purpose**: Generate shell completion scripts

**Examples**:
```bash
# Generate bash completion
stellar completion bash

# Generate zsh completion
stellar completion zsh
```

## Common Command Patterns

### **Development Workflow**

```bash
# 1. Build contract
stellar contract build

# 2. Deploy to testnet
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source alice \
  --network testnet \
  --alias dev-contract

# 3. Generate bindings
stellar contract bindings typescript \
  --network testnet \
  --contract-id DEPLOYED_CONTRACT_ID \
  --output-dir src/contracts

# 4. Test contract
stellar contract invoke \
  --id DEPLOYED_CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  test_function
```

### **Production Deployment**

```bash
# 1. Build optimized contract
stellar contract build --release

# 2. Deploy to mainnet
stellar contract deploy \
  --wasm target/wasm32v1-none/release/contract.wasm \
  --source mainnet-alice \
  --network mainnet \
  --alias production-contract

# 3. Verify deployment
stellar contract invoke \
  --id DEPLOYED_CONTRACT_ID \
  --source mainnet-alice \
  --network mainnet \
  -- \
  get_status
```

### **Testing and Debugging**

```bash
# 1. Test on testnet
stellar contract invoke \
  --id CONTRACT_ID \
  --source alice \
  --network testnet \
  -- \
  test_function

# 2. Check transaction
stellar transaction show TRANSACTION_HASH --network testnet

# 3. Generate bindings for frontend
stellar contract bindings typescript \
  --network testnet \
  --contract-id CONTRACT_ID \
  --output-dir src/contracts
```

## Environment Variables

### **Configuration Variables**

```bash
STELLAR_NETWORK           # Default network
STELLAR_RPC_URL          # RPC endpoint
STELLAR_HORIZON_URL      # Horizon endpoint
STELLAR_TIMEOUT          # Default timeout
STELLAR_VERBOSE          # Verbose output
```

### **Usage Examples**

```bash
# Set environment variables
export STELLAR_NETWORK=testnet
export STELLAR_RPC_URL=https://soroban-testnet.stellar.org:443
export STELLAR_TIMEOUT=30000

# Use in commands
stellar contract deploy --wasm contract.wasm --source alice
```

## Error Codes and Troubleshooting

### **Common Error Messages**

| Error | Cause | Solution |
|-------|-------|----------|
| `insufficient balance` | Account has no XLM | Fund account with Friendbot or transfer XLM |
| `contract not found` | Invalid contract ID | Verify contract ID and network |
| `unauthorized` | Wrong identity | Use correct source identity |
| `network error` | Connection issues | Check internet and RPC endpoint |
| `invalid wasm` | Corrupted WASM file | Rebuild contract |

### **Debugging Commands**

```bash
# Enable verbose output
stellar --verbose contract invoke --id CONTRACT_ID --source alice --network testnet -- test_function

# Check network status
stellar network show

# Verify identity
stellar keys address alice

# Test RPC connection
curl -X POST https://soroban-testnet.stellar.org:443 -H "Content-Type: application/json" -d '{"jsonrpc":"2.0","id":1,"method":"rpc.discover"}'
```

## Best Practices

### **Command Organization**

1. **Use Aliases**: Give contracts meaningful names
2. **Network Separation**: Use different identities for different networks
3. **Version Control**: Keep track of deployed contract versions
4. **Documentation**: Document your deployment commands

### **Security Considerations**

1. **Secure Identities**: Keep private keys secure
2. **Network Isolation**: Test thoroughly on testnet before mainnet
3. **Access Control**: Use appropriate source identities
4. **Verification**: Always verify contract deployments

### **Performance Tips**

1. **Optimize Builds**: Use `--release` for production
2. **Batch Operations**: Group related operations
3. **Network Selection**: Use appropriate network for your use case
4. **Timeout Settings**: Set appropriate timeouts for your network

## What's Next?

Now that you understand the CLI commands, you're ready to:

1. **[Use the Rust SDK](rust-sdk.md)** - Programmatic contract interaction
2. **[Configure networks](network-config.md)** - Set up different environments
3. **[Build production apps](advanced/README.md)** - Deploy secure applications

Remember: The CLI is your primary tool for contract development. Practice these commands regularly to become proficient!

---

**Next**: [Rust SDK](rust-sdk.md) - Soroban SDK reference and examples
