//!
//! # Build Script
//!
//! Generate version information using vergen

use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // Generate git and build info
    vergen::EmitBuilder::builder()
        .git_sha(true)
        .git_commit_date()
        .rustc_version()
        .cargo_features()
        .emit()?;
    
    Ok(())
}
