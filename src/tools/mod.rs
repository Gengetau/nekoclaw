/// Tools 模块导出 🔧
///
/// @诺诺 的 Tools 模块统一入口喵
///
/// 功能：
/// - Shell 命令执行工具（安全保护）
/// - Agent Family 协议通信工具
/// - 工具链管理系统
///
/// 🔒 SAFETY: 所有 Tool 都经过安全沙箱保护
///
/// 模块作者: 诺诺 (Nono) ⚡

pub mod shell;
pub mod brain;

// 🔒 SAFETY: 重新导出公共接口喵
pub use shell::{ShellTool, ShellRequest, ShellResult, ShellError};
pub use brain::{BrainTool, AgentMessage, MessageKind, AgentInfo, SubAgentConfig, BrainError};

// 🔒 SAFETY: 为了兼容性，定义类型别名
pub type ToolChain = ToolsManager;

/// 🔒 SAFETY: 工具链管理器结构体喵
/// 统一管理所有可用工具
#[derive(Debug, Clone)]
pub struct ToolsManager {
    /// Shell 工具
    shell: Option<ShellTool>,
    /// Brain 工具
    brain: Option<BrainTool>,
}

impl ToolsManager {
    /// 🔒 SAFETY: 创建新的工具链管理器喵
    pub fn new() -> Self {
        Self {
            shell: None,
            brain: None,
        }
    }

    /// 🔒 SAFETY: 添加 Shell 工具喵
    pub fn with_shell(mut self, tool: ShellTool) -> Self {
        self.shell = Some(tool);
        self
    }

    /// 🔒 SAFETY: 添加 Brain 工具喵
    pub fn with_brain(mut self, tool: BrainTool) -> Self {
        self.brain = Some(tool);
        self
    }

    /// 🔒 SAFETY: 获取 Shell 工具喵
    pub fn shell(&self) -> Result<&ShellTool, String> {
        self.shell
            .as_ref()
            .ok_or_else(|| "Shell tool not initialized".to_string())
    }

    /// 🔒 SAFETY: 获取 Brain 工具喵
    pub fn brain(&self) -> Result<&BrainTool, String> {
        self.brain
            .as_ref()
            .ok_or_else(|| "Brain tool not initialized".to_string())
    }
}

impl Default for ToolsManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 🔒 SAFETY: 测试辅助函数喵
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tools_manager_creation() {
        let manager = ToolsManager::new();
        assert!(manager.shell().is_err());
        assert!(manager.brain().is_err());
    }
}
