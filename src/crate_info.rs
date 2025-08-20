use std::process::Command;
use std::path::PathBuf;
use serde_json::Value;

/// Get crate name from Cargo metadata
pub fn get_crate_name(mod_path: &PathBuf) -> Result<String, String> {
    let output = Command::new("cargo")
        .args(&["metadata", "--format-version", "1", "--no-deps"])
        .current_dir(mod_path)
        .output()
        .map_err(|e| format!("Failed to run cargo metadata: {e}"))?;

    if !output.status.success() {
        return Err("cargo metadata failed".into());
    }

    let stdout = String::from_utf8(output.stdout)
        .map_err(|e| format!("Invalid UTF-8 from cargo metadata: {e}"))?;

    let metadata: Value = serde_json::from_str(&stdout)
        .map_err(|e| format!("Failed to parse cargo metadata JSON: {e}"))?;

    let package_name = metadata["packages"][0]["name"]
        .as_str()
        .ok_or("Failed to get package name")?;

    Ok(package_name.to_string())
}