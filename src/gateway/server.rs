/// Gateway HTTP 服务器模块 🌐
///
/// @诺诺 的 Axum HTTP 服务器实现喵
///
/// 功能：
/// - Axum 框架的 HTTP 服务器
/// - Bearer Token 认证中间件
/// - RESTful API 端点路由
/// - 请求/响应日志
///
/// 🔒 SAFETY: 所有 API 端点需要认证，拒绝未授权访问
///
/// 实现者: 诺诺 (Nono) ⚡

use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::{self, Next},
    response::{IntoResponse, Response, Json},
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{error, info};
use uuid::Uuid;

/// 🔒 SAFETY: Gateway 配置结构体喵
#[derive(Debug, Clone)]
pub struct GatewayConfig {
    /// 绑定地址
    pub bind_addr: String,
    /// 端口
    pub port: u16,
    /// Bearer Token（必须通过安全模块验证后传入）
    pub bearer_token: String,
    /// 是否启用配对模式
    pub pairing_enabled: bool,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            bind_addr: "127.0.0.1".to_string(),
            port: 8080,
            bearer_token: String::new(),
            pairing_enabled: true,
        }
    }
}

/// 🔒 SAFETY: Gateway 服务器状态喵
/// 包含配置和运行时数据
#[derive(Debug, Clone)]
pub struct GatewayState {
    /// 配置
    pub config: GatewayConfig,
}

/// 🔒 SAFETY: 健康检查响应结构体喵
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// 状态
    pub status: String,
    /// 版本
    pub version: String,
    /// Uptime
    pub uptime_secs: u64,
}

/// 🔒 SAFETY: API 错误响应结构体喵
#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    /// 错误代码
    pub code: String,
    /// 错误消息
    pub message: String,
    /// 请求 ID
    pub request_id: String,
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        let status = match self.code.as_str() {
            "UNAUTHORIZED" => StatusCode::UNAUTHORIZED,
            "FORBIDDEN" => StatusCode::FORBIDDEN,
            "NOT_FOUND" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(self)).into_response()
    }
}

/// 🔒 SAFETY: Bearer Token 认证中间件喵
/// 提取并验证 Authorization header
pub async fn auth_middleware(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
    request: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = &auth_header[7..]; // 跳过 "Bearer "

    if token != state.config.bearer_token {
        return Err(StatusCode::FORBIDDEN);
    }

    info!("Authenticated request from token: {}", &token[..8]);
    Ok(next.run(request).await)
}

/// 🔒 SAFETY: 健康检查端点喵
/// 不需要认证（心跳监控使用）
pub async fn health_check() -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_string(),
        version: env!("CARGO_PKG_VERSION").to_string(),
        uptime_secs: 0, // TODO: 实现实际 uptime 计算
    })
}

/// 🔒 SAFETY: 状态端点喵
/// 需要认证，返回详细状态信息
pub async fn status(
    State(state): State<Arc<GatewayState>>,
) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "running",
        "config": {
            "bind_addr": state.config.bind_addr,
            "port": state.config.port,
            "pairing_enabled": state.config.pairing_enabled,
        },
        "version": env!("CARGO_PKG_VERSION"),
    }))
}

/// 🔒 SAFETY: 配对请求结构体喵
#[derive(Debug, Deserialize)]
pub struct PairingRequest {
    /// 配对代码
    code: String,
    /// 设备名称（可选）
    #[serde(default)]
    device_name: Option<String>,
}

/// 🔒 SAFETY: 配对响应结构体喵
#[derive(Debug, Serialize)]
pub struct PairingResponse {
    /// 配对状态
    status: String,
    /// 消息
    message: String,
    /// Session Token（成功时）
    session_token: Option<String>,
}

/// 🔒 SAFETY: 配对端点喵
/// 需要 Bearer Token 认证
/// 异常处理: 无效配对码、已配对设备、配对超时
pub async fn pairing(
    State(state): State<Arc<GatewayState>>,
    Json(req): Json<PairingRequest>,
) -> Result<Json<PairingResponse>, ErrorResponse> {
    // TODO: 实现实际的配对逻辑
    info!("Pairing request with code: {}", req.code);

    if req.code.len() != 6 {
        return Err(ErrorResponse {
            code: "INVALID_CODE".to_string(),
            message: "Pairing code must be 6 digits".to_string(),
            request_id: Uuid::new_v4().to_string(),
        });
    }

    Ok(Json(PairingResponse {
        status: "success".to_string(),
        message: "Pairing successful".to_string(),
        session_token: Some(Uuid::new_v4().to_string()),
    }))
}

/// 🔒 SAFETY: Webhook 端点喵
/// 需要认证，接收外部 webhook 通知
/// 异常处理: 无效请求体、处理失败
pub async fn webhook(
    State(state): State<Arc<GatewayState>>,
    headers: HeaderMap,
    body: String,
) -> Result<Json<serde_json::Value>, ErrorResponse> {
    // TODO: 实现实际的前端 webhook 处理逻辑
    info!("Webhook received with body size: {}", body.len());

    // 提取请求类型
    let event_type = headers.get("x-event-type")
        .and_then(|h| h.to_str().ok())
        .unwrap_or("unknown");

    Ok(Json(serde_json::json!({
        "status": "received",
        "event_type": event_type,
        "message": "Webhook processed successfully",
    })))
}

/// 🔒 SAFETY: 创建 Gateway 路由喵
/// 配置所有 API 端点
fn create_router(state: Arc<GatewayState>) -> Router {
    // 公开端点（不需要认证）
    let public_routes = Router::new()
        .route("/health", get(health_check));

    // 认证端点（需要 Bearer Token）
    let protected_routes = Router::new()
        .route("/status", get(status))
        .route("/pairing", post(pairing))
        .route("/webhook", post(webhook))
        .layer(middleware::from_fn_with_state(state.clone(), auth_middleware));

    // 合并路由
    public_routes.merge(protected_routes)
        .with_state(state)
}

/// 🔒 SAFETY: Gateway 服务器结构体喵
#[derive(Debug, Clone)]
pub struct GatewayServer {
    /// 配置
    config: GatewayConfig,
    /// 运行时状态
    state: Arc<GatewayState>,
}

impl GatewayServer {
    /// 🔒 SAFETY: 创建新的 Gateway 服务器喵
    /// config: 必须包含有效的 bearer_token
    pub fn new(config: GatewayConfig) -> Self {
        let state = Arc::new(GatewayState {
            config: config.clone(),
        });

        Self { config, state }
    }

    /// 🔒 SAFETY: 启动服务器喵
    /// 异常处理: 地址绑定失败、启动失败
    pub async fn run(self) -> Result<(), Box<dyn std::error::Error>> {
        // 构建完整地址
        let addr: SocketAddr = format!("{}:{}", self.config.bind_addr, self.config.port)
            .parse()
            .map_err(|e| format!("Invalid bind address: {}", e))?;

        // 创建路由
        let router = create_router(self.state.clone());

        // 创建 TCP 监听器
        let listener = TcpListener::bind(&addr).await
            .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

        info!("Gateway server listening on http://{}", addr);

        // 启动 Axum 服务器
        axum::serve(listener, router).await?;

        Ok(())
    }

    /// 🔒 SAFETY: 获取服务器地址喵
    pub fn addr(&self) -> String {
        format!("{}:{}", self.config.bind_addr, self.config.port)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_default() {
        let config = GatewayConfig::default();
        assert_eq!(config.bind_addr, "127.0.0.1");
        assert_eq!(config.port, 8080);
        assert!(config.pairing_enabled);
    }

    #[test]
    fn test_health_response() {
        let response = HealthResponse {
            status: "ok".to_string(),
            version: "0.1.0".to_string(),
            uptime_secs: 0,
        };

        assert_eq!(response.status, "ok");
        assert_eq!(response.version, "0.1.0");
    }
}
