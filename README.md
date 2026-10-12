# Solana Jupiter Route Optimizer (SJRO)

<p align="center">
  <img src="assets/architecture.svg" alt="Solana Jupiter Route Optimizer Architecture" width="780" />
</p>

[![License: Apache 2.0](https://img.shields.io/badge/License-Apache_2.0-blue.svg)](LICENSE)
[![Network: Solana Mainnet & Devnet](https://img.shields.io/badge/Solana-Anchor_Framework-14F195?logo=solana)](https://solana.com)
[![DEX: Jupiter v6 & Raydium](https://img.shields.io/badge/DEX-Jupiter_v6_%26_Raydium-38BDF8.svg)](https://jup.ag)
[![TypeScript: 5.4](https://img.shields.io/badge/TypeScript-5.4-3178C6?logo=typescript)](https://www.typescriptlang.org)

**Solana Jupiter Route Optimizer (SJRO)** is a production-grade Solana Anchor protocol and TypeScript client engine providing **multi-hop DEX route discovery, dynamic slippage control, and atomic limit order execution** using the Jupiter v6 swap API and Raydium concentrated liquidity pools.

---

## ⚡ Core Capabilities

1. **Multi-Hop Split-Route Discovery**: Compares deep liquidity across Orca Whirlpools, Raydium CLMM, and Phoenix orderbooks to maximize net output.
2. **On-Chain Settlement Verification**: Anchor program enforces minimum received token amounts and executes atomic PDA token escrow releases.
3. **Dynamic Priority Fee Injection**: Integrates rolling compute-budget fee estimation to ensure timely execution during high-congestion slots.

---

## 🛠 Smart Contracts & Client SDK

- **`programs/route_optimizer/src/lib.rs`**: Anchor Rust program (`Anchor 0.30+`):
  - `initialize_swap_vault`: Sets authorized swap authorities and fee split tiers.
  - `execute_routed_swap`: Verifies pre- and post-swap balances against caller slippage bounds.
- **`src/route_evaluator.ts`**: Pure TypeScript Pareto-optimal path selection engine.
- **`src/jupiter_client.ts`**: High-performance transaction bundle builder with `@solana/web3.js`.
- **`tests/route.test.ts`**: Automated unit test suite.

---

## 🚀 Quick Start

### Installation

```bash
git clone https://github.com/dishonesthero/solana-jupiter-route-optimizer.git
cd solana-jupiter-route-optimizer
npm install
npm test
```

---

## 📜 License

Licensed under the [Apache License, Version 2.0](LICENSE).
