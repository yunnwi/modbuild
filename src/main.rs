use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn main() {
    let args: Vec<String> = env::args().collect();
    let output_dir = args.get(1).map(|p| {
        let path = PathBuf::from(p);
        path.canonicalize().unwrap_or(path)
    });

    let crate_name = get_crate_name();
    let base = crate_name.replace('-', "_");

    let targets = vec![
        ("linux", "x86_64-unknown-linux-gnu", "so"),
        ("windows", "x86_64-pc-windows-gnu", "dll"),
        ("mac-intel", "x86_64-apple-darwin", "dylib"),
        ("mac-arm64", "aarch64-apple-darwin", "dylib"),
    ];

    for (name, target, ext) in &targets {
        if name.starts_with("mac") && !has_mac_compiler() {
            println!("⚠️  Skipping {name} build: osxcross (o64-clang) or zig not found");
            continue;
        }

        println!("🔧 Building for {name}...");

        let lib_basename = if *ext == "dll" {
            base.clone() // Windows: no "lib" prefix
        } else {
            format!("lib{}", base)
        };

        let mut cmd = Command::new("cargo");
        //cmd.arg(if name.starts_with("mac") { "zigbuild" } else { "build" });
        cmd.arg("zigbuild");

        let status = cmd
          .args(["--release", "--target", target])
          .stdout(Stdio::inherit())
          .stderr(Stdio::inherit())
          .status()
          .expect("failed to run cargo");

        if status.success() {
            println!("✅ Built {name}: target/{target}/release/{lib_basename}.{ext}");

            if let Some(ref out_dir) = output_dir {
                let cargo_root = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                  .parent()
                  .unwrap()
                  .to_path_buf();

                let built_path = cargo_root.join(format!(
                    "target/{target}/release/{lib_basename}.{ext}"
                ));
                let out_name = format!("{lib_basename}-{name}.{ext}");
                let out_path = out_dir.join(out_name);

                if !built_path.exists() {
                    eprintln!("❗ Built file not found: {}", built_path.display());
                }

                if let Err(e) = fs::copy(&built_path, &out_path) {
                    eprintln!("❌ Failed to copy to {}: {e}", out_path.display());
                } else {
                    println!("📦 Copied to {}", out_path.display());
                }
            }
        } else {
            println!("❌ Failed to build for {name}");
        }
    }
}


fn get_crate_name() -> String {
    let cargo_toml = std::fs::read_to_string("Cargo.toml")
      .expect("Failed to read Cargo.toml");
    for line in cargo_toml.lines() {
        if let Some(name) = line.strip_prefix("name = ") {
            return name.trim_matches('"').to_string();
        }
    }
    panic!("Could not find crate name in Cargo.toml");
}

fn has_mac_compiler() -> bool {
    if let Ok(path) = env::var("CC") {
        return path.contains("o64-clang") || path.contains("zig");
    }
    Command::new("which")
      .arg("zig")
      .output()
      .map(|o| o.status.success())
      .unwrap_or(false)
}
