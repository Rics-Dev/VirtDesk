// 1. Register the commands file as a public sub-module
pub mod commands;

// 2. Re-export the items so they are available at crate::qemu_backend::QemuCmd
pub use commands::{QemuCmd, Format};