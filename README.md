# 🐾 Neko-Claw

**A High-Performance AI Assistant Core in Rust**

[![Rust](https://img.shields.io/badge/rust-1.75%2B-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-CLOSED--SOURCE-red.svg)](#license)

**Version**: v0.1.0 (Development)  
**Status**: Phase 1-6 Complete ✅

---

## 📋 Overview

Neko-Claw is a high-performance AI assistant core written **100% in Rust**, specifically optimized for low-resource environments (e.g., 2GB RAM servers).

### Core Advantages

| Metrics | OpenClaw (Node) | Neko-Claw (Rust) | Improvement |
|---------|-----------------|------------------|-------------|
| Memory Usage | 1.52 GB | < 20 MB | **98.7%** ↓ |
| Cold Start | 3.31s | < 100ms | **97%** ↓ |
| Binary Size | 28 MB | < 5 MB | **82%** ↓ |
| Response Latency | 5.98s | < 50ms | **91%** ↓ |
| Concurrent Connections | ~1,000 | >100,000 | **100x** ↑ |

---

## 🏗️ Architecture

```
CLI (clap) → Core (traits/config) → Providers/Channels/Memory
    → Tools → Gateway (Axum) → Security/Obfuscation
```

### Core Trait Abstractions

- **Provider**: AI Model Adapters (OpenAI, Anthropic, OpenRouter, NVIDIA NIM)
- **Channel**: Messaging Channels (Discord, Telegram)
- **Memory**: Memory System (SQLite + FTS5 + Vector Search)
- **Tool**: Capability Extensions (Shell, Brain, Memory Recall)
- **Security**: Encryption, Sandbox, Whitelist

Detailed Docs: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

---

## 🚀 Quick Start

### Prerequisites

- **Rust**: 1.75 or higher
- **Cargo**: Installed automatically with Rust

### Installation

```bash
# Clone the repository
git clone https://github.com/Gengetau/nekoclaw.git
cd nekoclaw

# Build Release version
cargo build --release

# (Optional) Install to system path
cargo install --path .
```

### Running

```bash
# Show CLI help
nekoclaw --help

# Start Agent mode
nekoclaw agent

# Start Web Gateway
nekoclaw gateway

# Check System Status
nekoclaw status
```

---

## 📚 Documentation

| Document | Description |
|----------|-------------|
| [User Guide](docs/USAGE.md) | Full installation, configuration and command reference |
| [Quick Start](docs/QUICKSTART.md) | 5-minute onboarding guide |
| [Migration Guide](docs/MIGRATION.md) | Migrating from OpenClaw to Neko-Claw |
| [Architecture](docs/ARCHITECTURE.md) | Trait abstraction and modular design |
| [Security](docs/SECURITY.md) | Defense-in-depth and obfuscation strategies |
| [Performance](docs/PERFORMANCE.md) | Start-up and memory optimization report |

---

## 🛡️ Security Features

- **Rust Compile-time Safety**: Ownership and Borrow Checker prevents 99% of memory vulnerabilities
- **Command Injection Protection**: Shell tool whitelisting and parameter filtering
- **Filesystem Sandbox**: Workspace enforcement and path blacklisting
- **Channel Security**: Discord/Telegram sender whitelist verification
- **Code Obfuscation**: String encryption, symbol stripping, and encrypted API keys (AES-256-GCM)

---

## 📝 Roadmap

- [x] **Phase 1**: Base Infrastructure (Core, Traits, Config)
- [x] **Phase 2**: Adapter Implementation (Provider, Memory, Security)
- [x] **Phase 3**: Channels & Gateway (Discord, Telegram, Axum)
- [x] **Phase 4**: Tool Integration (Shell, Brain Tool)
- [x] **Phase 5**: Performance Optimization & CLI Integration
- [x] **Phase 6**: Migration & Compatibility Layer
- [ ] **Phase 7**: Multi-platform Release & Production Testing (In Progress)

---

## 📊 Project Stats

| Category | Count |
|----------|-------|
| Rust Source Code | ~10,217 lines |
| Unit Tests | 24 |
| Benchmark Tests | 8 |
| Documentation Files | 7 |
| Total Files | 75 |

---

## 🤝 Contributors

Developed by the **Neko-Claw Team**:

- **Nia** - Project Coordinator, Design Lead
- **Muse** - Chief Engineer, Architect
- **Nono** - Performance & Concurrency Lead
- **Karin** - Security & Permission Lead

---

## 📄 License

**CLOSED SOURCE** - All rights reserved.

Unauthorized copying, distribution, or modification of this software is strictly prohibited.

---

**🐾 Meow... Welcome to Neko-Claw...** 💜
