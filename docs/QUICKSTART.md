# Neko-Claw 快速入门指南 🐾⚡

> 5 分钟上手高性能 Rust AI 助手框架

## 📋 目录

- [前置条件](#前置条件)
- [安装](#安装)
- [第一个命令](#第一个命令)
- [配置](#配置)
- [常见问题](#常见问题)

---

## 前置条件

- **Rust**: 1.70+ (推荐 1.93+)
- **系统**: Linux / macOS / Windows (WSL2)
- **内存**: 最低 100MB RAM (目标 <20MB)

### 检查 Rust 版本

```bash
rustc --version
```

如果没有安装 Rust，请使用 [rustup](https://rustup.rs/) 安装：

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

---

## 安装

### 方式 1: 克隆仓库（推荐）

```bash
# 克隆仓库
git clone https://github.com/your-org/nekoclaw.git
cd nekoclaw

# 构建发布版本
cargo build --release

# 二进制文件位置
./target/release/nekoclaw --help
```

### 方式 2: Cargo 安装（未来）

```bash
# 即将支持
cargo install nekoclaw
```

### 构建优化

为了获得最小二进制尺寸，使用以下优化：

```bash
# 1. 创建发布构建
cargo build --release

# 2. Strip 二进制（可选，进一步减少体积）
strip target/release/nekoclaw

# 3. 检查二进制大小
ls -lh target/release/nekoclaw
```

**预期尺寸**: <2.5MB

---

## 第一个命令

### 1. 查看帮助

```bash
nekoclaw --help
```

输出：

```
Neko-Claw v0.1.0 - 高性能 Rust AI 助手框架

USAGE:
    nekoclaw [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -c, --config <CONFIG>    配置文件路径 (默认: ~/.nekoclaw/config.json)
    -v, --verbose           详细输出
    -h, --help              显示帮助信息

SUBCOMMANDS:
    start       启动 Neko-Claw 服务
    status      查看运行状态
    stop        停止服务
    restart     重启服务
    config      配置管理
    test        运行测试
    version     显示版本信息
```

### 2. 查看版本

```bash
nekoclaw version
```

输出：

```
Neko-Claw v0.1.0
Rust: 1.93.1
```

### 3. 运行测试

```bash
nekoclaw test
```

这将运行所有单元测试和集成测试。

---

## 配置

### 1. 配置文件位置

默认配置文件：`~/.nekoclaw/config.json`

如果不存在，首次启动时会自动创建。

### 2. 最小配置示例

创建 `~/.nekoclaw/config.json`：

```json
{
  "version": "1.0.0",
  "gateway": {
    "host": "localhost",
    "port": 8080
  },
  "models": {
    "providers": {
      "nvidia": {
        "apiKey": "your-api-key-here"
      }
    }
  },
  "channels": {
    "discord": {
      "accounts": {
        "main_bot": {
          "token": "your-discord-token"
        }
      }
    }
  },
  "agents": {
    "defaults": {
      "model": {
        "primary": "nvidia/z-ai/glm4.7"
      }
    }
  }
}
```

### 3. 从 OpenClaw 迁移配置

如果已有 OpenClaw 配置，直接复制：

```bash
# 复制 OpenClaw 配置
cp ~/.openclaw/openclaw.json ~/.nekoclaw/config.json

# 验证配置（可选）
nekoclaw config validate
```

详细迁移指南请参考：[MIGRATION.md](./MIGRATION.md)

---

## 启动服务

### 前台运行（开发模式）

```bash
nekoclaw start
```

输出：

```
[INFO] Neko-Claw v0.1.0 starting...
[INFO] Loaded config from ~/.nekoclaw/config.json
[INFO] Initializing providers...
[INFO] Starting gateway server on localhost:8080...
[INFO] Neko-Claw is running! 🚀
```

### 后台运行（生产模式）

```bash
# 使用 nohup
nohup nekoclaw start > nekoclaw.log 2>&1 &

# 或使用 systemd（推荐）
sudo systemctl enable nekoclaw
sudo systemctl start nekoclaw
```

### 查看运行状态

```bash
nekoclaw status
```

输出：

```
Neko-Claw Status:
  Version:    v0.1.0
  Status:     Running
  Uptime:     2h 15m
  Memory:     18.5MB / 20MB
  Services:   5/5 active
```

---

## 常见问题

### ❓ 为什么启动失败？

**可能原因 1: 配置文件缺失或格式错误**

```bash
# 验证配置
nekoclaw config validate
```

**可能原因 2: 端口被占用**

修改配置文件中的端口：

```json
{
  "gateway": {
    "port": 9000  // 改为其他端口
  }
}
```

**可能原因 3: API Key 无效**

检查 `models.providers.*.apiKey` 是否正确。

---

### ❓ 如何查看详细日志？

使用 `--verbose` 标志：

```bash
nekoclaw start --verbose
```

或查看日志文件：

```bash
# 默认日志位置
tail -f ~/.nekoclaw/nekoclaw.log
```

---

### ❓ 内存使用超过 20MB？

检查是否有未关闭的会话：

```bash
nekoclaw config list-sessions
```

清理过期会话：

```bash
nekoclaw config cleanup
```

---

### ❓ 如何升级到最新版本？

```bash
# 拉取最新代码
git pull origin main

# 重新构建
cargo build --release

# 重启服务
nekoclaw restart
```

---

### ❓ 支持哪些 Provider？

当前支持的 Provider：

- ✅ OpenAI
- ✅ Anthropic (Claude)
- ✅ OpenRouter
- ✅ NVIDIA (推荐，高性能)

查看完整列表：[USAGE.md](./USAGE.md)

---

### ❓ 如何联系支持？

- 📚 [文档中心](https://docs.nekoclaw.ai)
- 💬 [Discord 社区](https://discord.gg/nekoclaw)
- 🐛 [GitHub Issues](https://github.com/your-org/nekoclaw/issues)

---

## 下一步

- 📖 阅读完整使用指南：[USAGE.md](./USAGE.md)
- 🔧 配置迁移指南：[MIGRATION.md](./MIGRATION.md)
- 🏗️ 架构文档：[../ARCHITECTURE.md](../ARCHITECTURE.md)
- 🔒 安全文档：[./SECURITY.md](./SECURITY.md)

---

**祝使用愉快！** 🐾⚡

*Neko-Claw - 零开销 Rust AI 助手框架*
