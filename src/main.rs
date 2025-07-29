use std::process::{Command, Stdio};
use std::env;

fn main() {
    let targets = vec![
        ("linux", "x86_64-unknown-linux-gnu", "so"),
        ("windows", "x86_64-pc-windows-gnu", "dll"),
        ("mac", "x86_64-apple-darwin", "dylib"),
    ];

    for (name, target, ext) in targets {
        if name == "mac" && !has_mac_compiler() {
            println!("⚠️  Skipping mac build: osxcross (o64-clang) or zig not found");
            continue;
        }

        println!("🔧 Building for {name}...");

        let mut cmd = Command::new("cargo");
        cmd.arg(if name == "mac" { "zigbuild" } else { "build" });

        let status = cmd
          .args(["--release", "--target", target])
          .stdout(Stdio::inherit())
          .stderr(Stdio::inherit())
          .status()
          .expect("failed to run cargo");

        if status.success() {
            println!("✅ Built {name}: target/{target}/release/libhello.{ext}");
        } else {
            println!("❌ Failed to build for {name}");
        }
    }
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
