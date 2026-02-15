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

use crate::core::traits::*;

#[async_trait::async_trait]
impl Channel for DiscordBot {
    async fn send(&self, content: &str, target: Option<&str>) -> Result<()> {
        let channel_id = target.ok_or("Target channel ID required")?;
        self.send_message(channel_id, content).await
    }

    async fn receive(&self) -> Pin<Box<dyn futures::Stream<Item = Result<ChannelEvent>> + Send>> {
        // 简化实现：返回空流
        // TODO: 实现完整的 WebSocket 消息接收
        use futures::stream;
        
        let event = ChannelEvent {
            source: "discord".to_string(),
            sender_id: "system".to_string(),
            message: "Mock event: Discord connection pending".to_string(),
            metadata: None,
        };
        
        Box::pin(stream::once(async { Ok(event) }))
    }

    fn name(&self) -> &str {
        "discord"
    }

    fn channel_type(&self) -> &str {
        "discord"
    }
}
