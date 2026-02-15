/// 配置验证模块 🔍
///
/// @诺诺 的配置验证器实现喵
///
/// 功能：
/// - 必填项检查
- 配置类型验证
- 配置范围检查
- 迁移前验证
///
/// 🔒 SAFETY: 验证失败必须阻断启动
///
/// 实现者: 诺诺 (Nono) ⚡

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 🔒 SAFETY: 验证错误类型喵
#[derive(Debug)]
pub enum ValidationError {
    /// 缺少必填项
    #[error("Missing required field: {0}")]
    MissingRequired(String),
    /// 类型不匹配
    #[error("Type mismatch for field '{0}': expected {1}, got {2}")]
    TypeMismatch(String, String, String),
    /// 值超出范围
    #[error("Value out of range for field '{0}': {1} not in {2}..{3}")]
    OutOfRange(String, String, String, String),
    /// 无效的值
    #[error("Invalid value for field '{0}': {1}")]
    InvalidValue(String, String),
    /// 格式错误
    #[error("Invalid format for field '{0}': {1}")]
    InvalidFormat(String, String),
    /// 依赖项缺失
    #[error("Missing dependency: {0} requires {1}")]
    MissingDependency(String, String),
    /// 多个错误
    #[error("Multiple validation errors: {0}")]
    Multiple(Vec<ValidationError>),
}

/// 🔒 SAFETY: 验证规则结构体喵
#[derive(Debug, Clone)]
pub struct ValidationRule {
    /// 字段名
    pub field_name: String,
    /// 是否必填
    pub required: bool,
    /// 期望类型
    pub expected_type: Option<String>,
    /// 最小值（数字）
    pub min: Option<f64>,
    /// 最大值（数字）
    pub max: Option<f64>,
    /// 最小长度（字符串/数组）
    pub min_length: Option<usize>,
    /// 最大长度（字符串/数组）
    pub max_length: Option<usize>,
    /// 允许的值（枚举）
    pub allowed_values: Option<Vec<String>>,
    /// 正则表达式格式验证
    pub regex_pattern: Option<String>,
    /// 依赖的字段
    pub dependencies: Vec<String>,
}

impl ValidationRule {
    /// 🔒 SAFETY: 创建新的验证规则喵
    pub fn new(field_name: String) -> Self {
        Self {
            field_name,
            required: false,
            expected_type: None,
            min: None,
            max: None,
            min_length: None,
            max_length: None,
            allowed_values: None,
            regex_pattern: None,
            dependencies: Vec::new(),
        }
    }

    /// 🔒 SAFETY: 设置为必填喵
    pub fn required(mut self) -> Self {
        self.required = true;
        self
    }

    /// 🔒 SAFETY: 设置期望类型喵
    pub fn with_type(mut self, type_name: String) -> Self {
        self.expected_type = Some(type_name);
        self
    }

    /// 🔒 SAFETY: 设置数值范围喵
    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    /// 🔒 SAFETY: 设置长度范围喵
    pub fn with_length_range(mut self, min_len: usize, max_len: usize) -> Self {
        self.min_length = Some(min_len);
        self.max_length = Some(max_len);
        self
    }

    /// 🔒 SAFETY: 设置允许的值喵
    pub fn with_allowed_values(mut self, values: Vec<String>) -> Self {
        self.allowed_values = Some(values);
        self
    }

    /// 🔒 SAFETY: 设置正则表达式喵
    pub fn with_pattern(mut self, pattern: String) -> Self {
        self.regex_pattern = Some(pattern);
        self
    }

    /// 🔒 SAFETY: 添加依赖喵
    pub fn with_dependency(mut self, dependency: String) -> Self {
        self.dependencies.push(dependency);
        self
    }
}

/// 🔒 SAFETY: 配置验证器喵
pub struct ConfigValidator {
    /// 验证规则集合
    rules: HashMap<String, ValidationRule>,
}

impl ConfigValidator {
    /// 🔒 SAFETY: 创建新的配置验证器喵
    pub fn new() -> Self {
        Self {
            rules: HashMap::new(),
        }
    }

    /// 🔒 SAFETY: 添加验证规则喵
    pub fn add_rule(&mut self, rule: ValidationRule) {
        self.rules.insert(rule.field_name.clone(), rule);
    }

    /// 🔒 SAFETY: 批量添加验证规则喵
    pub fn add_rules(&mut self, rules: Vec<ValidationRule>) {
        for rule in rules {
            self.add_rule(rule);
        }
    }

    /// 🔒 SAFETY: 验证配置值喵
    /// 异常处理: 验证失败返回 ValidationError
    pub fn validate(&self, config: &serde_json::Value) -> Result<(), ValidationError> {
        let mut errors = Vec::new();

        for (field_name, rule) in &self.rules {
            // 检查必填项
            if rule.required && !config.get(field_name).is_some() {
                errors.push(ValidationError::MissingRequired(field_name.clone()));
                continue;
            }

            // 获取字段值
            let value = match config.get(field_name) {
                Some(v) => v,
                None => continue, // 非必填项且不存在，跳过
            };

            // 检查依赖项
            for dep in &rule.dependencies {
                if !config.get(dep).is_some() {
                    errors.push(ValidationError::MissingDependency(
                        field_name.clone(),
                        dep.clone(),
                    ));
                }
            }

            // 类型检查
            if let Some(ref expected_type) = rule.expected_type {
                let actual_type = match value {
                    serde_json::Value::String(_) => "string",
                    serde_json::Value::Number(_) => "number",
                    serde_json::Value::Bool(_) => "boolean",
                    serde_json::Value::Array(_) => "array",
                    serde_json::Value::Object(_) => "object",
                    serde_json::Value::Null => "null",
                };

                if actual_type != expected_type {
                    errors.push(ValidationError::TypeMismatch(
                        field_name.clone(),
                        expected_type.clone(),
                        actual_type.to_string(),
                    ));
                }
            }

            // 数值范围检查
            if let (Some(ref value), Some(min), Some(max)) = (
                value.as_f64(),
                rule.min,
                rule.max,
            ) {
                if *value < min || *value > max {
                    errors.push(ValidationError::OutOfRange(
                        field_name.clone(),
                        value.to_string(),
                        min.to_string(),
                        max.to_string(),
                    ));
                }
            }

            // 长度范围检查（字符串）
            if let Some(ref str_val) = value.as_str() {
                if let (Some(min_len), Some(max_len)) = (rule.min_length, rule.max_length) {
                    let len = str_val.chars().count();
                    if len < min_len || len > max_len {
                        errors.push(ValidationError::OutOfRange(
                            field_name.clone(),
                            len.to_string(),
                            min_len.to_string(),
                            max_len.to_string(),
                        ));
                    }
                }
            }

            // 长度范围检查（数组）
            if let Some(ref arr_val) = value.as_array() {
                if let (Some(min_len), Some(max_len)) = (rule.min_length, rule.max_length) {
                    let len = arr_val.len();
                    if len < min_len || len > max_len {
                        errors.push(ValidationError::OutOfRange(
                            field_name.clone(),
                            len.to_string(),
                            min_len.to_string(),
                            max_len.to_string(),
                        ));
                    }
                }
            }

            // 允许的值检查
            if let Some(ref allowed) = rule.allowed_values {
                let str_value = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    _ => continue,
                };

                if !allowed.contains(&str_value) {
                    errors.push(ValidationError::InvalidValue(
                        field_name.clone(),
                        str_value,
                    ));
                }
            }

            // 正则表达式检查
            if let (Some(ref pattern), Some(ref str_val)) = (&rule.regex_pattern, value.as_str()) {
                match regex::Regex::new(pattern) {
                    Ok(re) => {
                        if !re.is_match(str_val) {
                            errors.push(ValidationError::InvalidFormat(
                                field_name.clone(),
                                pattern.clone(),
                            ));
                        }
                    }
                    Err(e) => {
                        errors.push(ValidationError::InvalidFormat(
                            field_name.clone(),
                            format!("Invalid regex: {}", e),
                        ));
                    }
                }
            }
        }

        if errors.is_empty() {
            Ok(())
        } else if errors.len() == 1 {
            Err(errors.into_iter().next().unwrap())
        } else {
            Err(ValidationError::Multiple(errors))
        }
    }

    /// 🔒 SAFETY: 验证 YAML 配置喵
    pub fn validate_yaml(&self, yaml_str: &str) -> Result<(), ValidationError> {
        let config: serde_json::Value = serde_yaml::from_str(yaml_str)
            .map_err(|e| ValidationError::InvalidFormat("root".to_string(), e.to_string()))?;
        self.validate(&config)
    }

    /// 🔒 SAFETY: 验证 JSON 配置喵
    pub fn validate_json(&self, json_str: &str) -> Result<(), ValidationError> {
        let config: serde_json::Value = serde_json::from_str(json_str)
            .map_err(|e| ValidationError::InvalidFormat("root".to_string(), e.to_string()))?;
        self.validate(&config)
    }
}

impl Default for ConfigValidator {
    fn default() -> Self {
        Self::new()
    }
}

/// 🔒 SAFETY: 验证结果结构体喵
#[derive(Debug, Clone, Serialize)]
pub struct ValidationResult {
    /// 是否通过
    pub passed: bool,
    /// 错误列表
    pub errors: Vec<String>,
    /// 警告列表
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// 🔒 SAFETY: 创建成功的验证结果喵
    pub success() -> Self {
        Self {
            passed: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// 🔒 SAFETY: 创建失败的验证结果喵
    pub failure(error: ValidationError) -> Self {
        Self {
            passed: false,
            errors: vec![error.to_string()],
            warnings: Vec::new(),
        }
    }

    /// 🔒 SAFETY: 添加警告喵
    pub fn with_warning(mut self, warning: String) -> Self {
        self.warnings.push(warning);
        self
    }
}

/// 🔒 SAFETY: 配置迁移验证器喵
/// 专门用于验证 OpenClaw 到 Neko-Claw 的配置迁移
pub struct MigrationValidator {
    /// 验证器
    validator: ConfigValidator,
}

impl MigrationValidator {
    /// 🔒 SAFETY: 创建新的迁移验证器喵
    pub fn new() -> Self {
        let mut validator = ConfigValidator::new();

        // Provider 配置验证
        validator.add_rule(
            ValidationRule::new("models.providers.nvidia.apiKey")
                .required()
                .with_type("string".to_string())
                .with_length_range(1, 1000),
        );

        // Discord Token 验证
        validator.add_rule(
            ValidationRule::new("channels.discord.accounts.main_bot.token")
                .required()
                .with_type("string".to_string())
                .with_pattern(r"^[A-Za-z0-9._-]{24,}\.[A-Za-z0-9._-]{6,}\.[A-Za-z0-9._-]{27,}$".to_string()),
        );

        // Agent 模型验证
        validator.add_rule(
            ValidationRule::new("agents.defaults.model.primary")
                .required()
                .with_type("string".to_string()),
        );

        // 内存验证
        validator.add_rule(
            ValidationRule::new("memory.enabled")
                .with_type("boolean".to_string()),
        );

        // 性能配置验证
        validator.add_rule(
            ValidationRule::new("performance.maxContextTokens")
                .with_type("number".to_string())
                .with_range(1000.0, 128000.0),
        );

        Self { validator }
    }

    /// 🔒 SAFETY: 验证 OpenClaw 配置喵
    pub fn validate_openclaw_config(&self, config: &serde_json::Value) -> Result<ValidationResult, ValidationError> {
        self.validator.validate(config)?;
        Ok(ValidationResult::success())
    }

    /// 🔒 SAFETY: 验证迁移后的 Neko-Claw 配置喵
    pub fn validate_nekoclaw_config(&self, config: &serde_json::Value) -> Result<ValidationResult, ValidationError> {
        // TODO: 添加 Neko-Claw 特有的验证规则
        self.validator.validate(config)?;
        Ok(ValidationResult::success())
    }
}

impl Default for MigrationValidator {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_rule_creation() {
        let rule = ValidationRule::new("test_field".to_string())
            .required()
            .with_type("string".to_string())
            .with_length_range(1, 100);

        assert_eq!(rule.field_name, "test_field");
        assert!(rule.required);
        assert_eq!(rule.expected_type, Some("string".to_string()));
    }

    #[test]
    fn test_config_validator_required_field() {
        let mut validator = ConfigValidator::new();
        validator.add_rule(
            ValidationRule::new("required_field".to_string())
                .required()
        );

        let config = serde_json::json!({});
        let result = validator.validate(&config);

        assert!(result.is_err());
        match result {
            Err(ValidationError::MissingRequired(field)) => assert_eq!(field, "required_field"),
            _ => panic!("Expected MissingRequired error"),
        }
    }

    #[test]
    fn test_config_validator_type_mismatch() {
        let mut validator = ConfigValidator::new();
        validator.add_rule(
            ValidationRule::new("age".to_string())
                .with_type("number".to_string())
        );

        let config = serde_json::json!({ "age": "not a number" });
        let result = validator.validate(&config);

        assert!(result.is_err());
        match result {
            Err(ValidationError::TypeMismatch(field, expected, actual)) => {
                assert_eq!(field, "age");
                assert_eq!(expected, "number");
                assert_eq!(actual, "string");
            }
            _ => panic!("Expected TypeMismatch error"),
        }
    }

    #[test]
    fn test_config_validator_success() {
        let mut validator = ConfigValidator::new();
        validator.add_rule(
            ValidationRule::new("name".to_string())
                .required()
                .with_type("string".to_string())
                .with_length_range(1, 50),
        );

        let config = serde_json::json!({ "name": "Test User" });
        let result = validator.validate(&config);

        assert!(result.is_ok());
    }

    #[test]
    fn test_validation_result() {
        let success = ValidationResult::success();
        assert!(success.passed);
        assert!(success.errors.is_empty());

        let error = ValidationError::MissingRequired("field".to_string());
        let failure = ValidationResult::failure(error).with_warning("This is a warning".to_string());
        assert!(!failure.passed);
        assert_eq!(failure.warnings.len(), 1);
    }

    #[test]
    fn test_migration_validator() {
        let validator = MigrationValidator::new();

        let valid_config = serde_json::json!({
            "models": {
                "providers": {
                    "nvidia": {
                        "apiKey": "test-api-key-123456"
                    }
                }
            },
            "channels": {
                "discord": {
                    "accounts": {
                        "main_bot": {
                            "token": "DISCORD_BOT_TOKEN_PLACEHOLDER"
                        }
                    }
                }
            },
            "agents": {
                "defaults": {
                    "model": {
                        "primary": "nvidia/z-ai/glm4.7"
                    }
                }
            }
        });

        let result = validator.validate_openclaw_config(&valid_config);
        assert!(result.is_ok());
    }
}
