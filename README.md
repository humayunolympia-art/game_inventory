# Project Title

Game Inventory – In-Game Item Ownership on Stellar via Soroban Smart Contracts

## Project Vision

This project demonstrates **in-game item ownership management using Soroban smart contracts** on the Stellar blockchain. It provides:
- How to write a Soroban smart contract in Rust for game asset management
- How to manage persistent storage for item ownership and user inventories
- How to handle user authentication and item transfers on Stellar
- How to deploy and interact with game contracts on Stellar Testnet

The goal is to provide a foundational system for blockchain-based game item ownership that developers can study, deploy, and extend.

---

## Description

A Soroban smart contract dApp that enables **in-game item ownership on Stellar Testnet**. Admins can mint items with specific types and stats, while users can transfer items among themselves. All item data is stored permanently on-chain, enabling transparent ownership tracking.

---

## Features

### 1. Admin Item Minting
- Admin can mint new items with unique IDs
- Each item has a type (String) and stats (u64)
- Items are assigned to owners upon minting

### 2. User Item Transfers
- Item owners can transfer items to other users
- Transfers require authentication from the sender
- Owner history is updated on-chain

### 3. Ownership Tracking
- Get current owner of any item
- Get item details (type and stats)
- Get all items owned by a specific user

### 4. Persistent Storage
- All items stored permanently on blockchain
- User inventories tracked on-chain
- Complete ownership history verifiable by anyone

---

## Contract

- **Network**: Stellar Testnet
- **Contract ID**: [CCP3G2QAZ4A7FZ2YHCPVRNEQT4YX53OUD7ZC4NFXIZRX6ZP3TNUQIVU7](https://stellar.expert/explorer/testnet/tx/ebda1bba9c11d2981f43b6896e128dc1a088760e574d3b877eb68d7a25a9fc30)

![screenshot](https://i.ibb.co/jPtFM4GH/image.png)

### Contract Methods

| Method | Description | Auth Required |
|--------|-------------|---------------|
| `init` | Initialize the contract | No |
| `mint_item(admin, owner, item_id, item_type, stats)` | Admin mints new item | Admin |
| `transfer_item(from, to, item_id)` | Transfer item between users | From address |
| `get_owner(item_id)` | Get current owner of item | No |
| `get_item(item_id)` | Get item type and stats | No |
| `get_user_items(user)` | Get all item IDs owned by user | No |

---

## Future Scopes

### 1. Item Trading Marketplace
- Build a marketplace for trading items between users
- Add price listing and order matching

### 2. Item Evolution/Upgrade System
- Allow combining items or upgrading stats
- Add item level progression

### 3. Limited Edition Items
- Add scarcity mechanisms (limited mints, rarity levels)
- Support for special event items

### 4. Frontend dApp
- Build a React or vanilla JS interface for item management
- Display user inventory and item details visually

### 5. Batch Operations
- Support minting multiple items in one transaction
- Add batch transfer capabilities

### 6. Access Control
- Role-based permissions (admin, user)
- Pausable contract for emergency situations

---

## Profile

- **Name:** humayunolympia
