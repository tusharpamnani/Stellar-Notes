# Stellar Hello World Frontend

This project demonstrates how to connect a Stellar smart contract (on Soroban) to a modern Astro-based frontend. It is designed to be beginner-friendly, with clear explanations and step-by-step instructions.

## Features
- **Connects to a Stellar Soroban contract** on the testnet
- **Takes user input** (name) and sends it to the contract
- **Displays the contract's greeting** dynamically on the page
- **Modern, attractive UI** with responsive design

---

## Quick Start: Step-by-Step Setup

### 1. Create the Astro Project
If you haven't already, create a new Astro project:
```bash
npm create astro@latest
```
Follow the prompts to set up your project.

### 2. Copy the Contract Metadata
After deploying your Soroban contract, a `.stellar` folder is generated in your contract's directory. If your frontend is in a different directory, copy this folder into your frontend project root:
```bash
cp -R ../.stellar/ .stellar
```
> **Note:** This step is needed so the frontend knows the contract ID and ABI. Always keep `.stellar` up to date if you redeploy the contract.

### 3. Generate TypeScript Bindings for the Contract
Use the Stellar CLI to generate TypeScript bindings for your contract:
```bash
stellar contract bindings typescript \
  --network testnet \
  --contract-id hello_world \
  --output-dir packages/hello_world
```
This creates the `packages/hello_world` directory with the code needed to interact with your contract.

### 4. Install Dependencies and Build the Contract Package
```bash
cd packages/hello_world
npm install
npm run build
cd ../..
```

### 5. Install Frontend Dependencies
From your project root:
```bash
npm install
```

### 6. Run the Development Server
```bash
npm run dev
```
Visit [http://localhost:3000](http://localhost:3000) in your browser.

---

## How the Contract is Called from the Frontend
- The frontend automatically reads the contract metadata from `.stellar` and uses the generated bindings in `packages/hello_world` to call the contract.
- When a user enters their name and clicks the button, the frontend calls the contract's `hello` method and displays the result.

### (Optional) How to Call the Contract Manually
If you want to experiment with contract calls in your own scripts, you can do something like:
```js
import * as Client from './packages/hello_world';
const contract = new Client.Client({
  ...Client.networks.testnet,
  rpcUrl: 'https://soroban-testnet.stellar.org:443'
});
const { result } = await contract.hello({ to: 'YourName' });
console.log(result.join(' '));
```
This is not needed for the app to work, but it's useful for learning and debugging.

---

## How It Works

### 1. Contract Client Setup
- The contract client is imported from the `packages/hello_world` package, which contains the code to interact with your Soroban contract.
- The client is initialized with the testnet configuration and the Soroban RPC URL:
  ```js
  import * as Client from '../packages/hello_world';
  const contract = new Client.Client({
    ...Client.networks.testnet,
    rpcUrl: 'https://soroban-testnet.stellar.org:443'
  });
  ```

### 2. How the Contract Metadata is Used
- When you deploy your Soroban contract, deployment tools (like the Soroban CLI) generate contract metadata and save it in a hidden folder called `.stellar` in your project directory.
- This metadata includes important information such as the contract ID, network, and ABI (Application Binary Interface).
- The frontend client (in `packages/hello_world`) reads this metadata to know how to invoke the contract and what methods are available.
- This means you **do not need to hardcode contract details** in your frontend; the client uses the metadata from `.stellar` to connect and interact with the deployed contract.
- If you redeploy or update your contract, make sure the `.stellar` folder is up to date so the frontend always has the correct contract information.

### 3. User Input and Form
- The page displays an input box and a button labeled **"Invoke Hello World Contract"**.
- The user enters their name and clicks the button.

### 4. Client-Side Contract Call
- A `<script type="module">` in the page listens for the form submission.
- When the form is submitted:
  1. The name is read from the input.
  2. The contract's `hello` method is called with `{ to: name }`.
  3. The result (greeting) is displayed on the page.
- Example code:
  ```js
  form.addEventListener('submit', async (e) => {
    e.preventDefault();
    const name = input.value.trim();
    if (!name) return;
    greetingEl.textContent = 'Loading...';
    try {
      const { result } = await contract.hello({ to: name });
      greetingEl.textContent = result.join(' ');
    } catch (err) {
      greetingEl.textContent = 'Error: ' + (err?.message || 'Could not fetch greeting');
    }
  });
  ```

### 5. UI/UX Details
- The greeting is shown in a large, colorful heading.
- The background uses a blurred SVG for a modern look.
- The page is responsive and works on mobile.
- Helpful links to Astro and Soroban documentation are provided.

---

## Customization
- **Change the contract:**
  - Update the import and client initialization in `src/pages/index.astro` to point to your own contract package and network.
- **Change the greeting logic:**
  - Edit the contract call in the `<script>` block to use a different method or parameters.
- **Style the page:**
  - Modify the CSS in `index.astro` for your own branding.

---

## Troubleshooting
- If you see `Error: Could not fetch greeting`, check your network connection and contract deployment.
- Make sure the contract is deployed and accessible on the Soroban testnet.
- Check the browser console for more detailed error messages.

---

## Further Reading
- [Astro Documentation](https://docs.astro.build)
- [Stellar Soroban Documentation](https://soroban.stellar.org/docs)
- [Stellar Testnet Explorer](https://testnet.stellarchain.io/)

---

## Project Structure
- `src/pages/index.astro` — Main page, UI, and contract interaction logic
- `packages/hello_world/` — Contract client code (auto-generated or hand-written)
- `.stellar/` — Contains contract metadata (ID, ABI, etc.) generated after deployment
- `src/assets/` — Images and SVGs for UI
- `public/` — Static assets

---

## Questions?
If you get stuck, check the docs above or ask for help in the [Stellar Discord](https://discord.gg/stellar) or [Astro Discord](https://astro.build/chat).

Happy building! 🚀
