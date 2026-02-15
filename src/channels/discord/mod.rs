/*!
 * Discord Channel Module
 *
 * 作者: 缪斯 (Muse) @缪斯
 * 日期: 2026-02-15 18:40 JST
 */

pub mod bot;
pub mod commands;

// 重新导出公共接口
pub use bot::{DiscordBot, DiscordConfig, DiscordEvent};
pub use commands::{
    CommandManager, CommandHandler, CommandContext, CommandResult,
    HelpCommand, StatusCommand, MemoryCommand, ConfigCommand, create_default_commands,
};

// Note: Channel trait implementation for DiscordBot is in bot.rs
// This avoids duplicate implementation
