/*!
 * Configuration Management - 配置管理模块
 *
 * 作者: 诺诺 (Nono) @诺诺
 * 日期: 2026-02-16 JST
 *
 * 功能：
 * - 加载 JSON/TOML 配置文件
 * - 默认配置生成
 * - 环境变量覆盖
 */

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use anyhow::{Result, Context};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NekoClawConfig {
    pub version: String,
    pub gateway: GatewayConfig,
    pub models: ModelsConfig,
    pub channels: ChannelsConfig,
    pub agents: AgentsConfig,
    pub bindings: Vec<BindingConfig>,
    pub commands: CommandsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub host: String,
    pub port: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelsConfig {
    pub providers: std::collections::HashMap<String, ProviderConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProviderConfig {
    #[serde(default)]
    pub base_url: Option<String>,
    pub api_key: Option<String>,
    #[serde(default)]
    pub api: Option<String>,
    #[serde(default)]
    pub models: Vec<ModelConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelConfig {
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub reasoning: bool,
    #[serde(default)]
    pub input: Vec<String>,
    #[serde(default)]
    pub cost: CostConfig,
    #[serde(default)]
    pub context_window: usize,
    #[serde(default)]
    pub max_tokens: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CostConfig {
    #[serde(default)]
    pub input: f64,
    #[serde(default)]
    pub output: f64,
    #[serde(default)]
    pub cache_read: f64,
    #[serde(default)]
    pub cache_write: f64,
}

impl Default for CostConfig {
    fn default() -> Self {
        Self {
            input: 0.0,
            output: 0.0,
            cache_read: 0.0,
            cache_write: 0.0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelsConfig {
    pub discord: Option<DiscordChannelConfig>,
    pub telegram: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordChannelConfig {
    #[serde(default)]
    pub enabled: bool,
    pub accounts: std::collections::HashMap<String, DiscordAccountConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscordAccountConfig {
    pub token: String,
    #[serde(default)]
    pub group_policy: String,
    #[serde(default)]
    pub guilds: std::collections::HashMap<String, GuildConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuildConfig {
    #[serde(default)]
    pub require_mention: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentsConfig {
    pub defaults: AgentDefaultsConfig,
    #[serde(default)]
    pub list: Vec<AgentConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentDefaultsConfig {
    pub model: ModelRefConfig,
    #[serde(default)]
    pub workspace: Option<String>,
    #[serde(default)]
    pub thinking_default: Option<String>,
    #[serde(default)]
    pub max_concurrent: Option<usize>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelRefConfig {
    pub primary: String,
    #[serde(default)]
    pub fallbacks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentConfig {
    pub id: String,
    #[serde(default)]
    pub default: bool,
    pub name: String,
    pub model: String,
    #[serde(default)]
    pub workspace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingConfig {
    pub agent_id: String,
    pub match: BindingMatchConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingMatchConfig {
    pub channel: String,
    pub account_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandsConfig {
    #[serde(default)]
    pub native: Option<String>,
    #[serde(default)]
    pub native_skills: Option<String>,
    #[serde(default)]
    pub owner_allow_from: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_provider: Option<String>,
    pub default_model: Option<String>,
    pub default_temperature: f64,
    pub workspace: PathBuf,

    // Discord 配置
    #[serde(rename = "discord")]
    pub discord_config: Option<DiscordConfig>,

    // NVIDIA 配置
    #[serde(rename = "nvidia")]
    pub nvidia_config: Option<NvidiaConfig>,

    // Gateway 配置
    pub gateway_port: Option<u16>,
    pub gateway_bind: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename = "DiscordConfig")]
pub struct DiscordConfig {
    #[serde(default)]
    pub enabled: bool,
    pub token: Option<String>,
    #[serde(default)]
    pub allowed_users: Vec<String>,
    #[serde(default)]
    pub require_mention: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NvidiaConfig {
    #[serde(default)]
    pub base_url: String,
    pub api_key: String,
    #[serde(default)]
    pub api: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: None,
            default_provider: Some("nvidia".to_string()),
            default_model: Some("deepseek-ai/deepseek-v3.2".to_string()),
            default_temperature: 0.7,
            workspace: dirs::home_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join(".nekoclaw/workspace"),
            discord_config: None,
            nvidia_config: None,
            gateway_port: Some(8080),
            gateway_bind: Some("127.0.0.1".to_string()),
        }
    }
}

pub async fn load_config(path: &PathBuf) -> Config {
    if !path.exists() {
        return Config::default();
    }

    let content = match std::fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Failed to read config: {}", e);
            return Config::default();
        }
    };

    if path.extension().map_or(false, |e| e == "json") {
        match serde_json::from_str::<Config>(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to parse JSON config: {}", e);
                Config::default()
            }
        }
    } else if path.extension().map_or(false, |e| e == "toml") {
        match toml::from_str::<Config>(&content) {
            Ok(config) => config,
            Err(e) => {
                eprintln!("Failed to parse TOML config: {}", e);
                Config::default()
            }
        }
    } else {
        eprintln!("Unsupported config format");
        Config::default()
    }
}
