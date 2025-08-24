# Network Configuration

Complete reference for Stellar network configuration. This page provides all the settings, endpoints, and configuration options needed to connect to different Stellar networks.

## Network Overview

Stellar provides multiple networks for different purposes:

- **Testnet** - Development and testing environment
- **Futurenet** - Advanced testing and feature previews
- **Mainnet** - Production network with real assets

## Network Configurations

### **Testnet Configuration**

The testnet is the primary development environment for Stellar developers.

```json
{
  "name": "testnet",
  "description": "Stellar Test Network",
  "rpcUrl": "https://soroban-testnet.stellar.org:443",
  "horizonUrl": "https://horizon-testnet.stellar.org",
  "networkPassphrase": "Test SDF Network ; September 2015",
  "friendbotUrl": "https://friendbot.stellar.org",
  "baseReserve": "0.5",
  "baseFee": "100",
  "maxFee": "10000"
}
```

**Key Features:**
- **Free Testing**: No real value at stake
- **Friendbot Funding**: Automatic account funding
- **Fast Confirmation**: Quick transaction processing
- **Stable Environment**: Reliable for development

**Use Cases:**
- Contract development and testing
- Frontend integration testing
- User experience validation
- Performance testing

### **Futurenet Configuration**

Futurenet provides access to upcoming Stellar features and capabilities.

```json
{
  "name": "futurenet",
  "description": "Stellar Future Network",
  "rpcUrl": "https://soroban-futurenet.stellar.org:443",
  "horizonUrl": "https://horizon-futurenet.stellar.org",
  "networkPassphrase": "Test SDF Future Network ; October 2022",
  "friendbotUrl": "https://friendbot.stellar.org",
  "baseReserve": "0.5",
  "baseFee": "100",
  "maxFee": "10000"
}
```

**Key Features:**
- **Feature Preview**: Access to upcoming capabilities
- **Advanced Testing**: Test cutting-edge features
- **Experimental**: May have instability
- **Development Focus**: For advanced developers

**Use Cases:**
- Testing new Stellar features
- Advanced contract development
- Protocol experimentation
- Early adopter testing

### **Mainnet Configuration**

Mainnet is the production Stellar network with real assets and value.

```json
{
  "name": "mainnet",
  "description": "Stellar Main Network",
  "rpcUrl": "https://soroban-mainnet.stellar.org:443",
  "horizonUrl": "https://horizon.stellar.org",
  "networkPassphrase": "Public Global Stellar Network ; September 2015",
  "friendbotUrl": null,
  "baseReserve": "1.0",
  "baseFee": "100",
  "maxFee": "10000"
}
```

**Key Features:**
- **Real Assets**: Actual value and transactions
- **Production Ready**: Stable and reliable
- **No Friendbot**: Manual account funding required
- **Higher Costs**: Real XLM for fees

**Use Cases:**
- Production applications
- Real asset management
- Live user services
- Commercial deployments

## Configuration Management

### **Environment Variables**

Set network configuration using environment variables.

```bash
# Testnet Configuration
export STELLAR_NETWORK=testnet
export STELLAR_RPC_URL=https://soroban-testnet.stellar.org:443
export STELLAR_HORIZON_URL=https://horizon-testnet.stellar.org
export STELLAR_NETWORK_PASSPHRASE="Test SDF Network ; September 2015"

# Futurenet Configuration
export STELLAR_NETWORK=futurenet
export STELLAR_RPC_URL=https://soroban-futurenet.stellar.org:443
export STELLAR_HORIZON_URL=https://horizon-futurenet.stellar.org
export STELLAR_NETWORK_PASSPHRASE="Test SDF Future Network ; October 2022"

# Mainnet Configuration
export STELLAR_NETWORK=mainnet
export STELLAR_RPC_URL=https://soroban-mainnet.stellar.org:443
export STELLAR_HORIZON_URL=https://horizon.stellar.org
export STELLAR_NETWORK_PASSPHRASE="Public Global Stellar Network ; September 2015"
```

### **Configuration Files**

Store network configuration in files for easy management.

```json
// config/networks.json
{
  "testnet": {
    "rpcUrl": "https://soroban-testnet.stellar.org:443",
    "horizonUrl": "https://horizon-testnet.stellar.org",
    "networkPassphrase": "Test SDF Network ; September 2015",
    "friendbotUrl": "https://friendbot.stellar.org",
    "baseReserve": "0.5",
    "baseFee": "100"
  },
  "futurenet": {
    "rpcUrl": "https://soroban-futurenet.stellar.org:443",
    "horizonUrl": "https://horizon-futurenet.stellar.org",
    "networkPassphrase": "Test SDF Future Network ; October 2022",
    "friendbotUrl": "https://friendbot.stellar.org",
    "baseReserve": "0.5",
    "baseFee": "100"
  },
  "mainnet": {
    "rpcUrl": "https://soroban-mainnet.stellar.org:443",
    "horizonUrl": "https://horizon.stellar.org",
    "networkPassphrase": "Public Global Stellar Network ; September 2015",
    "friendbotUrl": null,
    "baseReserve": "1.0",
    "baseFee": "100"
  }
}
```

### **Configuration Classes**

Implement configuration management in your applications.

```typescript
// Network configuration class
class NetworkConfig {
  private static configs = {
    testnet: {
      rpcUrl: 'https://soroban-testnet.stellar.org:443',
      horizonUrl: 'https://horizon-testnet.stellar.org',
      networkPassphrase: 'Test SDF Network ; September 2015',
      friendbotUrl: 'https://friendbot.stellar.org',
      baseReserve: 0.5,
      baseFee: 100
    },
    futurenet: {
      rpcUrl: 'https://soroban-futurenet.stellar.org:443',
      horizonUrl: 'https://horizon-futurenet.stellar.org',
      networkPassphrase: 'Test SDF Future Network ; October 2022',
      friendbotUrl: 'https://friendbot.stellar.org',
      baseReserve: 0.5,
      baseFee: 100
    },
    mainnet: {
      rpcUrl: 'https://soroban-mainnet.stellar.org:443',
      horizonUrl: 'https://horizon.stellar.org',
      networkPassphrase: 'Public Global Stellar Network ; September 2015',
      friendbotUrl: null,
      baseReserve: 1.0,
      baseFee: 100
    }
  };

  static getConfig(network: string) {
    return this.configs[network] || this.configs.testnet;
  }

  static getRpcUrl(network: string) {
    return this.getConfig(network).rpcUrl;
  }

  static getHorizonUrl(network: string) {
    return this.getConfig(network).horizonUrl;
  }

  static getNetworkPassphrase(network: string) {
    return this.getConfig(network).networkPassphrase;
  }

  static getFriendbotUrl(network: string) {
    return this.getConfig(network).friendbotUrl;
  }
}

// Usage
const testnetConfig = NetworkConfig.getConfig('testnet');
const mainnetRpcUrl = NetworkConfig.getRpcUrl('mainnet');
```

## Network Switching

### **CLI Network Switching**

Switch between networks using the Stellar CLI.

```bash
# Show current network
stellar network show

# List available networks
stellar network list

# Switch to testnet
stellar network switch testnet

# Switch to futurenet
stellar network switch futurenet

# Switch to mainnet
stellar network switch mainnet
```

### **Programmatic Network Switching**

Switch networks in your applications.

```typescript
class NetworkManager {
  private currentNetwork: string = 'testnet';

  async switchNetwork(network: string) {
    // Validate network
    if (!['testnet', 'futurenet', 'mainnet'].includes(network)) {
      throw new Error(`Invalid network: ${network}`);
    }

    // Get network configuration
    const config = NetworkConfig.getConfig(network);

    // Update client configuration
    await this.updateClientConfig(config);

    // Update current network
    this.currentNetwork = network;

    // Emit network change event
    this.emit('networkChanged', network);

    // Store preference
    localStorage.setItem('preferred-network', network);
  }

  getCurrentNetwork() {
    return this.currentNetwork;
  }

  getCurrentConfig() {
    return NetworkConfig.getConfig(this.currentNetwork);
  }

  private async updateClientConfig(config: any) {
    // Update RPC client
    this.rpcClient.setEndpoint(config.rpcUrl);

    // Update Horizon client
    this.horizonClient.setEndpoint(config.horizonUrl);

    // Update network passphrase
    this.setNetworkPassphrase(config.networkPassphrase);
  }
}
```

### **Frontend Network Selection**

Provide network selection in user interfaces.

```html
<!-- Network selection component -->
<div class="network-selector">
  <label for="network-select">Network:</label>
  <select id="network-select" onchange="switchNetwork(this.value)">
    <option value="testnet">Testnet (Development)</option>
    <option value="futurenet">Futurenet (Preview)</option>
    <option value="mainnet">Mainnet (Production)</option>
  </select>
  
  <div class="network-info">
    <span id="current-network">testnet</span>
    <span id="network-status" class="status-connected">Connected</span>
  </div>
</div>
```

```javascript
// Network switching handler
async function switchNetwork(network) {
  try {
    // Show loading state
    document.getElementById('network-status').textContent = 'Switching...';
    document.getElementById('network-status').className = 'status-switching';

    // Switch network
    await networkManager.switchNetwork(network);

    // Update UI
    document.getElementById('current-network').textContent = network;
    document.getElementById('network-status').textContent = 'Connected';
    document.getElementById('network-status').className = 'status-connected';

    // Show success message
    showNotification(`Switched to ${network} network`, 'success');

  } catch (error) {
    // Show error state
    document.getElementById('network-status').textContent = 'Error';
    document.getElementById('network-status').className = 'status-error';

    // Show error message
    showNotification(`Failed to switch to ${network}: ${error.message}`, 'error');
  }
}
```

## Endpoint Management

### **RPC Endpoints**

Manage RPC connections for smart contract interactions.

```typescript
class RpcManager {
  private endpoints: Map<string, string> = new Map();
  private currentEndpoint: string;

  constructor() {
    this.initializeEndpoints();
  }

  private initializeEndpoints() {
    this.endpoints.set('testnet', 'https://soroban-testnet.stellar.org:443');
    this.endpoints.set('futurenet', 'https://soroban-futurenet.stellar.org:443');
    this.endpoints.set('mainnet', 'https://soroban-mainnet.stellar.org:443');
    
    // Set default endpoint
    this.currentEndpoint = this.endpoints.get('testnet');
  }

  getEndpoint(network?: string) {
    if (network) {
      return this.endpoints.get(network) || this.currentEndpoint;
    }
    return this.currentEndpoint;
  }

  setEndpoint(network: string, endpoint: string) {
    this.endpoints.set(network, endpoint);
  }

  async testConnection(endpoint: string) {
    try {
      const response = await fetch(endpoint, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: 1,
          method: 'rpc.discover'
        })
      });

      return response.ok;
    } catch (error) {
      return false;
    }
  }

  async getBestEndpoint(network: string) {
    const endpoint = this.getEndpoint(network);
    
    if (await this.testConnection(endpoint)) {
      return endpoint;
    }

    // Try alternative endpoints
    const alternatives = this.getAlternativeEndpoints(network);
    for (const alt of alternatives) {
      if (await this.testConnection(alt)) {
        return alt;
      }
    }

    throw new Error(`No working endpoints found for ${network}`);
  }
}
```

### **Horizon Endpoints**

Manage Horizon API connections for account and transaction data.

```typescript
class HorizonManager {
  private endpoints: Map<string, string> = new Map();
  private currentEndpoint: string;

  constructor() {
    this.initializeEndpoints();
  }

  private initializeEndpoints() {
    this.endpoints.set('testnet', 'https://horizon-testnet.stellar.org');
    this.endpoints.set('futurenet', 'https://horizon-futurenet.stellar.org');
    this.endpoints.set('mainnet', 'https://horizon.stellar.org');
    
    this.currentEndpoint = this.endpoints.get('testnet');
  }

  getEndpoint(network?: string) {
    if (network) {
      return this.endpoints.get(network) || this.currentEndpoint;
    }
    return this.currentEndpoint;
  }

  async getAccountInfo(accountId: string, network?: string) {
    const endpoint = this.getEndpoint(network);
    const response = await fetch(`${endpoint}/accounts/${accountId}`);
    
    if (!response.ok) {
      throw new Error(`Failed to get account info: ${response.statusText}`);
    }
    
    return response.json();
  }

  async getTransactionInfo(transactionHash: string, network?: string) {
    const endpoint = this.getEndpoint(network);
    const response = await fetch(`${endpoint}/transactions/${transactionHash}`);
    
    if (!response.ok) {
      throw new Error(`Failed to get transaction info: ${response.statusText}`);
    }
    
    return response.json();
  }

  async submitTransaction(transactionXdr: string, network?: string) {
    const endpoint = this.getEndpoint(network);
    const response = await fetch(`${endpoint}/transactions`, {
      method: 'POST',
      headers: { 'Content-Type': 'application/x-www-form-urlencoded' },
      body: `tx=${encodeURIComponent(transactionXdr)}`
    });
    
    if (!response.ok) {
      const error = await response.json();
      throw new Error(`Transaction failed: ${error.extras.result_codes.operations.join(', ')}`);
    }
    
    return response.json();
  }
}
```

## Network-Specific Features

### **Friendbot Integration**

Automatically fund test accounts on testnet and futurenet.

```typescript
class FriendbotManager {
  private friendbotUrls: Map<string, string> = new Map();

  constructor() {
    this.friendbotUrls.set('testnet', 'https://friendbot.stellar.org');
    this.friendbotUrls.set('futurenet', 'https://friendbot.stellar.org');
    // Mainnet has no friendbot
  }

  async fundAccount(accountId: string, network: string) {
    const friendbotUrl = this.friendbotUrls.get(network);
    
    if (!friendbotUrl) {
      throw new Error(`Friendbot not available for ${network}`);
    }

    try {
      const response = await fetch(friendbotUrl, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ addr: accountId })
      });

      if (!response.ok) {
        const error = await response.json();
        throw new Error(`Friendbot funding failed: ${error.detail}`);
      }

      const result = await response.json();
      return result;

    } catch (error) {
      throw new Error(`Friendbot request failed: ${error.message}`);
    }
  }

  isAvailable(network: string) {
    return this.friendbotUrls.has(network);
  }
}
```

### **Network Validation**

Validate network-specific requirements and constraints.

```typescript
class NetworkValidator {
  static validateNetwork(network: string) {
    const validNetworks = ['testnet', 'futurenet', 'mainnet'];
    
    if (!validNetworks.includes(network)) {
      throw new Error(`Invalid network: ${network}. Must be one of: ${validNetworks.join(', ')}`);
    }
  }

  static validateMainnetOperation(operation: any, network: string) {
    if (network === 'mainnet') {
      // Additional validation for mainnet
      if (operation.amount && parseFloat(operation.amount) > 1000000) {
        throw new Error('Large amounts require additional verification on mainnet');
      }
      
      if (operation.destination && !this.isValidMainnetAddress(operation.destination)) {
        throw new Error('Invalid mainnet address format');
      }
    }
  }

  static validateTestnetOperation(operation: any, network: string) {
    if (network === 'testnet' || network === 'futurenet') {
      // Relaxed validation for test networks
      if (operation.amount && parseFloat(operation.amount) > 10000000) {
        console.warn('Very large amount detected on test network');
      }
    }
  }

  private static isValidMainnetAddress(address: string) {
    // Basic Stellar address validation
    return /^G[A-Z2-7]{55}$/.test(address);
  }
}
```

## Configuration Best Practices

### **Environment Separation**

Keep different environments properly separated.

```typescript
class EnvironmentManager {
  private static instance: EnvironmentManager;
  private currentEnvironment: string;

  private constructor() {
    this.currentEnvironment = this.detectEnvironment();
  }

  static getInstance() {
    if (!EnvironmentManager.instance) {
      EnvironmentManager.instance = new EnvironmentManager();
    }
    return EnvironmentManager.instance;
  }

  private detectEnvironment() {
    // Detect environment from various sources
    if (process.env.NODE_ENV === 'production') {
      return 'mainnet';
    }
    
    if (process.env.STELLAR_NETWORK) {
      return process.env.STELLAR_NETWORK;
    }
    
    if (window.location.hostname.includes('test')) {
      return 'testnet';
    }
    
    return 'testnet'; // Default to testnet
  }

  getCurrentEnvironment() {
    return this.currentEnvironment;
  }

  isProduction() {
    return this.currentEnvironment === 'mainnet';
  }

  isDevelopment() {
    return this.currentEnvironment === 'testnet' || this.currentEnvironment === 'futurenet';
  }

  getConfiguration() {
    return NetworkConfig.getConfig(this.currentEnvironment);
  }
}
```

### **Configuration Validation**

Validate configuration before use.

```typescript
class ConfigurationValidator {
  static validateNetworkConfig(config: any) {
    const required = ['rpcUrl', 'horizonUrl', 'networkPassphrase'];
    
    for (const field of required) {
      if (!config[field]) {
        throw new Error(`Missing required configuration field: ${field}`);
      }
    }

    // Validate URL formats
    if (!this.isValidUrl(config.rpcUrl)) {
      throw new Error(`Invalid RPC URL: ${config.rpcUrl}`);
    }

    if (!this.isValidUrl(config.horizonUrl)) {
      throw new Error(`Invalid Horizon URL: ${config.horizonUrl}`);
    }

    // Validate network passphrase
    if (typeof config.networkPassphrase !== 'string' || config.networkPassphrase.length < 10) {
      throw new Error('Invalid network passphrase');
    }

    return true;
  }

  private static isValidUrl(url: string) {
    try {
      new URL(url);
      return true;
    } catch {
      return false;
    }
  }
}
```

## Troubleshooting

### **Common Network Issues**

Identify and resolve common network problems.

```typescript
class NetworkTroubleshooter {
  static async diagnoseConnection(network: string) {
    const config = NetworkConfig.getConfig(network);
    const issues = [];

    // Test RPC connection
    try {
      const rpcWorking = await this.testRpcConnection(config.rpcUrl);
      if (!rpcWorking) {
        issues.push('RPC endpoint not responding');
      }
    } catch (error) {
      issues.push(`RPC connection error: ${error.message}`);
    }

    // Test Horizon connection
    try {
      const horizonWorking = await this.testHorizonConnection(config.horizonUrl);
      if (!horizonWorking) {
        issues.push('Horizon endpoint not responding');
      }
    } catch (error) {
      issues.push(`Horizon connection error: ${error.message}`);
    }

    // Test network passphrase
    if (!config.networkPassphrase || config.networkPassphrase.length < 10) {
      issues.push('Invalid network passphrase');
    }

    return {
      network,
      working: issues.length === 0,
      issues
    };
  }

  private static async testRpcConnection(url: string) {
    try {
      const response = await fetch(url, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          jsonrpc: '2.0',
          id: 1,
          method: 'rpc.discover'
        })
      });
      return response.ok;
    } catch {
      return false;
    }
  }

  private static async testHorizonConnection(url: string) {
    try {
      const response = await fetch(`${url}/ledgers?limit=1`);
      return response.ok;
    } catch {
      return false;
    }
  }
}
```

### **Network Recovery**

Implement automatic network recovery mechanisms.

```typescript
class NetworkRecovery {
  private retryAttempts = 3;
  private retryDelay = 1000;

  async recoverConnection(network: string) {
    const config = NetworkConfig.getConfig(network);
    
    for (let attempt = 1; attempt <= this.retryAttempts; attempt++) {
      try {
        console.log(`Attempting to recover ${network} connection (${attempt}/${this.retryAttempts})`);
        
        // Test connection
        const working = await this.testConnection(config);
        
        if (working) {
          console.log(`${network} connection recovered successfully`);
          return true;
        }
        
        // Wait before retry
        if (attempt < this.retryAttempts) {
          await this.delay(this.retryDelay * attempt);
        }
        
      } catch (error) {
        console.error(`Recovery attempt ${attempt} failed:`, error);
      }
    }

    console.error(`Failed to recover ${network} connection after ${this.retryAttempts} attempts`);
    return false;
  }

  private async testConnection(config: any) {
    const rpcWorking = await this.testRpcConnection(config.rpcUrl);
    const horizonWorking = await this.testHorizonConnection(config.horizonUrl);
    
    return rpcWorking && horizonWorking;
  }

  private async testRpcConnection(url: string) {
    // RPC connection test implementation
    return true;
  }

  private async testHorizonConnection(url: string) {
    // Horizon connection test implementation
    return true;
  }

  private delay(ms: number) {
    return new Promise(resolve => setTimeout(resolve, ms));
  }
}
```

## What You've Learned

### **Network Fundamentals**
- ✅ **Network Types** - Testnet, futurenet, and mainnet configurations
- ✅ **Configuration Management** - Environment variables, files, and classes
- ✅ **Network Switching** - CLI and programmatic network switching
- ✅ **Endpoint Management** - RPC and Horizon endpoint configuration

### **Advanced Features**
- ✅ **Network-Specific Features** - Friendbot integration and validation
- ✅ **Configuration Best Practices** - Environment separation and validation
- ✅ **Troubleshooting** - Connection diagnosis and recovery
- ✅ **Production Ready** - Comprehensive network management

### **Implementation Ready**
- ✅ **Complete Configuration** - All network settings and endpoints
- ✅ **Practical Examples** - Real-world configuration patterns
- ✅ **Error Handling** - Robust error management and recovery
- ✅ **Best Practices** - Production-ready configuration management

## Next Steps

### **Immediate Actions**
1. **Configure Networks** - Set up testnet, futurenet, and mainnet
2. **Test Connections** - Verify all endpoints are working
3. **Implement Switching** - Add network switching to your applications
4. **Validate Configuration** - Ensure all settings are correct

### **Advanced Configuration**
1. **[CLI Commands](cli-commands.md)** - Network management commands
2. **[Rust SDK](rust-sdk.md)** - SDK network configuration
3. **[Advanced Topics](advanced/README.md)** - Network optimization
4. **Production Deployment** - Mainnet configuration and deployment

### **Resources**
- **Official Documentation** - [developers.stellar.org](https://developers.stellar.org)
- **Network Status** - [status.stellar.org](https://status.stellar.org)
- **Community Discord** - [discord.gg/stellar](https://discord.gg/stellar)
- **Network Explorer** - [stellar.expert](https://stellar.expert)

## Summary

Network configuration is the foundation of Stellar development. Understanding how to configure and manage different networks will enable you to develop, test, and deploy applications effectively across all Stellar environments.

Remember: Always test on testnet first, validate on futurenet for new features, and only deploy to mainnet when you're confident in your application's stability and security.

The best network configuration is one that provides seamless switching between environments while maintaining security and reliability. Invest time in proper configuration—it will save you time and prevent issues in the long run!

---

**Next**: [Advanced Topics](advanced/README.md) - Security, optimization, and production readiness

