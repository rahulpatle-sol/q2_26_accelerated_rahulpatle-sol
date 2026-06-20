# Turbin3 Q2 2026 — Accelerated Cohort

> **Rahul Patle** · Solana Accelerated Builder · Q2 2026

A collection of Solana programs built during the **Turbin3 Accelerated Cohort (Q2 2026)**. This repository tracks all hands-on work across the cohort — from core Anchor patterns to advanced topics like Token-2022 Transfer Hooks, MagicBlock Ephemeral Rollups, and on-chain task scheduling with TukTuk.

---

## Tech Stack

| Layer | Tool |
|---|---|
| Language | Rust |
| Framework | Anchor |
| Token Standard | SPL Token + Token-2022 |
| Testing | LiteSVM / `anchor test` |
| Scheduling | TukTuk SDK |
| Real-time Layer | MagicBlock Ephemeral Rollup |
| Network | Solana Devnet / Localnet |

---

## Projects

### 1. `accel-p-escrow` — High-Performance Escrow

A blazing-fast, production-grade escrow program validated with **LiteSVM** instead of the standard `solana-test-validator`.

**What it does:**
- Implements the classic **maker/taker** paradigm for trustless token swaps
- Enforces strict chronological **time-locks** natively on-chain
- Maker deposits Token A into an escrow vault PDA; taker fulfills by sending Token B
- Escrow closes atomically once both sides are satisfied or is cancelled by maker

**Key concepts:** PDAs as vaults, CPI into SPL Token, time-lock constraints, LiteSVM for fast test feedback

---

### 2. `accel_anchor-esrow` — Anchor Escrow

A standard Anchor-based escrow showcasing clean account validation and constraint patterns.

**What it does:**
- Alice deposits tokens into an escrow account
- Bob fulfills the trade by sending the agreed token amount
- On success, both parties receive their expected tokens atomically
- Supports cancel flow to refund Alice if the trade is abandoned

**Key concepts:** `#[account]` constraints, `has_one`, `close`, CPI with `transfer_checked`

---

### 3. `anchor-core-staking` — Token Staking Program

An on-chain staking program that locks user tokens and tracks staking duration for reward calculations.

**What it does:**
- Users stake SPL tokens into a program-controlled vault PDA
- A `StakeInfo` account records the staker's deposit amount and start timestamp
- Implements unstake with basic time-based reward accrual logic
- Admin-controlled reward rate configurable at stake pool initialization

**Key concepts:** `Clock` sysvar, stake/unstake instructions, reward math with `checked_mul`/`checked_div`, vault PDAs

---

### 4. `anchor-fundraiser` — Crowdfunding Program

A decentralized fundraiser that lets project creators raise SPL tokens from backers with a deadline and goal.

**What it does:**
- Creator initializes a campaign with a target amount and deadline slot
- Backers contribute tokens; funds are held in a campaign vault PDA
- If goal is met before deadline → creator can claim funds
- If deadline passes with goal unmet → backers can refund their contributions

**Key concepts:** `Clock::get().unwrap().slot` comparisons, contributor receipt accounts, refund flows, escrow vaults

---

### 5. `generic-storage` — Generic On-Chain Data Storage

A minimal Anchor program demonstrating how to create, update, and read arbitrary structured data stored in PDAs.

**What it does:**
- Initializes a PDA-backed storage account with typed fields
- Provides `create`, `update`, and `close` instructions
- Demonstrates space calculation and `#[account(init)]` patterns correctly

**Key concepts:** `#[account(init, payer, space)]`, borsh serialization, account space math, realloc patterns

---

### 6. `magicblock-er-example` — MagicBlock Ephemeral Rollup

Integration example for **MagicBlock's Ephemeral Rollup (ER)** — Solana's real-time execution layer enabling sub-100ms transaction finality.

**What it does:**
- Initializes a base-layer Solana account that gets **delegated** to the ER
- State mutations happen on the ER at near-zero latency
- Demonstrates how to commit (checkpoint) ER state back to base-layer Solana
- Implements an agent PDA that processes AI/LLM oracle callbacks across MagicBlock boundaries

**Important note on tests:** Steps that require outbound connections to `devnet.magicblock.app` are marked with `it.skip()` due to IPv6 fetch disconnects in Node 18+. All core base-layer interactions are fully verified.

**Key concepts:** ER delegation, ephemeral sessions, state checkpointing, MagicBlock devnet integration

---

### 7. `todo-queue` — On-Chain Task Queue

A simple on-chain todo/task management system using a queue data structure stored in Solana PDAs.

**What it does:**
- Push new tasks onto a persistent queue stored in a PDA
- Mark tasks as complete with an indexed state transition
- Demonstrates ordered data management within Solana's account model
- Serves as a foundation for understanding task sequencing on-chain

**Key concepts:** Vec-backed PDA storage, ordered state transitions, borsh encoding of complex structs

---

### 8. `tuktuk-counter` — TukTuk On-Chain Cron Counter

A counter program that auto-increments on a schedule using **TukTuk** — Solana's decentralized on-chain task scheduler.

**What it does:**
- Initializes a `CounterAccount` PDA storing a u64 count value
- Registers a cron-style recurring task with the TukTuk protocol
- TukTuk cranks trigger the `increment` instruction automatically at set intervals
- Demonstrates dynamic epoch-based task queue naming to avoid PDA collisions

**Key concepts:** `@helium/tuktuk-sdk`, on-chain cron jobs, task scheduling via TukTuk CPI, epoch-aware PDA naming, VRF randomness integration

---

### 9. `whitelist-transfer-hook-q2` — Token-2022 Transfer Hook (Whitelist)

A **Token-2022** transfer hook that enforces a whitelist — only approved wallets can receive the token.

**What it does:**
- Implements the `TransferHook` interface from SPL Token-2022
- Stores a `WhitelistAccount` PDA per approved wallet address
- Every token transfer triggers an `Execute` instruction that checks whitelist membership
- Non-whitelisted destinations cause the transfer to fail atomically
- Admin instructions for adding/removing wallets from the whitelist

**Key concepts:** `transfer_hook` extension, `ExtraAccountMeta` list, `ExecuteInstruction`, Token-2022 CPI, `spl-transfer-hook-interface`

---

## Repository Structure

```
q2_26_accelerated_rahulpatle-sol/
│
├── accel-p-escrow/           # submodule — LiteSVM-tested, time-locked escrow
├── accel_anchor-esrow/       # submodule — standard Anchor escrow (Alice/Bob)
├── anchor-core-staking/      # submodule — SPL token staking + rewards
├── anchor-fundraiser/        # submodule — crowdfunding with deadline + refund
├── generic-storage/          # on-chain generic PDA data storage
├── magicblock-er-example/    # submodule — MagicBlock ER integration
├── todo-queue/               # on-chain task queue
├── tuktuk-counter/           # submodule — TukTuk cron counter
├── whitelist-transfer-hook-q2/ # submodule — Token-2022 whitelist hook
│
├── .gitmodules
├── .gitignore
└── README.md
```

---

## Prerequisites

```bash
# Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup component add rustfmt clippy

# Solana CLI
sh -c "$(curl -sSfL https://release.anza.xyz/stable/install)"

# Anchor CLI (via AVM)
cargo install --git https://github.com/coral-xyz/anchor avm --locked
avm install latest
avm use latest

# Node.js (for TypeScript tests)
nvm install 20
nvm use 20
```

---

## Setup

```bash
# Clone with all submodules
git clone --recurse-submodules https://github.com/rahulpatle-sol/q2_26_accelerated_rahulpatle-sol.git
cd q2_26_accelerated_rahulpatle-sol

# If already cloned, init submodules
git submodule update --init --recursive
```

### Build & Test any program

```bash
# Enter a program directory
cd anchor-core-staking

# Install dependencies (if TS tests exist)
yarn install

# Build
anchor build

# Run tests
anchor test
```

For `accel-p-escrow` specifically (uses LiteSVM):

```bash
cd accel-p-escrow
cargo test-sbf
```

### Solana Devnet Setup

```bash
solana config set --url devnet
solana-keygen new --outfile ~/.config/solana/id.json
solana airdrop 2
```

---

## Environment Variables

Some programs (TukTuk, MagicBlock) need RPC endpoints. Create a `.env` in the relevant submodule:

```env
ANCHOR_WALLET=~/.config/solana/id.json
ANCHOR_PROVIDER_URL=https://api.devnet.solana.com
TUKTUK_RPC=https://devnet.helius-rpc.com/?api-key=YOUR_KEY
```

---

## Cohort Context

This repository is part of the **Turbin3 Accelerated Cohort** — an intensive Solana program development track that goes beyond the standard Builders program. The accelerated path covers:

- Advanced Anchor patterns and security constraints
- Token-2022 extensions (Transfer Hooks, Confidential Transfers)
- On-chain automation with TukTuk
- Real-time Solana with MagicBlock Ephemeral Rollups
- Production-grade testing strategies (LiteSVM)

---

## Author

**Rahul Patle**  
Full-Stack & Solana Developer · Turbin3 Q2 2026 Accelerated Cohort  
GitHub: [@rahulpatle-sol](https://github.com/rahulpatle-sol)

---
Thank  you  to  Turbine3 team"
Anderia , Shrinath ,jeff, berg, and Adamola..
*Built with ⚓ Anchor + 🦀 Rust on Solana*
