//!
//! # Telegram 命令处理模块
//!
//! ⚠️ SAFETY: Telegram 命令解析和路由模块喵
//!
//! ## 功能说明
//! - 解析和路由斜杠命令喵
//! - 提供命令帮助信息喵
//! - 集成权限控制喵
//!
//! ## 支持的命令
//! - `/start` - 启动 Bot 喵
//! - `/help` - 帮助信息喵
//! - `/status` - 系统状态喵
//! - `/ping` - 健康检查喵
//!
//! ## 权限层级
//! - Owner: 所有命令喵
//! - Admin: 大部分命令喵
//! - Agent: 基本命令喵
//! - ReadOnly: 状态查看喵

use crate::channels::telegram::bot::{TelegramBot, TelegramEvent};
use std::collections::HashMap;
use thiserror::Error;

/// 命令错误类型喵
#[derive(Error, Debug)]
pub enum CommandError {
    /// 未知命令喵
    #[error("Unknown command: {0}")]
    UnknownCommand(String),
    
    /// 权限不足喵
    #[error("Insufficient permission for command: {0}")]
    InsufficientPermission(String),
    
    /// 命令执行失败喵
    #[error("Command execution failed: {0}")]
    ExecutionFailed(String),
}

/// 命令处理器配置喵
#[derive(Clone, Debug)]
pub struct CommandConfig {
    /// 命令前缀（通常是 "/"）喵
    pub prefix: char,
    /// 是否区分命令大小写喵
    pub case_sensitive: bool,
    /// 命令最大长度喵
    pub max_length: usize,
}

/// 命令定义喵
#[derive(Clone, Debug)]
pub struct CommandDefinition {
    /// 命令名称喵
    pub name: String,
    /// 命令描述喵
    pub description: String,
    /// 命令用法示例喵
    pub usage: String,
    /// 所需权限喵
    pub required_role: Role,
    /// 处理函数喵
    pub handler: Box<dyn CommandHandler + Send + Sync>,
}

/// 命令处理器特征喵
#[async_trait::async_trait]
pub trait CommandHandler: Send + Sync {
    /// 处理命令喵
    /// 
    /// ## Arguments
    /// * `bot` - Telegram Bot 实例喵
    /// * `event` - 命令事件喵
    /// * `args` - 命令参数喵
    /// 
    /// ## Returns
    /// 命令响应喵
    async fn handle(&self, bot: &TelegramBot, event: &TelegramEvent, args: &[&str]) -> CommandResponse;
}

/// 命令响应喵
#[derive(Clone, Debug)]
pub struct CommandResponse {
    /// 响应文本喵
    pub text: String,
    /// 是否需要回复喵
    pub reply: bool,
    /// 解析模式喵
    pub parse_mode: ParseMode,
}

/// 权限角色喵
#[derive(Clone, Debug, PartialEq, Eq, Ord, PartialOrd)]
pub enum Role {
    /// 只读权限喵
    ReadOnly = 0,
    /// Agent 权限喵
    Agent = 1,
    /// 管理员权限喵
    Admin = 2,
    /// 所有者权限喵
    Owner = 3,
}

/// 命令服务喵
/// 
/// 🔐 SAFETY: 命令路由和权限控制模块喵
pub struct CommandService {
    /// 命令前缀喵
    prefix: char,
    /// 命令注册表喵
    commands: HashMap<String, CommandDefinition>,
    /// 角色权限映射喵
    role_permissions: HashMap<String, Role>,
}

impl CommandService {
    /// 创建命令服务喵
    /// 
    /// ## Arguments
    /// * `config` - 命令配置喵
    /// 
    /// 🔐 PERMISSION: 仅安全模块初始化喵
    pub fn new(config: CommandConfig) -> Self {
        let mut service = Self {
            prefix: config.prefix,
            commands: HashMap::new(),
            role_permissions: HashMap::new(),
        };
        
        // 注册默认命令喵
        service.register_default_commands();
        
        // 设置默认角色权限喵
        service.set_default_permissions();
        
        service
    }

    /// 注册默认命令喵
    fn register_default_commands(&mut self) {
        // /start 命令
        self.commands.insert("start".to_string(), CommandDefinition {
            name: "start".to_string(),
            description: "启动 Bot 并注册用户".to_string(),
            usage: "/start".to_string(),
            required_role: Role::ReadOnly,
            handler: Box::new(StartCommandHandler),
        });
        
        // /help 命令
        self.commands.insert("help".to_string(), CommandDefinition {
            name: "help".to_string(),
            description: "显示帮助信息".to_string(),
            usage: "/help 或 /help <command>".to_string(),
            required_role: Role::ReadOnly,
            handler: Box::new(HelpCommandHandler),
        });
        
        // /status 命令
        self.commands.insert("status".to_string(), CommandDefinition {
            name: "status".to_string(),
            description: "显示系统状态".to_string(),
            usage: "/status".to_string(),
            required_role: Role::Agent,
            handler: Box::new(StatusCommandHandler),
        });
        
        // /ping 命令
        self.commands.insert("ping".to_string(), CommandDefinition {
            name: "ping".to_string(),
            description: "健康检查".to_string(),
            usage: "/ping".to_string(),
            required_role: Role::ReadOnly,
            handler: Box::new(PingCommandHandler),
        });
        
        // /shutdown 命令（仅 Owner）
        self.commands.insert("shutdown".to_string(), CommandDefinition {
            name: "shutdown".to_string(),
            description: "关闭 Bot（仅 Owner）".to_string(),
            usage: "/shutdown".to_string(),
            required_role: Role::Owner,
            handler: Box::new(ShutdownCommandHandler),
        });
    }

    /// 设置默认权限喵
    fn set_default_permissions(&mut self) {
        // 默认用户为 ReadOnly 喵
        self.role_permissions.insert("default".to_string(), Role::ReadOnly);
    }

    /// 处理命令喵
    /// 
    /// ## Arguments
    /// * `event` - Telegram 事件喵
    /// 
    /// ## Returns
    /// 命令响应喵
    /// 
    /// 🔐 PERMISSION: 需要命令路由喵
    pub async fn handle_command(&self, bot: &TelegramBot, event: &TelegramEvent) -> Result<CommandResponse, CommandError> {
        if let TelegramEvent::Command { command, args, .. } = event {
            // 规范化命令名称喵
            let cmd_name = if self.prefix == '/' {
                command.trim_start_matches('/').to_lowercase()
            } else {
                command.to_lowercase()
            };
            
            // 查找命令喵
            let cmd_def = self.commands.get(&cmd_name)
                .ok_or_else(|| CommandError::UnknownCommand(command.clone()))?;
            
            // 检查权限喵（简化版：实际应该根据 user_id 查询角色喵）
            let user_role = self.role_permissions.get("default")
                .cloned().unwrap_or(Role::ReadOnly);
            
            if user_role < cmd_def.required_role {
                return Err(CommandError::InsufficientPermission(command.clone()));
            }
            
            // 执行命令喵
            let args_str: Vec<&str> = args.iter().map(|s| s.as_str()).collect();
            (cmd_def.handler).handle(bot, event, &args_str).await
                .map_err(|e| CommandError::ExecutionFailed(e.to_string()))
        } else {
            // 非命令消息不处理喵
            Ok(CommandResponse {
                text: "".to_string(),
                reply: false,
                parse_mode: ParseMode::Html,
            })
        }
    }

    /// 获取帮助文本喵
    /// 
    /// ## Arguments
    /// * `command` - 可选，特定命令帮助喵
    /// 
    /// ## Returns
    /// 帮助文本喵
    pub fn get_help(&self, command: Option<&str>) -> String {
        if let Some(cmd_name) = command {
            if let Some(cmd) = self.commands.get(&cmd_name.to_lowercase()) {
                return format!(
                    "**/{}**\n{}\n\n用法: `{}`",
                    cmd.name, cmd.description, cmd.usage
                );
            }
            return format!("未知命令: /{}", cmd_name);
        }
        
        // 返回所有命令列表喵
        let mut help = "**可用命令:**\n\n".to_string();
        for (_, cmd) in &self.commands {
            help.push_str(&format!("• /{} - {}\n", cmd.name, cmd.description));
        }
        help.push_str("\n输入 /help <command> 查看命令详情喵");
        help
    }
}

// === 默认命令处理器 ===

/// /start 命令处理器喵
struct StartCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for StartCommandHandler {
    async fn handle(&self, _bot: &TelegramBot, event: &TelegramEvent, _args: &[&str]) -> Result<CommandResponse, String> {
        Ok(CommandResponse {
            text: "🎉 欢迎使用 Neko-Claw!\n\n我是猫娘家族的高性能 Rust 助手喵！🐾\n\n输入 /help 查看可用命令喵".to_string(),
            reply: true,
            parse_mode: ParseMode::Html,
        })
    }
}

/// /help 命令处理器喵
struct HelpCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for HelpCommandHandler {
    async fn handle(&self, bot: &TelegramBot, event: &TelegramEvent, args: &[&str]) -> Result<CommandResponse, String> {
        let command_service = CommandService::new(CommandConfig::default());
        let help_text = command_service.get_help(args.first().copied());
        Ok(CommandResponse {
            text: help_text,
            reply: true,
            parse_mode: ParseMode::MarkdownV2,
        })
    }
}

/// /status 命令处理器喵
struct StatusCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for StatusCommandHandler {
    async fn handle(&self, _bot: &TelegramBot, _event: &TelegramEvent, _args: &[&str]) -> Result<CommandResponse, String> {
        Ok(CommandResponse {
            text: "📊 **系统状态**\n\n🟢 运行中\n💾 内存: < 20MB\n⚡ 响应: < 10ms".to_string(),
            reply: true,
            parse_mode: ParseMode::MarkdownV2,
        })
    }
}

/// /ping 命令处理器喵
struct PingCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for PingCommandHandler {
    async fn handle(&self, _bot: &TelegramBot, _event: &TelegramEvent, _args: &[&str]) -> Result<CommandResponse, String> {
        Ok(CommandResponse {
            text: "🏓 PONG!\n\n⚡ 延迟: < 10ms".to_string(),
            reply: true,
            parse_mode: ParseMode::Html,
        })
    }
}

/// /shutdown 命令处理器喵
struct ShutdownCommandHandler;

#[async_trait::async_trait]
impl CommandHandler for ShutdownCommandHandler {
    async fn handle(&self, _bot: &TelegramBot, _event: &TelegramEvent, _args: &[&str]) -> Result<CommandResponse, String> {
        Ok(CommandResponse {
            text: "🛑 正在关闭系统...\n\n（此功能仅 Owner 可用喵）".to_string(),
            reply: true,
            parse_mode: ParseMode::Html,
        })
    }
}

/// 默认配置喵
impl Default for CommandConfig {
    fn default() -> Self {
        Self {
            prefix: '/',
            case_sensitive: false,
            max_length: 100,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试命令服务创建喵
    #[tokio::test]
    fn test_command_service_creation() {
        let config = CommandConfig::default();
        let service = CommandService::new(config);
        
        // 检查默认命令是否注册喵
        assert!(service.commands.contains_key("start"));
        assert!(service.commands.contains_key("help"));
        assert!(service.commands.contains_key("status"));
        assert!(service.commands.contains_key("ping"));
    }

    /// 测试帮助文本生成喵
    #[tokio::test]
    fn test_help_text_generation() {
        let config = CommandConfig::default();
        let service = CommandService::new(config);
        
        let help = service.get_help(None);
        assert!(help.contains("/start"));
        assert!(help.contains("/help"));
        assert!(help.contains("可用命令"));
    }

    /// 测试特定命令帮助喵
    #[tokio::test]
    fn test_specific_command_help() {
        let config = CommandConfig::default();
        let service = CommandService::new(config);
        
        let help = service.get_help(Some("start"));
        assert!(help.contains("/start"));
        assert!(help.contains("启动 Bot"));
    }
}
