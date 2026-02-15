//!
//! # Build Script
//!
//! Generate version information at compile time using vergen with gitcl mode.
//! This uses the system's git command line tool instead of the gix library.

fn main() {
    // Tell Cargo to rerun if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
    
    // Emit build info using vergen with gitcl (Git Command Line) mode
    // gitcl uses the system git command instead of gix library
    vergen::EmitBuilder::builder()
        .build_timestamp()
        .rustc_semver()           // Correct API for rustc version
        .cargo_features()
        .git_sha(true)            // Short SHA
        .git_commit_date()
        .gitcl()                  // Use git command line instead of gix
        .emit()
        .expect("Failed to emit vergen instructions");
}
