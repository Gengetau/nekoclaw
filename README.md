# 🐾 Neko-Claw (猫爪核心)

**项目代号**: nekoclaw
**版本**: v0.1.0 (Development)
**状态**: 🚀 Phase 0 - 环境准备中

---

## 📋 项目概述

Neko-Claw 是一个**100% 原研 Rust** 的高性能 AI 助手核心，专为低资源环境（2GB 内存服务器）优化。

### 核心优势

| 指标 | OpenClaw (Node) | nekoclaw (Rust) | 提升 |
|------|-----------------|-----------------|------|
| 内存占用 | 1.52 GB | < 20 MB | **98.7%** ↓ |
| 冷启动时间 | 3.31s | < 500ms | **85%** ↓ |
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
- **Tool**: 工具扩展 (Shell, File, Memory Recall)

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

# 启动 Discord 机器人
nekoclaw agent

# 启动 Web Gateway
nekoclaw gateway

# 查看系统状态
nekoclaw status
```

---

## 📚 文档

- [架构设计草案](docs/ARCHITECTURE.md) - 完整的 Trait 抽象层和模块解耦设计
- [安全与权限白皮书](docs/SECURITY.md) - 纵深防御策略和闭源混淆方案
- [高性能并发调研](docs/PERFORMANCE.md) - Tokio 异步运行时和性能测试报告
- [综合提案](PROPOSAL.md) - v1.0 完整项目提案

---

## 🛡️ 安全特性

- **Rust 编译级安全**: 所有权系统、借用检查器拦截 99% 内存漏洞
- **命令注入防护**: Shell 工具白名单、参数过滤
- **文件系统沙箱**: 强制 workspace 限制、禁止路径黑名单
- **Discord 白名单**: 仅允许授权用户发送指令
- **闭源混淆**: 编译时字符串混淆、符号剥离、配置加密

---

## 📝 开发路线图

### Phase 1: 基础架构 (3-5 天)
- [x] 草案完成
- [ ] 实现 core/traits.rs
- [ ] 实现 core/config.rs
- [ ] CLI 框架 (Clap)

### Phase 2: Provider & Memory (2-3 天)
- [ ] OpenAI Provider
- [ ] SQLite Memory + FTS5
- [ ] 向量相似度计算

### Phase 3: Gateway & Channel (2-3 天)
- [ ] Axum HTTP server
- [ ] Discord bot (初版)
- [ ] Pairing mechanism

### Phase 4: 安全加固 (2-3 天)
- [ ] Shell 白名单
- [ ] 文件系统沙箱
- [ ] 代码混淆

**总计**: 19-27 天 (3-4 周)

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

**🐾 喵...欢迎来到猫爪核心喵...** 💜
