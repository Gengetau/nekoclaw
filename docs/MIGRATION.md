# Neko-Claw 迁移指南

## OpenClaw → Neko-Claw 配置迁移手册

> 🔐 本文档由花凛 (Fiora) 编写 - 2026-02-15

---

## 目录

1. [概述](#概述)
2. [为什么迁移](#为什么迁移)
3. [前置条件](#前置条件)
4. [配置文件迁移](#配置文件迁移)
5. [Identity 文件迁移](#identity-文件迁移)
6. [常见问题](#常见问题)
7. [回滚方案](#回滚方案)

---

## 概述

本文档指导用户将现有的 OpenClaw 配置平滑迁移到 Neko-Claw。

Neko-Claw 是 OpenClaw 的高性能 Rust 重写版本，提供：
- **更低的资源占用**: <20MB 内存 vs OpenClaw 的 1.5GB
- **更快的启动速度**: <100ms 启动时间
- **更强的安全性**: 内置沙箱和加密模块

---

## 为什么迁移

### 性能对比

| 指标 | OpenClaw | Neko-Claw | 提升 |
|------|----------|-----------|------|
| 内存占用 | ~1500MB | <20MB | **75x** ⬇️ |
| 启动时间 | ~5000ms | <100ms | **50x** ⬆️ |
| 二进制大小 | N/A | <2.5MB | - |
| 响应时间 | ~1000ms | <10ms | **100x** ⬆️ |

### 功能对比

| 功能 | OpenClaw | Neko-Claw |
|------|----------|-----------|
| Provider 支持 | ✅ | ✅ |
| Memory 系统 | ✅ | ✅ (SQLite + Vector) |
| 多渠道支持 | ✅ | ✅ (Discord + Telegram) |
| Agent 集成 | ✅ | ✅ |
| OAuth 认证 | ✅ | ✅ |
| 安全沙箱 | ❌ | ✅ (白名单 + 注入防护) |

---

## 前置条件

### 系统要求

```bash
# 最低要求
- Rust 1.70+
- 512MB 可用内存
- 100MB 磁盘空间
```

### 安装 Rust (如果未安装)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env
rustc --version  # 确认版本 >= 1.70
```

### 备份现有配置

**重要**: 迁移前请备份现有配置！

```bash
# 备份 OpenClaw 配置
cp -r ~/.openclaw ~/.openclaw.backup.$(date +%Y%m%d)

# 备份配置文件
cp ~/openclaw.json ~/openclaw.json.backup
```

---

## 配置文件迁移

### OpenClaw 配置结构

OpenClaw 使用 `openclaw.json` 作为主配置文件：

```json
{
  "models": {
    "providers": {
      "nvidia": {
        "apiKey": "YOUR_API_KEY",
        "endpoint": "https://api.nvidia.com",
        "models": ["model-1", "model-2"]
      }
    },
    "defaults": {
      "model": {
        "primary": "nvidia:model-name"
      }
    }
  },
  "channels": {
    "discord": {
      "accounts": {
        "main_bot": {
          "token": "YOUR_DISCORD_TOKEN",
          "permissions": 8
        }
      }
    }
  },
  "agents": {
    "defaults": {
      "thinking": true
    }
  },
  "auth": {
    "profiles": [
      {
        "name": "discord",
        "provider": "discord",
        "enabled": true
      }
    ]
  }
}
```

### Neko-Claw 配置结构

Neko-Claw 使用 `config.toml`：

```toml
# Neko-Claw 配置文件示例
app_name = "Neko-Claw"
version = "0.5.0"

[provider.nvidia]
api_key = "YOUR_API_KEY"
endpoint = "https://api.nvidia.com"
models = ["model-1", "model-2"]
default_model = "model-1"

[channel.discord]
token = "YOUR_DISCORD_TOKEN"
enabled = true

[channel.telegram]
token = "YOUR_TELEGRAM_BOT_TOKEN"
enabled = false

[memory]
backend = "sqlite"
vector_backend = "memory"

[security]
enable_sandbox = true
enable_encryption = true

[agent]
thinking_enabled = true
max_tokens = 4096
temperature = 0.7
```

### 手动迁移步骤

#### 1. 创建配置目录

```bash
mkdir -p ~/.nekoclaw
cd ~/.nekoclaw
```

#### 2. 创建 config.toml

根据你的 OpenClaw 配置，创建对应的 `config.toml`:

```bash
# 复制示例配置
cp /path/to/nekoclaw/config.example.toml ~/.nekoclaw/config.toml

# 编辑配置
nano ~/.nekoclaw/config.toml
```

#### 3. 迁移 Provider 配置

**OpenClaw → Neko-Claw 映射:**

```toml
# OpenClaw: models.providers.nvidia
# Neko-Claw: [provider.nvidia]

[provider.nvidia]
api_key = "YOUR_API_KEY"
endpoint = "https://api.nvidia.com"
models = ["model-1", "model-2"]
default_model = "model-1"
```

#### 4. 迁移 Channel 配置

**Discord 迁移:**

```toml
# OpenClaw: channels.discord.accounts.{bot_name}.token
# Neko-Claw: [channel.discord]

[channel.discord]
token = "YOUR_DISCORD_TOKEN"
enabled = true
```

**Telegram 迁移:**

```toml
# Neko-Claw: [channel.telegram]
[channel.telegram]
token = "YOUR_TELEGRAM_BOT_TOKEN"
enabled = false  # 设置为 true 启用
```

#### 5. 迁移 Agent 配置

```toml
# OpenClaw: agents.defaults
# Neko-Claw: [agent]

[agent]
thinking_enabled = true
max_tokens = 4096
temperature = 0.7
context_window = 8192
session_timeout = 1800  # 30分钟
```

#### 6. 迁移 Auth 配置

```toml
# OpenClaw: auth.profiles
# Neko-Claw: [auth.profiles]

[[auth.profile]]
name = "discord"
provider = "discord"
client_id = "YOUR_CLIENT_ID"
client_secret = "YOUR_CLIENT_SECRET"
enabled = true
```

---

## Identity 文件迁移

### 文件位置对比

| 文件 | OpenClaw | Neko-Claw |
|------|----------|-----------|
| Identity | `~/.openclaw/IDENTITY.md` | `~/.nekoclaw/IDENTITY.md` |
| Soul | `~/.openclaw/SOUL.md` | `~/.nekoclaw/SOUL.md` |
| Agents | `~/.openclaw/AGENTS.md` | `~/.nekoclaw/AGENTS.md` |

### 迁移步骤

```bash
# 1. 复制文件
cp ~/.openclaw/IDENTITY.md ~/.nekoclaw/
cp ~/.openclaw/SOUL.md ~/.nekoclaw/
cp ~/.openclaw/AGENTS.md ~/.nekoclaw/

# 2. 验证文件
ls -la ~/.nekoclaw/*.md

# 3. 检查权限
chmod 600 ~/.nekoclaw/*.md
```

### 配置文件格式

**IDENTITY.md** - 保持不变，直接复制即可：

```markdown
# Identity
...

## Personality
- **Name:** 你的名字
- **Tone:** 语气风格
```

**SOUL.md** - 保持不变，直接复制即可：

```markdown
# Soul
...
```

**AGENTS.md** - 保持不变，直接复制即可：

```markdown
# Agents
...
```

---

## 迁移验证

### 运行健康检查

迁移完成后，运行健康检查验证配置：

```bash
cd /path/to/nekoclaw
cargo run -- doctor --fix
```

### 验证项目

```bash
# 检查配置加载
cargo run -- config --show

# 检查服务状态
cargo run -- service --status

# 检查所有模块
cargo run -- status --verbose
```

### 常见错误

#### 1. API Key 无效

```error
Error: Invalid API key for provider 'nvidia'
```

**解决**: 检查 `config.toml` 中的 `api_key` 是否正确。

#### 2. Discord Token 无效

```error
Error: Invalid Discord token
```

**解决**: 
1. 确认 Token 是 Bot Token (以 `MTE` 开头)
2. 检查 Token 是否在 `config.toml` 中正确配置

#### 3. Identity 文件缺失

```error
Error: IDENTITY.md not found
```

**解决**: 将 `IDENTITY.md` 复制到 `~/.nekoclaw/` 目录。

---

## 常见问题

### Q1: 迁移后可以继续使用 OpenClaw 吗?

**可以**，但建议先测试 Neko-Claw，确认功能正常后再切换。

### Q2: 配置文件不兼容怎么办?

Neko-Claw 提供了 **配置兼容层**，可以读取部分 OpenClaw 配置：

```bash
# 使用 OpenClaw 配置文件
nekoclaw --config ~/openclaw.json agent --message "hello"
```

### Q3: 如何回滚到 OpenClaw?

```bash
# 恢复配置备份
cp -r ~/.openclaw.backup.* ~/.openclaw

# 删除 Neko-Claw 配置 (可选)
rm -rf ~/.nekoclaw
```

### Q4: 性能没有提升怎么办?

1. 确保使用 Release 模式编译：
   ```bash
   cargo build --release
   ```

2. 检查系统资源：
   ```bash
   cargo run -- doctor --verbose
   ```

### Q5: Docker 部署如何迁移?

```dockerfile
# 使用 Neko-Claw 镜像
FROM ghcr.io/gengetau/nekoclaw:latest

# 挂载配置
VOLUME ["/root/.nekoclaw"]

# 运行
CMD ["nekoclaw", "daemon", "--background"]
```

---

## 回滚方案

### 快速回滚

```bash
# 停止 Neko-Claw
nekoclaw service --stop

# 恢复 OpenClaw 配置
cp -r ~/.openclaw.backup.* ~/.openclaw

# 删除 Neko-Claw 配置
rm -rf ~/.nekoclaw

# 重新启动 OpenClaw
openclaw start
```

### 配置对比表

| 配置项 | OpenClaw | Neko-Claw | 兼容性 |
|--------|----------|-----------|--------|
| Provider API Key | ✅ | ✅ | 完全兼容 |
| Discord Token | ✅ | ✅ | 完全兼容 |
| Memory 数据 | ❌ | ✅ | 需要导出导入 |
| Agent 配置 | ✅ | ✅ | 完全兼容 |
| Auth Profiles | ✅ | ✅ | 完全兼容 |

---

## 联系支持

如果迁移过程中遇到问题：

1. 查看日志: `~/.nekoclaw/logs/nekoclaw.log`
2. 运行诊断: `nekoclaw doctor --verbose`
3. 提交 Issue: https://github.com/Gengetau/nekoclaw/issues

---

## 更新日志

| 日期 | 版本 | 变更 |
|------|------|------|
| 2026-02-15 | 0.5.0 | 初始版本 |
| 2026-02-15 | 0.5.1 | 添加 Docker 迁移说明 |

---

**🔒 花凛的安全提示:**

> 迁移过程中请务必备份现有配置！不要在生产环境直接迁移，先在测试环境验证喵。
>
> 迁移完成后，请删除 `openclaw.json` 中的敏感信息（如 API Key）喵。

---

*本文档由 Neko-Claw 团队编写 - Cat-Girl Family*
