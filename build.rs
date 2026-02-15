//!
//! # Build Script
//!
//! Generate version information at compile time using vergen with gitcl mode.

fn main() {
    // Tell Cargo to rerun if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
    
    // Emit build info using vergen with gitcl (Git Command Line) mode
    // gitcl uses the system git command instead of gix library
    vergen::EmitBuilder::builder()
        .build_timestamp()
        .emit()
        .expect("Failed to emit vergen instructions");
}
