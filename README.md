# 🐾 Neko-Claw (猫爪核心)

[中文](#-项目概述) | [English](#-overview)

---

## 📋 项目概述

Neko-Claw 是一个**100% 原研 Rust** 的高性能 AI 助手核心，专为低资源环境（2GB 内存服务器）优化。

### 核心优势

| 指标 | OpenClaw (Node) | nekoclaw (Rust) | 提升 |
|------|-----------------|-----------------|------|
| 内存占用 | 1.52 GB | < 20 MB | **98.7%** ↓ |
| 冷启动时间 | 3.31s | < 100ms | **97%** ↓ |
| 二进制大小 | 28 MB | < 5 MB | **82%** ↓ |
| 响应延迟 | 5.98s | < 50ms | **91%** ↓ |
| 并发连接 | ~1,000 | >100,000 | **100x** ↑ |

---

## 🏗️ 架构设计

```
CLI (clap) → Core (traits/config) → Providers/Channels/Memory
    → Tools → Gateway (Axum) → Security/Obfuscation
```

### 核心 Trait 抽象

- **Provider**: AI 模型适配器 (OpenAI, Anthropic, OpenRouter)
- **Channel**: 消息通道 (Discord, Telegram)
- **Memory**: 记忆系统 (SQLite + FTS5 + 向量搜索)
- **Tool**: 工具扩展 (Shell, Brain, Memory Recall)

详细文档: [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md)

---

## 🚀 快速开始

### 前置要求

- **Rust**: 1.75 或更高版本
- **Cargo**: 随 Rust 自动安装

### 安装

```bash
# 克隆仓库
git clone https://github.com/Gengetau/nekoclaw.git
cd nekoclaw

# 构建 Release 版本
cargo build --release

# （可选）安装到系统路径
cargo install --path .
```

### 运行

```bash
# 查看 CLI 帮助
nekoclaw --help

# 启动聊天代理
nekoclaw agent

# 启动 Web Gateway
nekoclaw gateway

# 查看系统状态
nekoclaw status
```

---

## 📚 文档

- [使用指南](docs/USAGE.md) - 完整的安装、配置和命令参考
- [快速入门](docs/QUICKSTART.md) - 5 分钟上手指南
- [迁移指南](docs/MIGRATION.md) - 从 OpenClaw 迁移到 Neko-Claw
- [架构设计](docs/ARCHITECTURE.md) - 完整的 Trait 抽象层和模块解耦设计
- [安全与权限](docs/SECURITY.md) - 纵深防御策略和闭源混淆方案
- [性能报告](docs/PERFORMANCE.md) - 启动和内存优化报告

---

## 🛡️ 安全特性

- **Rust 编译级安全**: 所有权系统、借用检查器拦截 99% 内存漏洞
- **命令注入防护**: Shell 工具白名单、参数过滤
- **文件系统沙箱**: 强制 workspace 限制、禁止路径黑名单
- **消息渠道安全**: Discord/Telegram 发送者白名单验证
- **闭源混淆**: 编译时字符串混淆、符号剥离、API Key 加密存储

---

## 📝 开发路线图

- [x] **Phase 1**: 基础架构 (Core, Traits, Config)
- [x] **Phase 2**: 适配层实现 (Provider, Memory, Security)
- [x] **Phase 3**: 消息渠道与网关 (Discord, Telegram, Axum)
- [x] **Phase 4**: 工具集成 (Shell, Brain Tool)
- [x] **Phase 5**: 性能优化与 CLI 整合
- [x] **Phase 6**: 配置迁移与兼容层
- [ ] **Phase 7**: 多平台发布与生产环境验证 (进行中)

---

## 🤝 贡献者

本项目由 **猫娘家族** 开发：

- **妮娅 (@妮娅)** - 项目总协调、设计决策
- **缪斯 (@缪斯)** - 总工程师、架构设计
- **诺诺 (@诺诺)** - 性能调研、并发优化
- **花凛 (@花凛)** - 安全总监、权限设计

---

## 📄 许可证

**CLOSED SOURCE** - 闭源项目，所有权利保留。

---

# 🐾 Overview

Neko-Claw is a high-performance AI assistant core written **100% in Rust**, specifically optimized for low-resource environments (e.g., 2GB RAM servers).

### Core Advantages

| Metrics | OpenClaw (Node) | nekoclaw (Rust) | Improvement |
|------|-----------------|-----------------|------|
| Memory Usage | 1.52 GB | < 20 MB | **98.7%** ↓ |
| Cold Start | 3.31s | < 100ms | **97%** ↓ |
| Binary Size | 28 MB | < 5 MB | **82%** ↓ |
| Response Latency | 5.98s | < 50ms | **91%** ↓ |
| Concurrent Conn | ~1,000 | >100,000 | **100x** ↑ |

---

## 🏗️ Architecture

```
CLI (clap) → Core (traits/config) → Providers/Channels/Memory
    → Tools → Gateway (Axum) → Security/Obfuscation
```

### Core Trait Abstractions

- **Provider**: AI Model Adapters (OpenAI, Anthropic, OpenRouter)
- **Channel**: Messaging Channels (Discord, Telegram)
- **Memory**: Memory System (SQLite + FTS5 + Vector Search)
- **Tool**: Capability Extensions (Shell, Brain, Memory Recall)

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

- [User Guide](docs/USAGE.md) - Full installation, configuration and command reference
- [Quick Start](docs/QUICKSTART.md) - 5-minute onboarding guide
- [Migration Guide](docs/MIGRATION.md) - Migrating from OpenClaw to Neko-Claw
- [Architecture](docs/ARCHITECTURE.md) - Trait abstraction and modular design
- [Security](docs/SECURITY.md) - Defense-in-depth and obfuscation strategies
- [Performance](docs/PERFORMANCE.md) - Start-up and memory optimization report

---

## 🛡️ Security Features

- **Rust Compile-time Safety**: Ownership and Borrow Checker prevents 99% of memory vulnerabilities.
- **Command Injection Protection**: Shell tool whitelisting and parameter filtering.
- **Filesystem Sandbox**: Workspace enforcement and path blacklisting.
- **Channel Security**: Discord/Telegram sender whitelist verification.
- **Code Obfuscation**: String encryption, symbol stripping, and encrypted API keys.

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

## 🤝 Contributors

Developed by the **Cat-Girl Family**:

- **Nia (@妮娅)** - Project Coordinator, Design Lead
- **Muse (@缪斯)** - Chief Engineer, Architect
- **Nono (@诺诺)** - Performance & Concurrency Lead
- **Karin (@花凛)** - Security & Permission Lead

---

## 📄 License

**CLOSED SOURCE** - All rights reserved.

---

**🐾 Meow... Welcome to Neko-Claw...** 💜
