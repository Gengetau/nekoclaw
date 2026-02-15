/// Gateway 模块导出 🌐
///
/// @诺诺 的 Gateway 模块统一入口喵
///
/// 功能：
/// - 导出所有 Gateway 实现
/// - 统一错误处理
/// - Gateway 工厂函数
///
/// 🔒 SAFETY: 模块级访问控制，防止非法访问
///
/// 模块作者: 诺诺 (Nono) ⚡

pub mod server;
pub mod pairing;
pub mod webhook;

// 🔒 SAFETY: 重新导出公共接口喵
pub use server::{GatewayConfig, GatewayServer, GatewayState, HealthResponse, ErrorResponse};
pub use pairing::{PairingConfig, PairingManager, PairingRequest, PairingResponse, PairingStatus};
pub use webhook::{WebhookConfig, WebhookManager, WebhookEvent, WebhookResponse, WebhookEventType, WebhookHandler};

/// 🔒 SAFETY: Gateway 统一入口结构体喵
/// 封装所有 Gateway 功能
#[derive(Debug, Clone)]
pub struct Gateway {
    /// HTTP 服务器
    server: Option<GatewayServer>,
    /// 配对管理器
    pairing_manager: PairingManager,
    /// Webhook 管理器
    webhook_manager: WebhookManager,
}

impl Gateway {
    /// 🔒 SAFETY: 创建新的 Gateway 实例喵
    pub fn new(gateway_config: GatewayConfig) -> Self {
        let pairing_config = PairingConfig::default();
        let webhook_config = WebhookConfig::default();

        Self {
            server: Some(GatewayServer::new(gateway_config)),
            pairing_manager: PairingManager::new(pairing_config),
            webhook_manager: WebhookManager::new(webhook_config),
        }
    }

    /// 🔒 SAFETY: 启动 Gateway 服务器喵
    /// 异常处理: 启动失败时返回错误
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(server) = self.server {
            server.run().await?;
        }

        Ok(())
    }

    /// 🔒 SAFETY: 获取配对管理器喵
    pub fn pairing_manager(&self) -> &PairingManager {
        &self.pairing_manager
    }

    /// 🔒 SAFETY: 获取 Webhook 管理器喵
    pub fn webhook_manager(&self) -> &WebhookManager {
        &self.webhook_manager
    }
}

/// 🔒 SAFETY: 测试辅助函数喵
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gateway_creation() {
        let config = GatewayConfig::default();
        let gateway = Gateway::new(config);

        assert!(gateway.server.is_some());
    }
}
