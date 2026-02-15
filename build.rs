//!
//! # Build Script
//!
//! ⚠️ SAFETY: 生成版本信息喵
//!
//! 使用 vergen 生成 Git 和构建时间信息喵

fn main() {
    // 使用 vergen 生成版本信息喵
    vergen::Output::default()
        .git_sha(true)
        .git_commit_date(true)
        .rustc_version(true)
        .cargo_features(true)
        .emit()
        .expect("Failed to generate version info");
}
