// modbuild/src/build.rs

use crate::utils::{has_mac_compiler, has_zigbuild, is_host_triple};
use std::fs;
use std::io::BufRead;
use std::path::PathBuf;
use std::process::{Command, Stdio};

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
        BuildTarget {
            name: "linux",
            triple: "x86_64-unknown-linux-gnu",
            ext: "so",
            needs_mac: false,
        },
        BuildTarget {
            name: "windows-gnu",
            triple: "x86_64-pc-windows-gnu",
            ext: "dll",
            needs_mac: false,
        },
        BuildTarget {
            name: "windows-msvc",
            triple: "x86_64-pc-windows-msvc",
            ext: "dll",
            needs_mac: false,
        },
        BuildTarget {
            name: "mac-intel",
            triple: "x86_64-apple-darwin",
            ext: "dylib",
            needs_mac: true,
        },
        BuildTarget {
            name: "mac-arm64",
            triple: "aarch64-apple-darwin",
            ext: "dylib",
            needs_mac: true,
        },
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
pub fn build_for_target(
    out: &PathBuf,
    target: &BuildTarget,
    mod_path: &PathBuf,
) -> Result<(), String> {
    if target.needs_mac && !has_mac_compiler() {
        return Err(format!("Skipping {}: mac compiler not found", target.name));
    }

    #[cfg(target_os = "macos")]
    if target.triple == "x86_64-pc-windows-msvc" {
        return Err("Skipping windows-msvc: cannot cross-compile MSVC from macOS".into())
    }

    println!("Building for {}...", target.name);

    // Choose cargo subcommand: prefer zigbuild for macOS / cross, else plain build
    let use_zig = if target.needs_mac {
        has_zigbuild() && has_mac_compiler()
    } else if !is_host_triple(target.triple) {
        has_zigbuild()
    } else {
        false
    };

    let mut cmd = Command::new("cargo");
    cmd.current_dir(mod_path);

    if use_zig {
        cmd.arg("zigbuild");
    } else {
        cmd.arg("build");
    }

    cmd.args(["--release", "--target", target.triple, "--message-format=json"]);
    cmd.stdout(Stdio::piped());
    cmd.stderr(Stdio::inherit());

    let mut child = cmd.spawn().map_err(|e| format!("Failed to run cargo: {e}"))?;
    let stdout = std::io::BufReader::new(child.stdout.take().unwrap());

    let mut built_files: Vec<PathBuf> = Vec::new();

    for line in stdout.lines() {
        let line = line.map_err(|e| format!("IO error: {e}"))?;
        if line.is_empty() { continue; }
        let Ok(v) = serde_json::from_str::<serde_json::Value>(&line) else { continue; };
        if v["reason"] == "compiler-artifact" && v["target"]["kind"].as_array()
          .map(|kinds| kinds.iter().any(|k| k == "cdylib"))
          .unwrap_or(false)
        {
            if let Some(arr) = v["filenames"].as_array() {
                for p in arr {
                    if let Some(s) = p.as_str() {
                        if s.ends_with(&format!(".{}", target.ext)) {
                            built_files.push(PathBuf::from(s));
                        }
                    }
                }
            }
        }
    }

    let status = child.wait().map_err(|e| format!("cargo wait failed: {e}"))?;
    if !status.success() {
        return Err(format!("Build failed for {}", target.name));
    }

    if built_files.is_empty() {
        let guess = mod_path.join(format!("target/{}/release", target.triple));
        return Err(format!("Built file not found (no cdylib reported). Checked cargo JSON; try look in: {}", guess.display()));
    }

    let chosen = choose_best(&built_files, target).ok_or_else(|| {
        format!("No suitable {} found among: {:?}", target.ext, built_files)
    })?;

    println!("Built {} successfully.", target.name);

    let file_name = chosen.file_name().unwrap().to_string_lossy().into_owned();
    let out_file = add_suffix(&file_name, &format!("-{}", target.name));
    let out_path = out.join(out_file);

    fs::create_dir_all(out).map_err(|e| format!("Failed to create output directory: {e}"))?;
    fs::copy(&chosen, &out_path).map_err(|e| format!("Failed to copy file: {e}"))?;

    println!("Copied to {}", out_path.display());
    Ok(())
}

fn choose_best(files: &[PathBuf], target: &BuildTarget) -> Option<PathBuf> {
    for f in files {
        if f.extension().and_then(|e| e.to_str()) == Some(target.ext)
          && f.to_string_lossy().contains(&format!("/target/{}/", target.triple))
        {
            return Some(f.clone());
        }
    }
    files.iter()
      .find(|f| f.extension().and_then(|e| e.to_str()) == Some(target.ext))
      .cloned()
}

fn add_suffix(file_name: &str, suffix: &str) -> String {
    if let Some((base, ext)) = file_name.rsplit_once(".") {
        format!("{base}{suffix}.{ext}")
    } else {
        format!("{file_name}{suffix}")
    }
}