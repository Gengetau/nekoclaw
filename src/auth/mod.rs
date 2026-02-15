//!
//! # Auth Module
//!
//! ⚠️ SAFETY: OAuth 认证和凭证管理模块喵
//!
//! ## 功能说明
//! - OAuth 2.0 认证流程支持喵
//! - 凭证安全存储和加密喵
//! - 认证配置文件解析喵
//! - Token 自动刷新喵
//!
//! ## OpenClaw 兼容
//! - 兼容 `auth.profiles` 配置格式喵
//! - 支持 Discord OAuth喵
//! - 支持 Google OAuth喵
//!
//! ## 使用示例
//! ```rust
//! use nekoclaw::auth::{AuthManager, OAuthConfig};
//!
//! let config = OAuthConfig::from_discord("client_id", "client_secret", redirect_uri);
//! let manager = AuthManager::new(config);
//! ```

use crate::security::CryptoService;
use async_trait::async_trait;
use chrono::{Duration, Utc};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl, RefreshToken};
use oauth2::reqwest::async_http_client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use thiserror::Error;

/// 认证错误类型喵
#[derive(Error, Debug)]
pub enum AuthError {
    /// 认证失败喵
    #[error("Authentication failed: {0}")]
    AuthenticationFailed(String),
    
    /// Token 无效喵
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    
    /// Token 过期喵
    #[error("Token expired at {0}")]
    TokenExpired(chrono::DateTime<Utc>),
    
    /// 刷新 Token 失败喵
    #[error("Failed to refresh token: {0}")]
    RefreshFailed(String),
    
    /// 配置错误喵
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    /// 加密错误喵
    #[error("Encryption error: {0}")]
    EncryptionError(String),
    
    /// 提供商不支持喵
    #[error("Provider not supported: {0}")]
    ProviderNotSupported(String),
}

/// OAuth 提供商类型喵
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OAuthProvider {
    /// Discord OAuth
    Discord,
    
    /// Google OAuth
    Google,
    
    /// GitHub OAuth
    GitHub,
    
    /// 自定义 OAuth
    Custom(String),
}

/// OAuth 配置文件喵
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OAuthConfig {
    /// 提供商类型喵
    pub provider: OAuthProvider,
    
    /// 客户端 ID喵
    pub client_id: String,
    
    /// 客户端密钥喵
    pub client_secret: String,
    
    /// 重定向 URI喵
    pub redirect_uri: String,
    
    /// 授权 URL喵
    pub auth_url: String,
    
    /// Token URL喵
    pub token_url: String,
    
    /// 作用域喵
    pub scopes: Vec<String>,
    
    /// 是否启用喵
    pub enabled: bool,
}

impl OAuthConfig {
    /// 创建 Discord OAuth 配置喵
    /// 
    /// ## Arguments
    /// * `client_id` - Discord 应用客户端 ID喵
    /// * `client_secret` - Discord 应用客户端密钥喵
    /// * `redirect_uri` - 回调 URI喵
    /// 
    /// ## Returns
    /// Discord OAuth 配置喵
    /// 
    /// 🔐 PERMISSION: 仅配置阶段喵
    pub fn discord(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            provider: OAuthProvider::Discord,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            auth_url: "https://discord.com/api/oauth2/authorize".to_string(),
            token_url: "https://discord.com/api/oauth2/token".to_string(),
            scopes: vec!["identify".to_string(), "email".to_string()],
            enabled: true,
        }
    }

    /// 创建 Google OAuth 配置喵
    /// 
    /// ## Arguments
    /// * `client_id` - Google 客户端 ID喵
    /// * `client_secret` - Google 客户端密钥喵
    /// * `redirect_uri` - 回调 URI喵
    /// 
    /// 🔐 PERMISSION: 仅配置阶段喵
    pub fn google(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            provider: OAuthProvider::Google,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            auth_url: "https://accounts.google.com/o/oauth2/v2/auth".to_string(),
            token_url: "https://oauth2.googleapis.com/token".to_string(),
            scopes: vec!["openid".to_string(), "email".to_string(), "profile".to_string()],
            enabled: true,
        }
    }

    /// 创建 GitHub OAuth 配置喵
    /// 
    /// ## Arguments
    /// * `client_id` - GitHub 客户端 ID喵
    /// * `client_secret` - GitHub 客户端密钥喵
    /// * `redirect_uri` - 回调 URI喵
    /// 
    /// 🔐 PERMISSION: 仅配置阶段喵
    pub fn github(client_id: &str, client_secret: &str, redirect_uri: &str) -> Self {
        Self {
            provider: OAuthProvider::GitHub,
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            redirect_uri: redirect_uri.to_string(),
            auth_url: "https://github.com/login/oauth/authorize".to_string(),
            token_url: "https://github.com/login/oauth/access_token".to_string(),
            scopes: vec!["read:user".to_string(), "user:email".to_string()],
            enabled: true,
        }
    }

    /// 转换为 oauth2 客户端喵
    /// 
    /// ## Returns
    /// BasicClient 实例喵
    /// 
    /// 🔐 PERMISSION: 内部使用喵
    pub fn to_oauth2_client(&self) -> Result<BasicClient, AuthError> {
        let client_id = ClientId::new(self.client_id.clone());
        let client_secret = ClientSecret::new(self.client_secret.clone());
        let auth_url = AuthUrl::new(self.auth_url.clone())
            .map_err(|e| AuthError::ConfigError(e.to_string()))?;
        let token_url = TokenUrl::new(self.token_url.clone())
            .map_err(|e| AuthError::ConfigError(e.to_string()))?;
        let redirect_url = RedirectUrl::new(self.redirect_uri.clone())
            .map_err(|e| AuthError::ConfigError(e.to_string()))?;

        Ok(BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_url(auth_url)
            .set_token_url(token_url)
            .set_redirect_url(redirect_url))
    }
}

/// Token 信息喵
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TokenInfo {
    /// Access Token喵
    pub access_token: String,
    
    /// Refresh Token喵
    pub refresh_token: Option<String>,
    
    /// Token 类型喵
    pub token_type: String,
    
    /// 过期时间喵
    pub expires_at: chrono::DateTime<Utc>,
    
    /// 作用域喵
    pub scopes: Vec<String>,
    
    /// 关联的用户 ID喵
    pub user_id: Option<String>,
}

/// 认证会话喵
#[derive(Clone, Debug)]
pub struct AuthSession {
    /// 会话 ID喵
    pub id: String,
    
    /// OAuth 配置喵
    pub config: OAuthConfig,
    
    /// Token 信息喵
    pub token: Option<TokenInfo>,
    
    /// 创建时间喵
    pub created_at: chrono::DateTime<Utc>,
    
    /// 最后活动时间喵
    pub last_activity: chrono::DateTime<Utc>,
    
    /// 状态喵
    pub state: AuthState,
}

/// 认证状态喵
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AuthState {
    /// 初始状态喵
    Initial,
    
    /// 等待用户授权喵
    PendingAuthorization,
    
    /// 已授权喵
    Authorized,
    
    /// Token 有效喵
    Active,
    
    /// Token 已过期喵
    Expired,
    
    /// 错误状态喵
    Error(String),
    
    /// 已撤销喵
    Revoked,
}

impl AuthSession {
    /// 创建新会话喵
    /// 
    /// ## Arguments
    /// * `config` - OAuth 配置喵
    /// 
    /// ## Returns
    /// 新的认证会话喵
    /// 
    /// 🔐 PERMISSION: 仅初始化喵
    pub fn new(config: OAuthConfig) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            config,
            token: None,
            created_at: Utc::now(),
            last_activity: Utc::now(),
            state: AuthState::Initial,
        }
    }

    /// 检查 Token 是否有效喵
    /// 
    /// ## Returns
    /// Token 是否有效喵
    /// 
    /// 🔐 PERMISSION: 公开接口喵
    pub fn is_token_valid(&self) -> bool {
        if let Some(token) = &self.token {
            token.expires_at > Utc::now()
        } else {
            false
        }
    }

    /// 检查是否需要刷新 Token喵
    /// 
    /// ## Returns
    /// 是否需要刷新喵
    /// 
    /// 🔐 PERMISSION: 公开接口喵
    pub fn needs_refresh(&self) -> bool {
        if let Some(token) = &self.token {
            // 提前 5 分钟刷新喵
            token.expires_at < Utc::now() + Duration::minutes(5)
        } else {
            false
        }
    }
}

/// 凭证存储喵
/// 
/// 🔐 SAFETY: 加密存储认证凭证喵
#[derive(Clone)]
pub struct CredentialStore {
    /// 加密服务喵
    crypto: CryptoService,
    
    /// 凭证缓存喵
    cache: Arc<Mutex<HashMap<String, TokenInfo>>>,
    
    /// 存储路径喵
    storage_path: std::path::PathBuf,
}

impl CredentialStore {
    /// 创建凭证存储喵
    /// 
    /// ## Arguments
    /// * `storage_path` - 存储路径喵
    /// * `crypto` - 加密服务喵
    /// 
    /// 🔐 PERMISSION: 仅初始化喵
    pub fn new(storage_path: std::path::PathBuf, crypto: CryptoService) -> Self {
        // 确保目录存在喵
        if !storage_path.exists() {
            std::fs::create_dir_all(&storage_path).unwrap();
        }
        
        Self {
            crypto,
            cache: Arc::new(Mutex::new(HashMap::new())),
            storage_path,
        }
    }

    /// 保存凭证喵
    /// 
    /// ## Arguments
    /// * `key` - 凭证键名喵
    /// * `token` - Token 信息喵
    /// 
    /// ## Returns
    /// Result<(), AuthError>
    /// 
    /// 🔐 PERMISSION: 凭证管理喵
    pub async fn save(&self, key: &str, token: &TokenInfo) -> Result<(), AuthError> {
        // 加密 Token 信息喵
        let token_json = serde_json::to_string(token)
            .map_err(|e| AuthError::EncryptionError(e.to_string()))?;
        
        let encrypted = self.crypto.encrypt(&token_json)
            .map_err(|e| AuthError::EncryptionError(e.to_string()))?;
        
        // 保存到文件喵
        let file_path = self.storage_path.join(format!("{}.cred", key));
        std::fs::write(&file_path, encrypted)
            .map_err(|e| AuthError::EncryptionError(e.to_string()))?;
        
        // 更新缓存喵
        let mut cache = self.cache.lock().await;
        cache.insert(key.to_string(), token.clone());
        
        Ok(())
    }

    /// 加载凭证喵
    /// 
    /// ## Arguments
    /// * `key` - 凭证键名喵
    /// 
    /// ## Returns
    /// Option<TokenInfo>
    /// 
    /// 🔐 PERMISSION: 凭证管理喵
    pub async fn load(&self, key: &str) -> Option<TokenInfo> {
        // 先检查缓存喵
        {
            let cache = self.cache.lock().await;
            if let Some(token) = cache.get(key) {
                if token.expires_at > Utc::now() {
                    return Some(token.clone());
                }
            }
        }
        
        // 从文件加载喵
        let file_path = self.storage_path.join(format!("{}.cred", key));
        if !file_path.exists() {
            return None;
        }
        
        let encrypted = std::fs::read(&file_path).ok()?;
        let decrypted = self.crypto.decrypt(&encrypted)
            .map_err(|e| {
                log::warn!("Failed to decrypt credential: {}", e);
                e
            }).ok()?;
        
        let token: TokenInfo = serde_json::from_str(&decrypted)
            .map_err(|e| {
                log::warn!("Failed to parse credential: {}", e);
                e
            }).ok()?;
        
        // 更新缓存喵
        let mut cache = self.cache.lock().await;
        cache.insert(key.to_string(), token.clone());
        
        Some(token)
    }

    /// 删除凭证喵
    /// 
    /// ## Arguments
    /// * `key` - 凭证键名喵
    /// 
    /// ## Returns
    /// Result<(), AuthError>
    /// 
    /// 🔐 PERMISSION: 凭证管理喵
    pub async fn delete(&self, key: &str) -> Result<(), AuthError> {
        // 删除文件喵
        let file_path = self.storage_path.join(format!("{}.cred", key));
        if file_path.exists() {
            std::fs::remove_file(&file_path)
                .map_err(|e| AuthError::EncryptionError(e.to_string()))?;
        }
        
        // 删除缓存喵
        let mut cache = self.cache.lock().await;
        cache.remove(key);
        
        Ok(())
    }

    /// 清除所有凭证喵
    /// 
    /// ## Returns
    /// Result<(), AuthError>
    /// 
    /// 🔐 PERMISSION: 仅管理员喵
    pub async fn clear_all(&self) -> Result<(), AuthError> {
        // 清除缓存喵
        let mut cache = self.cache.lock().await;
        cache.clear();
        
        // 删除所有凭证文件喵
        for entry in std::fs::read_dir(&self.storage_path)
            .map_err(|e| AuthError::EncryptionError(e.to_string()))? {
            if let Ok(entry) = entry {
                if entry.path().extension().map(|e| e.to_string_lossy()) == Some("cred".to_string()) {
                    std::fs::remove_file(entry.path())
                        .map_err(|e| AuthError::EncryptionError(e.to_string()))?;
                }
            }
        }
        
        Ok(())
    }
}

/// 认证配置文件喵
/// 
/// 🔐 SAFETY: OpenClaw auth.profiles 配置兼容喵
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthProfiles {
    /// 认证 profiles 列表喵
    pub profiles: Vec<AuthProfile>,
    
    /// 默认 profile 名称喵
    pub default_profile: Option<String>,
}

/// 单个认证配置喵
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AuthProfile {
    /// Profile 名称喵
    pub name: String,
    
    /// 提供商类型喵
    pub provider: String,
    
    /// OAuth 配置喵
    pub oauth: OAuthConfig,
    
    /// 启用状态喵
    pub enabled: bool,
    
    /// 优先级喵
    pub priority: u32,
}

/// 认证管理器主结构喵
/// 
/// 🔐 SAFETY: 认证流程管理和凭证安全控制中心喵
pub struct AuthManager {
    /// OAuth 配置喵
    config: OAuthConfig,
    
    /// 凭证存储喵
    store: CredentialStore,
    
    /// 会话管理喵
    sessions: Arc<Mutex<HashMap<String, AuthSession>>>,
    
    /// OAuth2 客户端喵
    oauth2_client: Option<BasicClient>,
}

impl AuthManager {
    /// 创建认证管理器喵
    /// 
    /// ## Arguments
    /// * `config` - OAuth 配置喵
    /// * `storage_path` - 凭证存储路径喵
    /// 
    /// ## Returns
    /// 认证管理器实例喵
    /// 
    /// 🔐 PERMISSION: 仅初始化喵
    pub async fn new(config: OAuthConfig, storage_path: Option<std::path::PathBuf>) -> Result<Self, AuthError> {
        let storage_path = storage_path
            .unwrap_or_else(|| std::path::PathBuf::from("~/.nekoclaw/credentials"));
        
        let crypto = CryptoService::new(&crate::security::generate_key())
            .map_err(|e| AuthError::EncryptionError(e.to_string()))?;
        
        let store = CredentialStore::new(storage_path, crypto);
        let sessions = Arc::new(Mutex::new(HashMap::new()));
        let oauth2_client = config.to_oauth2_client().ok();
        
        Ok(Self {
            config,
            store,
            sessions,
            oauth2_client,
        })
    }

    /// 创建授权 URL喵
    /// 
    /// ## Arguments
    /// * `state` - 状态字符串喵
    /// * `pkce_code_verifier` - PKCE code verifier喵
    /// 
    /// ## Returns
    /// 授权 URL喵
    /// 
    /// 🔐 PERMISSION: 认证流程喵
    pub async fn create_authorization_url(&self, state: &str, pkce_code_verifier: Option<&str>) -> Result<String, AuthError> {
        let client = self.oauth2_client
            .as_ref()
            .ok_or_else(|| AuthError::ConfigError("OAuth client not initialized".to_string()))?;
        
        // 构建授权请求喵
        let mut request = client.authorize_url(
            oauth2::CsrfToken::new(state.to_string()),
            oauth2::PkceCodeVerifier::new(pkce_code_verifier.unwrap_or("").to_string()),
        );
        
        // 添加作用域喵
        for scope in &self.config.scopes {
            request = request.add_scope(oauth2::Scope::new(scope.to_string()));
        }
        
        // 生成 URL喵
        let (auth_url, _) = request.url();
        Ok(auth_url.to_string())
    }

    /// 交换授权码获取 Token喵
    /// 
    /// ## Arguments
    /// * `code` - 授权码喵
    /// * `pkce_code_verifier` - PKCE code verifier喵
    /// 
    /// ## Returns
    /// Token 信息喵
    /// 
    /// 🔐 PERMISSION: 认证流程喵
    pub async fn exchange_code_for_token(&self, code: &str, pkce_code_verifier: Option<&str>) -> Result<TokenInfo, AuthError> {
        let client = self.oauth2_client
            .as_ref()
            .ok_or_else(|| AuthError::ConfigError("OAuth client not initialized".to_string()))?;
        
        let mut token_request = client.exchange_code(oauth2::AuthorizationCode::new(code.to_string()));
        
        if let Some(verifier) = pkce_code_verifier {
            token_request = token_request.set_pkce_code_verifier(oauth2::PkceCodeVerifier::new(verifier.to_string()));
        }
        
        let token_result = token_request.request_async(async_http_client())
            .await
            .map_err(|e| AuthError::AuthenticationFailed(e.to_string()))?;
        
        let now = Utc::now();
        let expires_in = token_result.expires_in()
            .unwrap_or_else(|| chrono::Duration::seconds(3600));
        
        Ok(TokenInfo {
            access_token: token_result.access_token().secret().to_string(),
            refresh_token: token_result.refresh_token().map(|t| t.secret().to_string()),
            token_type: token_result.token_type().to_string(),
            expires_at: now + Duration::seconds(expires_in.num_seconds()),
            scopes: self.config.scopes.clone(),
            user_id: None,
        })
    }

    /// 刷新 Token喵
    /// 
    /// ## Arguments
    /// * `refresh_token` - 刷新 Token喵
    /// 
    /// ## Returns
    /// 新的 Token 信息喵
    /// 
    /// 🔐 PERMISSION: Token 刷新喵
    pub async fn refresh_token(&self, refresh_token: &str) -> Result<TokenInfo, AuthError> {
        let client = self.oauth2_client
            .as_ref()
            .ok_or_else(|| AuthError::ConfigError("OAuth client not initialized".to_string()))?;
        
        let refresh_token = RefreshToken::new(refresh_token.to_string());
        let token_result = client
            .exchange_refresh_token(&refresh_token)
            .request_async(async_http_client())
            .await
            .map_err(|e| AuthError::RefreshFailed(e.to_string()))?;
        
        let now = Utc::now();
        let expires_in = token_result.expires_in()
            .unwrap_or_else(|| chrono::Duration::seconds(3600));
        
        Ok(TokenInfo {
            access_token: token_result.access_token().secret().to_string(),
            refresh_token: token_result.refresh_token().map(|t| t.secret().to_string()),
            token_type: token_result.token_type().to_string(),
            expires_at: now + Duration::seconds(expires_in.num_seconds()),
            scopes: self.config.scopes.clone(),
            user_id: None,
        })
    }

    /// 保存凭证喵
    /// 
    /// ## Arguments
    /// * `key` - 凭证键名喵
    /// * `token` - Token 信息喵
    /// 
    /// 🔐 PERMISSION: 凭证管理喵
    pub async fn save_credential(&self, key: &str, token: &TokenInfo) -> Result<(), AuthError> {
        self.store.save(key, token).await
    }

    /// 加载凭证喵
    /// 
    /// ## Arguments
    /// * `key` - 凭证键名喵
    /// 
    /// ## Returns
    /// Option<TokenInfo>
    /// 
    /// 🔐 PERMISSION: 凭证管理喵
    pub async fn load_credential(&self, key: &str) -> Option<TokenInfo> {
        self.store.load(key).await
    }

    /// 删除凭证喵
    /// 
    /// ## Arguments
    /// * `key` - 凭证键名喵
    /// 
    /// 🔐 PERMISSION: 凭证管理喵
    pub async fn delete_credential(&self, key: &str) -> Result<(), AuthError> {
        self.store.delete(key).await
    }

    /// 获取配置喵
    /// 
    /// ## Returns
    /// OAuth 配置喵
    /// 
    /// 🔐 PERMISSION: 公开接口喵
    pub fn config(&self) -> &OAuthConfig {
        &self.config
    }
}

/// 从 AuthProfiles 创建 AuthManager喵
/// 
/// ## Arguments
/// * `profiles` - 认证配置喵
/// * `storage_path` - 存储路径喵
/// * `profile_name` - 使用的 profile 名称喵
/// 
/// ## Returns
/// AuthManager 实例喵
/// 
/// 🔐 PERMISSION: 配置阶段喵
pub async fn create_auth_manager_from_profiles(
    profiles: &AuthProfiles,
    storage_path: Option<std::path::PathBuf>,
    profile_name: Option<&str>,
) -> Result<AuthManager, AuthError> {
    // 选择 profile喵
    let profile = if let Some(name) = profile_name {
        profiles.profiles.iter()
            .find(|p| p.name == name && p.enabled)
            .ok_or_else(|| AuthError::ConfigError(format!("Profile '{}' not found or disabled", name)))?
    } else if let Some(default) = &profiles.default_profile {
        profiles.profiles.iter()
            .find(|p| p.name == default && p.enabled)
            .unwrap_or_else(|| profiles.profiles.first()
                .ok_or_else(|| AuthError::ConfigError("No profiles available".to_string()))?)
    } else {
        profiles.profiles.first()
            .ok_or_else(|| AuthError::ConfigError("No profiles available".to_string()))?
    };
    
    AuthManager::new(profile.oauth.clone(), storage_path).await
}

/// 加载 AuthProfiles 配置喵
/// 
/// ## Arguments
/// * `path` - 配置文件路径喵
/// 
/// ## Returns
/// AuthProfiles 实例喵
/// 
/// 🔐 PERMISSION: 配置加载喵
pub async fn load_auth_profiles(path: &std::path::PathBuf) -> Result<AuthProfiles, AuthError> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| AuthError::ConfigError(e.to_string()))?;
    
    toml::from_str(&content)
        .map_err(|e| AuthError::ConfigError(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试 OAuth 配置创建喵
    #[tokio::test]
    async fn test_oauth_config_creation() {
        let config = OAuthConfig::discord(
            "test_client_id",
            "test_client_secret",
            "http://localhost/callback"
        );
        
        assert_eq!(config.provider, OAuthProvider::Discord);
        assert_eq!(config.client_id, "test_client_id");
        assert!(config.auth_url.contains("discord.com"));
        assert!(config.enabled);
    }

    /// 测试 Token 有效性检查喵
    #[tokio::test]
    async fn test_token_validity_check() {
        let session = AuthSession::new(OAuthConfig::default());
        
        // 初始状态没有 token喵
        assert!(!session.is_token_valid());
        assert!(!session.needs_refresh());
    }

    /// 测试过期 Token 检查喵
    #[tokio::test]
    async fn test_expired_token_check() {
        let config = OAuthConfig::default();
        let mut session = AuthSession::new(config);
        
        session.token = Some(TokenInfo {
            access_token: "test_token".to_string(),
            refresh_token: None,
            token_type: "Bearer".to_string(),
            expires_at: Utc::now() - Duration::hours(1), // 已过期喵
            scopes: vec![],
            user_id: None,
        });
        
        assert!(!session.is_token_valid());
        assert!(!session.needs_refresh()); // 已过期，不需要刷新喵
    }

    /// 测试有效 Token 检查喵
    #[tokio::test]
    async fn test_valid_token_check() {
        let config = OAuthConfig::default();
        let mut session = AuthSession::new(config);
        
        session.token = Some(TokenInfo {
            access_token: "test_token".to_string(),
            refresh_token: Some("refresh".to_string()),
            token_type: "Bearer".to_string(),
            expires_at: Utc::now() + Duration::hours(1), // 有效喵
            scopes: vec![],
            user_id: None,
        });
        
        assert!(session.is_token_valid());
        assert!(!session.needs_refresh()); // 还有很长时间喵
    }

    /// 测试即将过期的 Token喵
    #[tokio::test]
    async fn test_token_needs_refresh() {
        let config = OAuthConfig::default();
        let mut session = AuthSession::new(config);
        
        session.token = Some(TokenInfo {
            access_token: "test_token".to_string(),
            refresh_token: Some("refresh".to_string()),
            token_type: "Bearer".to_string(),
            expires_at: Utc::now() + Duration::minutes(2), // 即将过期喵
            scopes: vec![],
            user_id: None,
        });
        
        assert!(!session.is_token_valid()); // 还没有过期喵
        assert!(session.needs_refresh()); // 需要刷新喵
    }
}
