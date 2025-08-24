# Environment Setup

Now it's time to set up your development environment! This guide will walk you through installing all the tools you need to build on Stellar. We'll cover Windows, macOS, and Linux.

## What We're Installing

1. **Rust** - Programming language for smart contracts
2. **Stellar CLI** - Command-line tools for Stellar development
3. **Git** - Version control (if not already installed)
4. **Text Editor** - For writing code

## Step 1: Install Rust

Rust is the programming language used for Soroban smart contracts. It's known for safety, performance, and excellent tooling.

### **Install Rust**

Visit [https://www.rust-lang.org/](https://www.rust-lang.org/) and click "Get Started" or run this command in your terminal:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

**For Windows users**: Download the installer from the Rust website.

### **Verify Installation**

After installation, restart your terminal and verify Rust is installed:

```bash
rustc --version
cargo --version
```

You should see version numbers like `rustc 1.75.0` and `cargo 1.75.0`.

### **Add WebAssembly Target**

Soroban contracts compile to WebAssembly (WASM). Add the appropriate target:

**For Rust v1.85.0 or higher:**
```bash
rustup target add wasm32v1-none
```

**For older Rust versions:**
```bash
rustup target add wasm32-unknown-unknown
```

## Step 2: Install Stellar CLI

The Stellar CLI (`stellar`) is your main tool for building, deploying, and interacting with smart contracts.

### **Windows Installation**

#### Option 1: Using winget (Recommended)
```bash
winget install --id Stellar.StellarCLI --version 22.8.1
```

#### Option 2: Using Cargo
```bash
cargo install --locked stellar-cli@22.8.1 --features opt
```

### **macOS Installation**

#### Option 1: Using Homebrew (Recommended)
```bash
brew install stellar-cli
```

#### Option 2: Using Cargo
```bash
cargo install --locked stellar-cli@22.8.1 --features opt
```

### **Linux Installation**

#### Option 1: Using Cargo (Recommended)
```bash
cargo install --locked stellar-cli@22.8.1 --features opt
```

#### Option 2: Using Package Manager
Some distributions have packages available:
```bash
# Ubuntu/Debian (if available)
sudo apt install stellar-cli

# Arch Linux (if available)
yay -S stellar-cli
```

### **Verify Stellar CLI Installation**

```bash
stellar --version
```

You should see a version number like `stellar 22.8.1`.

## Step 3: Install Git (if needed)

Git is used for version control and downloading example projects.

### **Check if Git is installed**
```bash
git --version
```

### **Install Git if needed**

**Windows**: Download from [https://git-scm.com/](https://git-scm.com/)

**macOS**: 
```bash
brew install git
```

**Linux**:
```bash
# Ubuntu/Debian
sudo apt install git

# CentOS/RHEL
sudo yum install git
```

## Step 4: Choose a Text Editor

You'll need a text editor for writing code. Here are some recommendations:

### **VS Code (Recommended for beginners)**
- Free and open-source
- Excellent Rust support
- Integrated terminal
- Extensions for blockchain development

**Download**: [https://code.visualstudio.com/](https://code.visualstudio.com/)

**Useful Extensions**:
- Rust Analyzer
- WebAssembly
- GitLens
- Stellar

### **Alternative Options**
- **Sublime Text**: Fast and lightweight
- **Vim/Neovim**: For experienced users
- **IntelliJ IDEA**: Full-featured IDE (paid)
- **Atom**: Simple and extensible

## Step 5: Configure Stellar CLI for Testnet

By default, Stellar CLI operates on the testnet, which is perfect for learning and development.

### **Create Your First Identity**

An identity is a keypair that represents you on the network. Create one for testing:

```bash
stellar keys generate --global alice --network testnet --fund
```

**What this does**:
- `--global alice`: Creates an identity named "alice" that you can reuse
- `--network testnet`: Ensures it's created on the test network
- `--fund`: Requests testnet Lumens from Friendbot (free test tokens)

### **View Your Identity**

```bash
stellar keys address alice
```

This shows your public key (address) that you'll use for receiving tokens and deploying contracts.

### **Create Additional Identities (Optional)**

You can create more identities for testing different scenarios:

```bash
stellar keys generate --global bob --network testnet --fund
stellar keys generate --global carol --network testnet --fund
```

## Step 6: Verify Your Setup

Let's make sure everything is working correctly:

### **Test Rust Compilation**
```bash
cargo new test-project
cd test-project
cargo build
```

### **Test Stellar CLI**
```bash
stellar keys list
```

### **Test WebAssembly Target**
```bash
cd test-project
cargo build --target wasm32v1-none --release
```

## Troubleshooting Common Issues

### **"Command not found" errors**
- Make sure you've restarted your terminal after installation
- Check that the tools are in your system PATH
- On Windows, you may need to restart your computer

### **Permission errors on Linux/macOS**
- Use `sudo` for system-wide installations
- Or install Rust using rustup (recommended)

### **Rust version conflicts**
- Use `rustup default stable` to set the stable version
- Check for multiple Rust installations

### **WebAssembly target issues**
- Ensure you're using the correct target for your Rust version
- Try removing and re-adding the target: `rustup target remove wasm32v1-none && rustup target add wasm32v1-none`

## What's Next?

Congratulations! You now have a fully configured Stellar development environment. You can:

1. **Build your first contract** - Move to [Your First Contract](smart-contracts/your-first-contract.md)
2. **Explore the examples** - Check out the projects in this workspace
3. **Join the community** - Connect with other developers on Discord

## Quick Reference Commands

```bash
# Check versions
rustc --version
cargo --version
stellar --version

# Create identity
stellar keys generate --global <name> --network testnet --fund

# View identity
stellar keys address <name>

# List identities
stellar keys list

# Build contract
stellar contract build

# Deploy contract
stellar contract deploy --wasm <path> --source <identity>
```

---

**Next**: [Your First Contract](smart-contracts/your-first-contract.md) - Building and deploying your first smart contract
