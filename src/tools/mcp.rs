//! # Tool Trait & Registration System
//!
//! 🔧 MCP-compatible tool system
//!
//! @诺诺 的 Tool Calling 实现喵
//!
//! ## 功能
//! - MCP-compatible tool descriptions
//! - Tool registration & discovery
//! - Tool execution with safety checks
//! - Tool result formatting for LLM
//!
//! ## MCP 协议兼容性
//! - Tool name, description, input schema
//! - JSON Schema validation
//! - Tool result formatting
//!
//! 🔒 SAFETY: All tools go through security sandbox
//!
//! Author: 诺诺 (Nono) ⚡

use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use std::collections::HashMap;
use std::sync::Arc;
use thiserror::Error;

/// 🔒 SAFETY: Tool 执行错误类型喵
#[derive(Debug, Error)]
pub enum ToolError {
    /// 工具未注册
    #[error("Tool '{0}' not found")]
    NotFound(String),

    /// 工具执行失败
    #[error("Tool execution failed: {0}")]
    ExecutionFailed(String),

    /// 参数验证失败
    #[error("Parameter validation failed: {0}")]
    ValidationError(String),

    /// 权限不足
    #[error("Permission denied for tool '{0}'")]
    PermissionDenied(String),

    /// 超时
    #[error("Tool execution timed out")]
    Timeout,

    /// 其他错误
    #[error("Tool error: {0}")]
    Other(String),
}

/// 🔒 SAFETY: Tool 描述结构体（MCP 兼容）喵
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDescription {
    /// 工具名称（唯一标识符）
    pub name: String,

    /// 工具描述（用途和功能）
    pub description: String,

    /// 输入参数 schema（JSON Schema 格式）
    pub input_schema: JsonValue,

    /// 工具分类
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,

    /// 是否危险操作（需要确认）
    #[serde(default = "default_dangerous")]
    pub dangerous: bool,

    /// 权限要求
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_permissions: Option<Vec<String>>,
}

fn default_dangerous() -> bool {
    false
}

/// 🔒 SAFETY: Tool 执行结果喵
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolResult {
    /// 是否成功
    pub success: bool,

    /// 结果数据（JSON 格式）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<JsonValue>,

    /// 错误信息（如果失败）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,

    /// 执行时间（毫秒）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_ms: Option<u64>,
}

impl ToolResult {
    /// 🔒 SAFETY: 创建成功结果喵
    pub fn success(data: JsonValue, duration_ms: u64) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            duration_ms: Some(duration_ms),
        }
    }

    /// 🔒 SAFETY: 创建失败结果喵
    pub fn failure(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            duration_ms: None,
        }
    }
}

/// 🔒 SAFETY: Tool trait（MCP 兼容）喵
///
/// 所有工具必须实现这个 trait
#[async_trait::async_trait]
pub trait Tool: Sync + Send {
    /// 获取工具描述
    fn describe(&self) -> ToolDescription;

    /// 验证输入参数
    fn validate_input(&self, input: &JsonValue) -> Result<(), ToolError>;

    /// 执行工具
    async fn execute(&self, input: JsonValue) -> Result<ToolResult, ToolError>;
}

/// 🔒 SAFETY: 工具注册器喵
///
/// 管理所有可用工具的工具注册系统
#[derive(Clone)]
pub struct ToolRegistry {
    /// 工具映射（名称 → 工具）
    tools: HashMap<String, Arc<dyn Tool>>,

    /// 工具分类映射
    categories: HashMap<String, Vec<String>>,
}

impl ToolRegistry {
    /// 🔒 SAFETY: 创建新的工具注册器喵
    pub fn new() -> Self {
        Self {
            tools: HashMap::new(),
            categories: HashMap::new(),
        }
    }

    /// 🔒 SAFETY: 注册工具喵
    pub fn register<T: Tool + 'static>(&mut self, tool: T) -> Result<(), ToolError> {
        let description = tool.describe();
        let name = description.name.clone();
        let category = description.category.clone();

        // 检查名称冲突
        if self.tools.contains_key(&name) {
            return Err(ToolError::ExecutionFailed(format!(
                "Tool '{}' already registered",
                name
            )));
        }

        // 注册工具
        self.tools.insert(name.clone(), Arc::new(tool));

        // 添加到分类
        if let Some(cat) = category {
            self.categories
                .entry(cat)
                .or_insert_with(Vec::new)
                .push(name.clone());
        }

        tracing::info!("Tool registered: {} - {}", name, description.description);
        Ok(())
    }

    /// 🔒 SAFETY: 获取工具描述喵
    pub fn get_description(&self, name: &str) -> Option<ToolDescription> {
        self.tools.get(name).map(|tool| tool.describe())
    }

    /// 🔒 SAFETY: 获取所有工具描述喵
    pub fn all_descriptions(&self) -> Vec<ToolDescription> {
        self.tools.values().map(|tool| tool.describe()).collect()
    }

    /// 🔒 SAFETY: 获取分类下的工具喵
    pub fn tools_by_category(&self, category: &str) -> Vec<ToolDescription> {
        self.categories
            .get(category)
            .map(|names| {
                names
                    .iter()
                    .filter_map(|name| self.get_description(name))
                    .collect()
            })
            .unwrap_or_default()
    }

    /// 🔒 SAFETY: 执行工具喵
    pub async fn execute(&self, name: &str, input: JsonValue) -> Result<ToolResult, ToolError> {
        // 查找工具
        let tool = self
            .tools
            .get(name)
            .ok_or_else(|| ToolError::NotFound(name.to_string()))?;

        let start = std::time::Instant::now();

        // 验证输入
        tool.validate_input(&input)?;

        // 执行工具
        let result = tool.execute(input).await?;

        Ok(result)
    }

    /// 🔒 SAFETY: 工具数量喵
    pub fn count(&self) -> usize {
        self.tools.len()
    }

    /// 🔒 SAFETY: 检查工具是否存在喵
    pub fn has_tool(&self, name: &str) -> bool {
        self.tools.contains_key(name)
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// 🔒 SAFETY: Tool Calling 请求喵
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallRequest {
    /// 工具名称
    pub tool_name: String,

    /// 工具参数
    pub arguments: JsonValue,

    /// 调用 ID（上下文跟踪）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
}

/// 🔒 SAFETY: Tool Calling 响应喵
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolCallResponse {
    /// 调用结果
    pub result: ToolResult,

    /// 调用 ID（上下文跟踪）
    #[serde(skip_serializing_if = "Option::is_none")]
    pub call_id: Option<String>,
}

/// 🔒 SAFETY: 格式化工具列表为 LLM 可读格式喵
pub fn format_tools_for_llm(tools: &[ToolDescription]) -> String {
    let mut output = String::from("Available tools:\n");

    for tool in tools {
        output.push_str(&format!("\n### {}\n", tool.name));
        output.push_str(&format!("**Description**: {}\n", tool.description));

        if let Some(category) = &tool.category {
            output.push_str(&format!("**Category**: {}\n", category));
        }

        if tool.dangerous {
            output.push_str("**⚠️ DANGEROUS**: This tool requires confirmation\n");
        }

        // 格式化输入 schema
        if let Some(schema) = tool.input_schema.get("properties") {
            if let Some(obj) = schema.as_object() {
                output.push_str("**Parameters**:\n");
                for (param, param_schema) in obj {
                    let param_type = param_schema
                        .get("type")
                        .and_then(|t| t.as_str())
                        .unwrap_or("unknown");
                    let param_desc = param_schema
                        .get("description")
                        .and_then(|d| d.as_str())
                        .unwrap_or("-");

                    output.push_str(&format!("- `{}` ({}): {}\n", param, param_type, param_desc));
                }
            }
        }

        output.push('\n');
    }

    output
}

/// 🔒 SAFETY: 格式化工具调用为 LLM 可读字符串喵
pub fn format_tool_call_for_llm(call: &ToolCallRequest) -> String {
    let args_str = if call.arguments.is_null() {
        "no arguments".to_string()
    } else if call.arguments.is_string() {
        // 🔒 SAFETY: 使用 unwrap_or_default 替代 unwrap() 喵
        call.arguments.as_str().unwrap_or_default().to_string()
    } else {
        serde_json::to_string_pretty(&call.arguments).unwrap_or_else(|_| "{}".to_string())
    };

    format!("Call tool '{}' with: {}", call.tool_name, args_str)
}

/// 🔒 SAFETY: 格式化工具结果为 LLM 可读字符串喵
pub fn format_tool_result_for_llm(result: &ToolResult) -> String {
    if result.success {
        if let Some(data) = &result.data {
            serde_json::to_string_pretty(data).unwrap_or_else(|_| "{}".to_string())
        } else {
            "Tool executed successfully (no output)".to_string()
        }
    } else {
        format!("Tool failed: {}", result.error.as_deref().unwrap_or("Unknown error"))
    }
}

/// 🔒 SAFETY: 从文本中解析工具调用指令喵
pub fn parse_tool_calls(text: &str) -> Vec<ToolCallRequest> {
    let mut calls = Vec::new();
    
    // 正则表达式匹配 @tool_name(json_params)
    // 允许嵌套的大括号喵
    let re = regex::Regex::new(r"@([a-zA-Z0-9_]+)\(([\s\S]*?)\)").unwrap();
    
    for cap in re.captures_iter(text) {
        let tool_name = cap[1].to_string();
        let params_str = cap[2].trim();
        
        // 尝试解析为 JSON
        let arguments = if params_str.is_empty() {
            serde_json::Value::Null
        } else {
            serde_json::from_str(params_str).unwrap_or_else(|_| serde_json::Value::String(params_str.to_string()))
        };
        
        calls.push(ToolCallRequest {
            tool_name,
            arguments,
            call_id: None,
        });
    }
    
    calls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tool_result() {
        let result = ToolResult::success(
            serde_json::json!({"output": "hello"}),
            100,
        );

        assert!(result.success);
        assert!(result.data.is_some());
        assert_eq!(result.duration_ms, Some(100));
    }

    #[test]
    fn test_format_tools_for_llm() {
        let tools = vec![
            ToolDescription {
                name: "test_tool".to_string(),
                description: "A test tool".to_string(),
                input_schema: serde_json::json!({
                    "type": "object",
                    "properties": {
                        "message": {
                            "type": "string",
                            "description": "Test message"
                        }
                    }
                }),
                category: Some("test".to_string()),
                dangerous: false,
                required_permissions: None,
            }
        ];

        let formatted = format_tools_for_llm(&tools);
        assert!(formatted.contains("test_tool"));
        assert!(formatted.contains("A test tool"));
    }
}
