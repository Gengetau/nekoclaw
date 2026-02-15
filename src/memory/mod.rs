/*!
 * Memory Module
 *
 * 作者: 缪斯 (Muse) @缪斯
 * 日期: 2026-02-15 18:20 JST
 *
 * 功能:
 * - SQLite 后端 + FTS5 全文搜索
 * - 简化向量存储 (不依赖外部库)
 * - OpenClaw IDENTITY.md 兼容解析
 */

pub mod sqlite;
pub mod vector;
pub mod identity_parser;

// 重新导出所有子模块接口
pub use sqlite::SqliteMemory;
pub use vector::SimpleVectorDB;
pub use identity_parser::{IdentityParser, OpenClawIdentity};

// 为了兼容性，导出 MemoryFactory 为 MemoryManager
pub use MemoryFactory as MemoryManager;

use std::sync::Arc;
use std::path::PathBuf;
use crate::core::traits::*;

/// Memory 工厂 - 创建不同类型的 Memory 实现
pub struct MemoryFactory;

impl MemoryFactory {
    /// 创建 SQLite Memory 实例
    pub fn create_sqlite(path: &str) -> Result<Arc<dyn Memory>> {
        let memory = SqliteMemory::new(path)?;
        Ok(Arc::new(memory))
    }

    /// 带向量搜索的 SQLite Memory
    pub fn create_sqlite_with_vector(path: &str) -> Result<Arc<dyn Memory>> {
        let memory = SqliteMemory::new_with_vector(path)?;
        Ok(Arc::new(memory))
    }
}
