# EIP-7702 Transaction Demo

A comprehensive Rust implementation demonstrating EIP-7702 (Set EOA account code) transaction patterns using the [OKX Wallet Core](https://github.com/okx/wallet-core/tree/main) as the delegate contract.


## 🔧 Setup

1. **Clone the repository**
   ```bash
   git clone <repository-url>
   cd tx-7702
   ```

2. **Create `.env` file and put configure variables**
   ```bash
   cp .env.example .env
   ```

## 🎯 Usage

### Run Individual Patterns

```bash
# Self-authorization pattern (Bob -> Receiver)
cargo run self

# Relayer pattern (Alice -> Receiver, Bob pays gas)
cargo run relayer

# normal, simplest and initialize
 
```


## 📋 Transaction Patterns

### Pattern 1: Self-Authorization

Bob authorizes his own EOA and sends the transaction:

```
┌─────────────────┐    ┌─────────────────┐
│   Bob's EOA     │    │    Receiver     │
│ (with WalletCore│    │                 │
│     code)       │    │                 │
└─────────────────┘    └─────────────────┘
         │                       │
         │ 1. Bob authorizes     │
         │    his own EOA        │
         │                       │
         │ 2. Bob sends tx       │
         │    and pays gas       │
         │                       │
         │ 3. Transfers tokens   │
         │    from Bob to        │
         │    Receiver           │
         ├──────────────────────►│
```

**Key characteristics:**
- Same person (Bob) handles authorization and transaction submission
- Bob pays gas fees
- Bob's tokens are transferred

### Pattern 2: Relayer (Execute From Relayer)

Alice signs off-chain, Bob submits the transaction:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Relayer (Bob) │    │  Alice's EOA    │    │    Receiver     │
│                 │    │ (with WalletCore│    │                 │
│                 │    │     code)       │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         │ 1. Sends tx to        │                       │
         │    Alice's address    │                       │
         ├──────────────────────►│                       │
         │                       │                       │
         │                       │ 2. Alice's address    │
         │                       │    executes the       │
         │                       │    WalletCore code    │
         │                       │                       │
         │                       │ 3. Transfers tokens   │
         │                       │    from Alice to      │
         │                       │    Receiver           │
         │                       ├──────────────────────►│
```

**Key characteristics:**
- Alice signs transaction off-chain
- Bob (relayer) submits transaction and pays gas fees
- Alice's tokens are transferred
- Enables gasless transactions for Alice

## 🏗️ Architecture

### Core Components

1. **Config** (`src/core/config.rs`)
   - Environment variable management
   - Signer and address configuration

2. **Contracts** (`src/core/contracts.rs`)
   - Smart contract interfaces using `sol!` macros
   - Type-safe contract interactions

3. **EIP-7702 Builder** (`src/core/builder.rs`)
   - Authorization creation and management
   - Transaction building utilities
   - Balance checking and debugging tools

4. **Examples** (`src/examples/`)
   - initialize the wallet
   - simplest 7702 tx example provided by alloy-rs
   - normal erc20 transfer
   - 7702 relayer case
   - 7702 self call case


## 🙏 Acknowledgments && Reference

- [OKX Wallet Core](https://github.com/okx/wallet-core) for the smart contract implementation
- [Alloy](https://github.com/alloy-rs/alloy) for the excellent Ethereum development framework
