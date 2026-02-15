/*!
 * 配置加载模块
 *
 * 作者: 缪斯 (Muse) @缪斯
 * 日期: 2026-02-15 17:40 JST
 */

use crate::core::traits::{Config, Result};
use std::path::Path;

impl Default for Config {
    fn default() -> Self {
        Config {
            api_key: None,
            default_provider: "openai".to_string(),
            default_model: "gpt-4".to_string(),
            default_temperature: 0.7,
            workspace: dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("/home/ubuntu"))
                .join(".nekoclaw/workspace"),
        }
    }
}

pub fn load(config_dir: &Path) -> Result<Config> {
    let config_path = config_dir.join("config.toml");

    // 如果配置文件不存在，使用默认配置
    if !config_path.exists() {
        return Ok(Config::default());
    }

    let content = std::fs::read_to_string(&config_path)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    let config: Config = toml::from_str(&content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    Ok(config)
}

pub fn save(config_dir: &Path, config: &Config) -> Result<()> {
    let config_path = config_dir.join("config.toml");
    std::fs::create_dir_all(config_dir)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    let content = toml::to_string_pretty(config)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    std::fs::write(&config_path, content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;

    Ok(())
}
