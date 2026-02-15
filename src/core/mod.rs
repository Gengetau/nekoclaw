/*!
 * Core Module
 *
 * 作者: 缪斯 (Muse) @缪斯
 */

pub mod traits;
pub mod config;

pub use config::{load as load_config, save as save_config};
pub use traits::*;
