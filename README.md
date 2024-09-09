# ArbiFund: Decentralized Crowdfunding on Arbitrum Stylus

ArbiFund is a decentralized crowdfunding platform built on Arbitrum Stylus using Rust. This project leverages the power of blockchain technology to create a transparent, efficient, and secure environment for fundraising campaigns.

## Table of Contents

1. [Introduction](#introduction)
2. [Features](#features)
3. [Prerequisites](#prerequisites)
4. [Quick Start](#quick-start)
5. [Project Structure](#project-structure)
6. [Smart Contract Details](#smart-contract-details)
7. [ABI Export](#abi-export)
8. [Deploying](#deploying)
9. [Interacting with the Contract](#interacting-with-the-contract)
10. [Testing](#testing)
11. [Contributing](#contributing)
12. [License](#license)

## Introduction

ArbiFund revolutionizes crowdfunding by utilizing Arbitrum Stylus, a Layer 2 scaling solution for Ethereum. By leveraging Rust for smart contract development, ArbiFund offers improved performance, enhanced security, and lower transaction costs compared to traditional Ethereum-based platforms.

## Features

- Create fundraising campaigns with customizable details
- Donate to campaigns using cryptocurrency
- Track donations and campaign progress in real-time
- Automatic fund transfer to campaign owners upon successful completion
- Transparent and immutable record of all transactions

## Prerequisites

Before you begin, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [Cargo Stylus](https://github.com/OffchainLabs/cargo-stylus)

## Quick Start

1. Install Rust and Cargo Stylus:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
cargo install --force cargo-stylus cargo-stylus-check
```

2. Add the `wasm32-unknown-unknown` build target:

```bash
rustup target add wasm32-unknown-unknown
```

3. Clone the ArbiFund repository:

```bash
git clone https://github.com/yourusername/arbifund.git
cd arbifund
```

4. Build the project:

```bash
cargo build --release
```

## Project Structure

```
arbifund/
├── src/
│   ├── lib.rs
│   └── main.rs
├── Cargo.toml
├── Cargo.lock
└── README.md
```

- `src/lib.rs`: Contains the main smart contract logic
- `src/main.rs`: Entry point for the Stylus program
- `Cargo.toml`: Project configuration and dependencies

## Smart Contract Details

The ArbiFund smart contract includes the following main functions:

- `create_campaign`: Create a new fundraising campaign
- `donate_to_campaign`: Make a donation to a specific campaign
- `get_donators`: Retrieve the list of donators for a campaign
- `get_campaigns`: Get details of all campaigns

For full details, refer to the `src/lib.rs` file in the project.

## ABI Export

To export the Solidity ABI for the ArbiFund contract:

```bash
cargo stylus export-abi
```

This will generate an interface similar to:

```solidity
interface ArbiFund {
    function createCampaign(address owner, string memory title, string memory description, uint256 target, uint256 deadline, string memory image) external returns (uint256);
    function donateToCampaign(uint256 campaignId) external payable;
    function getDonators(uint256 campaignId) external view returns (address[] memory, uint256[] memory);
    function getCampaigns() external view returns (address[] memory, string[] memory, string[] memory, uint256[] memory, uint256[] memory, uint256[] memory, string[] memory);
}
```

## Deploying

1. Check if your program compiles to valid WASM for Stylus:

```bash
cargo stylus check
```

2. Estimate gas costs for deployment:

```bash
cargo stylus deploy --private-key-path=<PRIVKEY_FILE_PATH> --estimate-gas
```

3. Deploy the contract:

```bash
cargo stylus deploy --private-key-path=<PRIVKEY_FILE_PATH>
```

## Interacting with the Contract

After deployment, you can interact with the ArbiFund contract using any Ethereum development tools that support Arbitrum, such as ethers.js or web3.js. Remember to use the contract address provided during deployment.

Example using ethers.js:

```javascript
const { ethers } = require('ethers')
const ArbiFundABI = require('./ArbiFundABI.json')

const provider = new ethers.providers.JsonRpcProvider(
  'https://stylus-testnet.arbitrum.io/rpc'
)
const signer = new ethers.Wallet(privateKey, provider)
const arbiFundContract = new ethers.Contract(
  contractAddress,
  ArbiFundABI,
  signer
)

// Create a campaign
await arbiFundContract.createCampaign(
  owner,
  'My Campaign',
  'Description',
  ethers.utils.parseEther('10'),
  Math.floor(Date.now() / 1000) + 30 * 24 * 60 * 60,
  'image_url'
)

// Donate to a campaign
await arbiFundContract.donateToCampaign(campaignId, {
  value: ethers.utils.parseEther('1'),
})
```

## Testing

To run the test suite:

```bash
cargo test
```

## Contributing

Contributions to ArbiFund are welcome! Please refer to the [CONTRIBUTING.md](CONTRIBUTING.md) file for guidelines.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
