use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use crate::utils::has_mac_compiler;

/// Represents a build target (OS + architecture)
#[derive(Clone)]
pub struct BuildTarget {
    pub name: &'static str,
    pub triple: &'static str,
    pub ext: &'static str,
    pub needs_mac: bool,
}

/// All supported targets
pub fn all_targets() -> Vec<BuildTarget> {
    vec![
        BuildTarget { name: "linux", triple: "x86_64-unknown-linux-gnu", ext: "so", needs_mac: false },
        BuildTarget { name: "windows", triple: "x86_64-pc-windows-gnu", ext: "dll", needs_mac: false },
        BuildTarget { name: "mac-intel", triple: "x86_64-apple-darwin", ext: "dylib", needs_mac: true },
        BuildTarget { name: "mac-arm64", triple: "aarch64-apple-darwin", ext: "dylib", needs_mac: true },
    ]
}

/// Select targets based on optional filter string
pub fn select_targets(filter: Option<String>) -> Vec<BuildTarget> {
    let all = all_targets();
    if let Some(f) = filter {
        f.split(',')
            .filter_map(|name| all.iter().find(|t| t.name == name.trim()))
            .cloned()
            .collect()
    } else {
        all
    }
}

/// Build a crate for a specific target
pub fn build_for_target(crate_name: &str, out: &PathBuf, target: &BuildTarget, mod_path: &PathBuf) -> Result<(), String> {
    if target.needs_mac && !has_mac_compiler() {
        return Err(format!("Skipping {}: mac compiler not found", target.name));
    }

    println!("Building for {}...", target.name);

    let lib_basename = if target.ext == "dll" {
        crate_name.replace('-', "_")
    } else {
        format!("lib{}", crate_name.replace('-', "_"))
    };

    let status = Command::new("cargo")
        .arg("zigbuild")
        .args(["--release", "--target", target.triple])
        .current_dir(mod_path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .map_err(|e| format!("Failed to run cargo: {e}"))?;

    if !status.success() {
        return Err(format!("Build failed for {}", target.name));
    }

    println!("Built {} successfully.", target.name);

    let built_path = mod_path.join(format!("target/{}/release/{}.{}", target.triple, lib_basename, target.ext));
    if !built_path.exists() {
        return Err(format!("Built file not found: {}", built_path.display()));
    }

    let out_name = format!("{}-{}.{}", lib_basename, target.name, target.ext);
    let out_path = out.join(out_name);

    fs::create_dir_all(out).map_err(|e| format!("Failed to create output directory: {e}"))?;
    fs::copy(&built_path, &out_path).map_err(|e| format!("Failed to copy file: {e}"))?;

    println!("Copied to {}", out_path.display());
    Ok(())
}