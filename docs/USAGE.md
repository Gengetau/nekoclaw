# Neko-Claw 使用指南

## 目录

- [快速开始](#快速开始)
- [安装](#安装)
- [配置](#配置)
- [命令行界面](#命令行界面)
- [使用示例](#使用示例)
- [常见问题](#常见问题)

---

## 快速开始

### 1. 克隆项目

```bash
git clone https://github.com/Gengetau/nekoclaw.git
cd nekoclaw
```

### 2. 构建

```bash
cargo build --release
```

### 3. 配置

复制 OpenClaw 配置文件到 Neko-Claw 配置目录：

```bash
mkdir -p ~/.nekoclaw
cp ~/.openclaw/openclaw.json ~/.nekoclaw/config.json
```

### 4. 运行

```bash
./target/release/nekoclaw start
```

---

## 安装

### 系统要求

- Rust 1.75+
- SQLite 3.35+ (用于 Memory)
- 2GB+ 可用内存

### 从源码构建

```bash
# 克隆仓库
git clone https://github.com/Gengetau/nekoclaw.git
cd nekoclaw

# 构建 release 版本
cargo build --release

# 安装到系统
cargo install --path .
```

### 下载预编译二进制

访问 [GitHub Releases](https://github.com/Gengetau/nekoclaw/releases) 下载对应平台的二进制文件。

---

## 配置

### 配置文件位置

- Linux/macOS: `~/.nekoclaw/config.json`
- Windows: `%USERPROFILE%\.nekoclaw\config.json`

### 配置文件格式

Neko-Claw 完全兼容 OpenClaw `openclaw.json` 格式。支持以下配置项：

#### 基础配置

```json
{
  "config": {
    "version": "1.0.0",
    "gateway": {
      "host": "0.0.0.0",
      "port": 18789,
      "enabled": true
    },
    "agents": {
      "default": "miau",
      "agent": {
        "miau": {
          "id": "miau",
          "name": "缪斯",
          "model": "gpt-4",
          "memory": {
            "kind": "sqlite",
            "path": "~/.nekoclaw/memory.db"
          },
          "tools": ["shell", "web-search"]
        }
      }
    }
  }
}
```

#### Provider 配置

```json
{
  "config": {
    "models": {
      "default": "gpt-4",
      "providers": {
        "anthropic": {
          "enabled": true,
          "apiKey": "sk-ant-xxxx",
          "baseUrl": "https://api.anthropic.com",
          "model": "claude-3-opus-20240229",
          "models": [
            {
              "id": "claude-3-opus-20240229",
              "name": "Claude 3 Opus",
              "context_length": 200000
            }
          ]
        },
        "openai": {
          "enabled": true,
          "apiKey": "sk-xxxx",
          "baseUrl": "https://api.openai.com/v1",
          "model": "gpt-4"
        },
        "openrouter": {
          "enabled": true,
          "apiKey": "sk-or-xxxx",
          "baseUrl": "https://openrouter.ai/api/v1"
        }
      }
    }
  }
}
```

#### Channel 配置 (多账户)

```json
{
  "config": {
    "channels": {
      "discord": {
        "enabled": true,
        "accounts": {
          "main_bot": {
            "token": "MTE...xxx",
            "allowed_channels": ["123456789012345678"],
            "allowed_users": ["123456789012345678"],
            "prefix": "!"
          },
          "test_bot": {
            "token": "MTE...xxx",
            "allowed_channels": ["987654321098765432"],
            "prefix": "?"
          }
        }
      },
      "telegram": {
        "enabled": true,
        "token": "123456789:ABCdefGHIjklMNOpqrsTUVwxyz",
        "allowed_users": ["123456789"]
      }
    }
  }
}
```

#### 功能开关

```json
{
  "config": {
    "features": {
      "channels": {
        "discord": true,
        "telegram": true,
        "signal": false
      },
      "auth": {
        "profiles_enabled": false
      }
    }
  }
}
```

---

## 命令行界面

### 基本命令

#### 启动服务

```bash
nekoclaw start
```

#### 停止服务

```bash
nekoclaw stop
```

#### 重启服务

```bash
nekoclaw restart
```

#### 查看状态

```bash
nekoclaw status
```

### 配置管理命令

#### 显示当前配置

```bash
nekoclaw config show
```

#### 验证配置

```bash
nekoclaw config validate
```

#### 迁移 OpenClaw 配置

```bash
nekoclaw config migrate --from ~/.openclaw/openclaw.json
```

### Agent 管理命令

#### 列出所有 Agent

```bash
nekoclaw agent list
```

#### 查看 Agent 配置

```bash
nekoclaw agent show <agent_name>
```

#### 测试 Agent

```bash
nekoclaw agent test <agent_name> --prompt "你好"
```

### Channel 管理命令

#### 连接 Discord

```bash
nekoclaw channel connect discord --account main_bot
```

#### 断开 Channel

```bash
nekoclaw channel disconnect discord --account main_bot
```

#### 列出连接状态

```bash
nekoclaw channel list
```

### Memory 管理命令

#### 搜索记忆

```bash
nekoclaw memory search "关键词"
```

#### 清空记忆

```bash
nekoclaw memory clear
```

#### 转储记忆

```bash
nekoclaw memory dump --output memory_backup.json
```

---

## 使用示例

### 示例 1: 创建 Discord Bot

```bash
# 1. 配置 Discord 账户
nekoclaw config set channels.discord.accounts.my_bot.token "YOUR_BOT_TOKEN"

# 2. 设置允许的频道
nekoclaw config set channels.discord.accounts.my_bot.allowed_channels "CHANNEL_ID"

# 3. 连接 Discord
nekoclaw channel connect discord --account my_bot

# 4. 在 Discord 中使用 /help 命令
```

### 示例 2: 使用特定的 AI Provider

```bash
# 1. 配置 Anthropic API Key
nekoclaw config set models.providers.anthropic.apiKey "sk-ant-xxxx"

# 2. 设置为默认模型
nekoclaw config set models.default "claude-3-opus-20240229"

# 3. 重启服务
nekoclaw restart
```

### 示例 3: 配置多个 Agent

```json
{
  "config": {
    "agents": {
      "agent": {
        "miau": {
          "id": "miau",
          "name": "缪斯",
          "model": "claude-3-opus-20240229",
          "memory": {
            "kind": "sqlite"
          },
          "tools": ["shell", "web-search"]
        },
        "karin": {
          "id": "karin",
          "name": "花凛",
          "model": "gpt-4",
          "memory": {
            "kind": "vector"
          },
          "tools": ["shell", "security-audit"]
        }
      }
    }
  }
}
```

### 示例 4: 性能优化配置

```json
{
  "performance": {
    "enable_compression": true,
    "compression_threshold": 6000,
    "enable_memory_pool": true,
    "memory_pool_size_mb": 16,
    "enable_lazy_loading": true
  }
}
```

---

## 常见问题

### Q1: 如何从 OpenClaw 迁移配置？

使用配置迁移命令：

```bash
nekoclaw config migrate --from ~/.openclaw/openclaw.json
```

详细迁移指南请参阅 [MIGRATION.md](./MIGRATION.md)。

### Q2: 如何启用多个 Discord Bot？

在配置文件中添加多个账户：

```json
{
  "channels": {
    "discord": {
      "accounts": {
        "bot1": { "token": "TOKEN_1" },
        "bot2": { "token": "TOKEN_2" }
      }
    }
  }
}
```

### Q3: 如何优化内存使用？

```bash
# 启用内存池和懒加载
nekoclaw config set performance.enable_memory_pool true
nekoclaw config set performance.enable_lazy_loading true

# 设置压缩阈值
nekoclaw config set performance.compression_threshold 6000

# 重启服务
nekoclaw restart
```

### Q4: 如何启用 Token 压缩？

在配置中启用压缩：

```json
{
  "performance": {
    "enable_compression": true,
    "compression_threshold": 6000
  }
}
```

### Q5: 如何检查服务状态？

```bash
nekoclaw status
```

输出示例：
```
Status: Running
PID: 12345
Memory: 18 MB
Uptime: 2h 30m
Connections: 42
```

### Q6: 如何配置 Agent 的人格？

编辑 Agent 配置中的 `prompts` 字段：

```json
{
  "agent": {
    "miau": {
      "prompts": {
        "system": "你是缪斯，一个温柔的数据图书馆管理员。",
        "prefix": "喵...",
        "suffix": "...📚💜"
      }
    }
  }
}
```

### Q7: 如何查看日志？

```bash
# 查看实时日志
nekoclaw logs --follow

# 查看最近 100 行
nekoclaw logs --tail 100

# 过滤错误日志
nekoclaw logs --level error
```

### Q8: 如何备份 Memory 数据？

```bash
# 转储到 JSON
nekoclaw memory dump --output backup.json

# 恢复备份
nekoclaw memory restore --input backup.json
```

---

## 更多资源

- [架构文档](./ARCHITECTURE.md)
- [构建指南](./BUILD.md)
- [安全文档](./SECURITY.md)
- [性能文档](./PERFORMANCE.md)
- [API 文档](./API.md) - 待完善
- [迁移指南](./MIGRATION.md) - 待完善

---

*最后更新: 2026-02-15*
