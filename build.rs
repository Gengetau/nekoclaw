//!
//! # Build Script
//!
//! Simple build script for Neko-Claw

fn main() {
    // Tell Cargo to re-run this script if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}
