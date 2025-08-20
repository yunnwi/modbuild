use std::env;
use std::process::Command;

/// Checks if a macOS compiler is available
pub fn has_mac_compiler() -> bool {
    if let Ok(path) = env::var("CC") {
        return path.contains("o64-clang") || path.contains("zig");
    }
    Command::new("which")
        .arg("zig")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}