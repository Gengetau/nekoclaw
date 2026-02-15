/*!
 * Core Traits - 可插拔的抽象层
 *
 * 作者: 缪斯 (Muse) @缪斯
 * 日期: 2026-02-15 17:08 JST
 */

use serde::{Deserialize, Serialize};
use serde_json::Value;
use chrono::{DateTime, Utc};
use std::pin::Pin;
use std::error::Error;
use std::fmt;
use futures::Stream;

pub type Result<T> = std::result::Result<T, Box<dyn Error + Send + Sync>>;

// ============================================================================
// Message Types
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub role: String, // "user", "assistant", "system"
    pub content: String,
}

// ============================================================================
// Provider Trait (AI Model Adapter)
// ============================================================================

#[async_trait::async_trait]
pub trait Provider: Send + Sync {
    async fn chat(&self, messages: &[Message]) -> Result<String>;
    async fn stream(&self, messages: &[Message]) -> Pin<Box<dyn Stream<Item = Result<String>> + Send>>;
    fn name(&self) -> &str;
    fn supports_streaming(&self) -> bool;
}

// ============================================================================
// Channel Trait (Messaging Platform Adapter)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelEvent {
    pub source: String,    // "discord", "telegram", etc.
    pub sender_id: String,  // 用户/频道 ID
    pub message: String,
    pub metadata: Option<Value>,
}

#[async_trait::async_trait]
pub trait Channel: Send + Sync {
    async fn send(&self, content: &str, target: Option<&str>) -> Result<()>;
    async fn receive(&self) -> Pin<Box<dyn Stream<Item = Result<ChannelEvent>> + Send>>;
    fn name(&self) -> &str;
    fn channel_type(&self) -> &str;
}

// ============================================================================
// Memory Trait (Memory System)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryItem {
    pub id: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: Option<Value>,
    pub created_at: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait Memory: Send + Sync {
    async fn recall(&self, query: &str, top_k: usize) -> Result<Vec<MemoryItem>>;
    async fn save(&self, item: MemoryItem) -> Result<String>; // 返回 ID
    async fn forget(&self, id: &str) -> Result<()>;
    async fn search(&self, query: &str) -> Result<Vec<MemoryItem>>;
}

// ============================================================================
// Tool Trait (Capability Extension)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInput {
    pub name: String,
    pub args: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolOutput {
    pub success: bool,
    pub result: Value,
    pub error: Option<String>,
}

#[async_trait::async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    async fn execute(&self, args: Value) -> Result<ToolOutput>;
    fn is_dangerous(&self) -> bool; // 用于沙箱检查
}

// ============================================================================
// Observer Trait (Observability)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metric {
    pub name: String,
    pub value: f64,
    pub tags: Vec<(String, String)>,
    pub timestamp: DateTime<Utc>,
}

#[async_trait::async_trait]
pub trait Observer: Send + Sync {
    async fn record(&self, metric: Metric) -> Result<()>;
    async fn flush(&self) -> Result<()>;
}

// ============================================================================
// Tunnel Trait (Network Tunnel)
// ============================================================================

#[async_trait::async_trait]
pub trait Tunnel: Send + Sync {
    async fn start(&self) -> Result<String>; // 返回公共 URL
    async fn stop(&self) -> Result<()>;
    fn public_url(&self) -> Option<&str>;
}

// ============================================================================
// Identity Trait (Persona Engine)
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Identity {
    pub name: String,
    pub personality: Personality,
    pub speech_patterns: SpeechPatterns,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Personality {
    pub tone: String,
    pub emoji: String,
    pub catchphrases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpeechPatterns {
    pub prefix: Vec<String>,
    pub suffix: Vec<String>,
    pub prohibited: Vec<String>,
}

pub trait IdentityEngine: Send + Sync {
    fn load(&mut self, path: &std::path::Path) -> Result<Identity>;
    fn inject(&self, response: &str) -> String;
}

// ============================================================================
// Config Structure
// ============================================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: Option<String>,
    pub default_provider: String,
    pub default_model: String,
    pub default_temperature: f64,
    pub workspace: std::path::PathBuf,
}

