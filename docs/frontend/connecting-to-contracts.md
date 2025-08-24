# Connecting to Contracts

Connecting your frontend to smart contracts is where the magic happens—this is where users can actually interact with your blockchain applications. This section will teach you how to set up contract clients, establish connections, and handle the communication between web applications and the Stellar network.

## Overview of Frontend-Contract Communication

### **How It Works**

```
Frontend (JavaScript) → Contract Client → Stellar Network → Smart Contract
      ↑                      ↓              ↓              ↓
User Interface ←     Response Handling ← Transaction ← Contract Execution
```

### **Key Components**

1. **Contract Client**: JavaScript object that represents your smart contract
2. **RPC Connection**: Connection to the Stellar network
3. **Transaction Building**: Creating blockchain transactions
4. **Response Handling**: Processing contract responses
5. **Error Management**: Handling failures gracefully

## Prerequisites

### **Before Starting**

- ✅ [Smart contract deployed](smart-contracts/deploying-contracts.md)
- ✅ Contract ID from deployment
- ✅ Basic web development knowledge (HTML, CSS, JavaScript)
- ✅ Understanding of async programming

### **Required Tools**

- **Web Browser**: Chrome, Firefox, Safari, or Edge
- **Text Editor**: VS Code, Sublime Text, or similar
- **Local Server**: For testing (VS Code Live Server, Python, etc.)
- **Stellar CLI**: For generating TypeScript bindings

## Step 1: Generate TypeScript Bindings

### **What Are TypeScript Bindings?**

TypeScript bindings are automatically generated code that provides:
- **Type Safety**: TypeScript interfaces for your contract functions
- **Client Class**: JavaScript class for interacting with your contract
- **Function Signatures**: Proper parameter types and return types
- **Error Handling**: Typed error responses

### **Generate Bindings**

```bash
# Navigate to your project
cd your-project-directory

# Generate TypeScript bindings
stellar contract bindings typescript \
  --network testnet \
  --contract-id YOUR_CONTRACT_ID \
  --output-dir src/contracts
```

**Command Breakdown:**
- `--network`: Which network your contract is on
- `--contract-id`: Your deployed contract's ID
- `--output-dir`: Where to save the generated files

### **Generated Files**

After running the command, you'll get:
```
src/contracts/
├── index.ts          # Main export file
├── client.ts         # Contract client class
├── types.ts          # TypeScript type definitions
└── constants.ts      # Contract constants
```

## Step 2: Set Up Your Frontend Project

### **Project Structure**

```
my-frontend/
├── index.html
├── styles.css
├── app.js
├── src/
│   └── contracts/    # Generated TypeScript bindings
└── package.json
```

### **Basic HTML Setup**

```html
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Stellar Contract Frontend</title>
    <link rel="stylesheet" href="styles.css">
</head>
<body>
    <div class="container">
        <h1>Stellar Contract Interface</h1>
        
        <div class="contract-section">
            <h2>Contract Status</h2>
            <div id="contract-status">Connecting...</div>
        </div>
        
        <div class="interaction-section">
            <h2>Interact with Contract</h2>
            <button id="increment-btn">Increment Counter</button>
            <button id="get-counter-btn">Get Counter</button>
            <div id="counter-display">Counter: --</div>
        </div>
        
        <div class="transaction-section">
            <h2>Transaction History</h2>
            <div id="transaction-log"></div>
        </div>
    </div>
    
    <script src="app.js"></script>
</body>
</html>
```

### **Basic CSS Styling**

```css
.container {
    max-width: 800px;
    margin: 0 auto;
    padding: 20px;
    font-family: Arial, sans-serif;
}

.contract-section, .interaction-section, .transaction-section {
    margin: 20px 0;
    padding: 20px;
    border: 1px solid #ddd;
    border-radius: 8px;
}

button {
    background-color: #007bff;
    color: white;
    border: none;
    padding: 10px 20px;
    margin: 5px;
    border-radius: 4px;
    cursor: pointer;
}

button:hover {
    background-color: #0056b3;
}

button:disabled {
    background-color: #6c757d;
    cursor: not-allowed;
}

#transaction-log {
    max-height: 200px;
    overflow-y: auto;
    background-color: #f8f9fa;
    padding: 10px;
    border-radius: 4px;
}
```

## Step 3: Initialize Contract Client

### **Basic Contract Client Setup**

```javascript
// app.js
class ContractInterface {
    constructor() {
        this.contract = null;
        this.contractId = 'YOUR_CONTRACT_ID_HERE';
        this.network = 'testnet';
        this.rpcUrl = 'https://soroban-testnet.stellar.org:443';
        
        this.init();
    }
    
    async init() {
        try {
            // Import the generated contract client
            const { Client, networks } = await import('./src/contracts/index.js');
            
            // Create contract client
            this.contract = new Client({
                ...networks.testnet,
                rpcUrl: this.rpcUrl
            });
            
            // Update UI
            this.updateStatus('Connected to contract');
            this.enableButtons();
            
        } catch (error) {
            console.error('Failed to initialize contract:', error);
            this.updateStatus('Failed to connect to contract');
        }
    }
    
    updateStatus(message) {
        document.getElementById('contract-status').textContent = message;
    }
    
    enableButtons() {
        document.getElementById('increment-btn').disabled = false;
        document.getElementById('get-counter-btn').disabled = false;
    }
    
    logTransaction(message) {
        const log = document.getElementById('transaction-log');
        const timestamp = new Date().toLocaleTimeString();
        log.innerHTML += `<div>[${timestamp}] ${message}</div>`;
        log.scrollTop = log.scrollHeight;
    }
}

// Initialize when page loads
document.addEventListener('DOMContentLoaded', () => {
    window.contractInterface = new ContractInterface();
});
```

### **Advanced Contract Client Setup**

```javascript
class AdvancedContractInterface {
    constructor() {
        this.contract = null;
        this.contractId = 'YOUR_CONTRACT_ID_HERE';
        this.network = 'testnet';
        this.rpcUrl = 'https://soroban-testnet.stellar.org:443';
        this.retryAttempts = 3;
        this.retryDelay = 1000;
        
        this.init();
    }
    
    async init() {
        try {
            // Import contract client
            const { Client, networks } = await import('./src/contracts/index.js');
            
            // Create client with advanced options
            this.contract = new Client({
                ...networks.testnet,
                rpcUrl: this.rpcUrl,
                timeout: 30000, // 30 second timeout
                retryAttempts: this.retryAttempts,
                retryDelay: this.retryDelay
            });
            
            // Test connection
            await this.testConnection();
            
            this.updateStatus('Connected to contract');
            this.enableButtons();
            
        } catch (error) {
            console.error('Failed to initialize contract:', error);
            this.updateStatus(`Connection failed: ${error.message}`);
            this.scheduleRetry();
        }
    }
    
    async testConnection() {
        // Try to call a simple read function
        try {
            await this.contract.getCounter();
        } catch (error) {
            throw new Error(`Contract connection test failed: ${error.message}`);
        }
    }
    
    scheduleRetry() {
        setTimeout(() => {
            this.updateStatus('Retrying connection...');
            this.init();
        }, 5000);
    }
    
    // ... rest of the class
}
```

## Step 4: Implement Contract Functions

### **Read-Only Functions (Simulation)**

```javascript
async getCounter() {
    try {
        this.logTransaction('Getting counter value...');
        
        // Call contract function (simulation)
        const result = await this.contract.getCounter();
        
        // Update UI
        document.getElementById('counter-display').textContent = `Counter: ${result}`;
        this.logTransaction(`Counter value: ${result}`);
        
        return result;
        
    } catch (error) {
        console.error('Failed to get counter:', error);
        this.logTransaction(`Error getting counter: ${error.message}`);
        throw error;
    }
}
```

### **State-Changing Functions (Transaction)**

```javascript
async incrementCounter() {
    try {
        this.logTransaction('Incrementing counter...');
        
        // Disable button during transaction
        const button = document.getElementById('increment-btn');
        button.disabled = true;
        button.textContent = 'Processing...';
        
        // Call contract function (actual transaction)
        const result = await this.contract.increment();
        
        // Update UI
        document.getElementById('counter-display').textContent = `Counter: ${result}`;
        this.logTransaction(`Counter incremented to: ${result}`);
        
        return result;
        
    } catch (error) {
        console.error('Failed to increment counter:', error);
        this.logTransaction(`Error incrementing counter: ${error.message}`);
        throw error;
        
    } finally {
        // Re-enable button
        const button = document.getElementById('increment-btn');
        button.disabled = false;
        button.textContent = 'Increment Counter';
    }
}
```

### **Function with Parameters**

```javascript
async setCounter(value) {
    try {
        this.logTransaction(`Setting counter to: ${value}`);
        
        // Validate input
        if (typeof value !== 'number' || value < 0) {
            throw new Error('Invalid counter value');
        }
        
        // Call contract function
        const result = await this.contract.setCounter({ value });
        
        // Update UI
        document.getElementById('counter-display').textContent = `Counter: ${value}`;
        this.logTransaction(`Counter set to: ${value}`);
        
        return result;
        
    } catch (error) {
        console.error('Failed to set counter:', error);
        this.logTransaction(`Error setting counter: ${error.message}`);
        throw error;
    }
}
```

## Step 5: Handle User Interactions

### **Button Event Listeners**

```javascript
setupEventListeners() {
    // Increment button
    document.getElementById('increment-btn').addEventListener('click', async () => {
        try {
            await this.incrementCounter();
        } catch (error) {
            this.showError('Failed to increment counter');
        }
    });
    
    // Get counter button
    document.getElementById('get-counter-btn').addEventListener('click', async () => {
        try {
            await this.getCounter();
        } catch (error) {
            this.showError('Failed to get counter value');
        }
    });
    
    // Form submission
    document.getElementById('counter-form').addEventListener('submit', async (e) => {
        e.preventDefault();
        
        const value = parseInt(document.getElementById('counter-input').value);
        
        try {
            await this.setCounter(value);
            document.getElementById('counter-input').value = '';
        } catch (error) {
            this.showError('Failed to set counter value');
        }
    });
}
```

### **Form Handling**

```html
<div class="form-section">
    <h3>Set Counter Value</h3>
    <form id="counter-form">
        <input 
            type="number" 
            id="counter-input" 
            placeholder="Enter counter value"
            min="0"
            required
        >
        <button type="submit">Set Counter</button>
    </form>
</div>
```

## Step 6: Error Handling and User Feedback

### **Comprehensive Error Handling**

```javascript
class ErrorHandler {
    static handleError(error, context) {
        console.error(`Error in ${context}:`, error);
        
        // Categorize errors
        if (error.message.includes('insufficient balance')) {
            return 'Insufficient balance to complete transaction';
        } else if (error.message.includes('unauthorized')) {
            return 'You are not authorized to perform this action';
        } else if (error.message.includes('network')) {
            return 'Network connection error. Please try again.';
        } else if (error.message.includes('contract not found')) {
            return 'Contract not found. Please check your configuration.';
        } else {
            return `An error occurred: ${error.message}`;
        }
    }
    
    static showError(message, duration = 5000) {
        // Create error notification
        const notification = document.createElement('div');
        notification.className = 'error-notification';
        notification.textContent = message;
        
        // Add to page
        document.body.appendChild(notification);
        
        // Remove after duration
        setTimeout(() => {
            notification.remove();
        }, duration);
    }
    
    static showSuccess(message, duration = 3000) {
        // Create success notification
        const notification = document.createElement('div');
        notification.className = 'success-notification';
        notification.textContent = message;
        
        // Add to page
        document.body.appendChild(notification);
        
        // Remove after duration
        setTimeout(() => {
            notification.remove();
        }, duration);
    }
}
```

### **Loading States and Progress Indicators**

```javascript
class LoadingManager {
    static showLoading(elementId, text = 'Loading...') {
        const element = document.getElementById(elementId);
        if (element) {
            element.disabled = true;
            element.dataset.originalText = element.textContent;
            element.textContent = text;
        }
    }
    
    static hideLoading(elementId) {
        const element = document.getElementById(elementId);
        if (element && element.dataset.originalText) {
            element.disabled = false;
            element.textContent = element.dataset.originalText;
            delete element.dataset.originalText;
        }
    }
    
    static showProgress(containerId, message) {
        const container = document.getElementById(containerId);
        if (container) {
            container.innerHTML = `
                <div class="progress-indicator">
                    <div class="spinner"></div>
                    <div class="progress-text">${message}</div>
                </div>
            `;
        }
    }
    
    static hideProgress(containerId) {
        const container = document.getElementById(containerId);
        if (container) {
            container.innerHTML = '';
        }
    }
}
```

## Step 7: Network Configuration

### **Environment-Specific Configuration**

```javascript
class NetworkConfig {
    static getConfig(environment = 'testnet') {
        const configs = {
            testnet: {
                rpcUrl: 'https://soroban-testnet.stellar.org:443',
                networkPassphrase: 'Test SDF Network ; September 2015',
                horizonUrl: 'https://horizon-testnet.stellar.org',
                friendbotUrl: 'https://friendbot.stellar.org'
            },
            futurenet: {
                rpcUrl: 'https://soroban-futurenet.stellar.org:443',
                networkPassphrase: 'Test SDF Future Network ; October 2022',
                horizonUrl: 'https://horizon-futurenet.stellar.org',
                friendbotUrl: 'https://friendbot.stellar.org'
            },
            mainnet: {
                rpcUrl: 'https://soroban-mainnet.stellar.org:443',
                networkPassphrase: 'Public Global Stellar Network ; September 2015',
                horizonUrl: 'https://horizon.stellar.org',
                friendbotUrl: null
            }
        };
        
        return configs[environment] || configs.testnet;
    }
    
    static async switchNetwork(environment) {
        const config = this.getConfig(environment);
        
        // Update contract client with new network
        if (window.contractInterface) {
            await window.contractInterface.switchNetwork(config);
        }
        
        // Update UI
        this.updateNetworkIndicator(environment);
        
        // Store preference
        localStorage.setItem('preferred-network', environment);
    }
    
    static updateNetworkIndicator(network) {
        const indicator = document.getElementById('network-indicator');
        if (indicator) {
            indicator.textContent = `Network: ${network}`;
            indicator.className = `network-indicator ${network}`;
        }
    }
}
```

### **Network Selection UI**

```html
<div class="network-section">
    <label for="network-select">Network:</label>
    <select id="network-select">
        <option value="testnet">Testnet</option>
        <option value="futurenet">Futurenet</option>
        <option value="mainnet">Mainnet</option>
    </select>
    <span id="network-indicator" class="network-indicator testnet">Network: testnet</span>
</div>
```

```javascript
// Network selection handler
document.getElementById('network-select').addEventListener('change', async (e) => {
    const network = e.target.value;
    await NetworkConfig.switchNetwork(network);
});
```

## Step 8: Testing and Debugging

### **Local Testing Setup**

```bash
# Start local server (VS Code Live Server)
# Or use Python
python -m http.server 8000

# Or use Node.js
npx http-server -p 8000
```

### **Browser Developer Tools**

```javascript
// Add debugging to your contract interface
class DebugContractInterface extends ContractInterface {
    constructor() {
        super();
        this.debugMode = true;
    }
    
    async callContractFunction(functionName, ...args) {
        if (this.debugMode) {
            console.log(`Calling ${functionName} with args:`, args);
        }
        
        try {
            const result = await this.contract[functionName](...args);
            
            if (this.debugMode) {
                console.log(`${functionName} result:`, result);
            }
            
            return result;
            
        } catch (error) {
            if (this.debugMode) {
                console.error(`${functionName} error:`, error);
            }
            throw error;
        }
    }
}
```

### **Common Issues and Solutions**

#### **1. CORS Issues**
```javascript
// If you get CORS errors, you may need to use a proxy
const rpcUrl = 'https://cors-anywhere.herokuapp.com/https://soroban-testnet.stellar.org:443';
```

#### **2. Contract Not Found**
```javascript
// Verify contract ID
console.log('Contract ID:', this.contractId);

// Check network configuration
console.log('Network config:', this.network);
```

#### **3. RPC Connection Issues**
```javascript
// Test RPC connection
async testRpcConnection() {
    try {
        const response = await fetch(this.rpcUrl, {
            method: 'POST',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ jsonrpc: '2.0', id: 1, method: 'rpc.discover' })
        });
        
        if (!response.ok) {
            throw new Error(`RPC connection failed: ${response.status}`);
        }
        
        console.log('RPC connection successful');
        
    } catch (error) {
        console.error('RPC connection test failed:', error);
        throw error;
    }
}
```

## What's Next?

Now that you can connect frontends to contracts, you're ready to:

1. **[Build complete user interfaces](building-frontend.md)** - Create beautiful, functional UIs
2. **[Handle complex user interactions](user-interaction.md)** - Manage user workflows
3. **[Deploy production applications](advanced/README.md)** - Get your apps live
4. **[Optimize performance](advanced/contract-optimization.md)** - Make your apps fast

Remember: Good frontend-contract integration makes blockchain technology accessible to everyone. Focus on user experience, error handling, and clear feedback!

---

**Next**: [Building a Frontend](building-frontend.md) - Creating complete user interfaces
