# 🛠️ modbuild — Cross-platform TerraVox Mod Builder

**modbuild** is a simple CLI tool to compile your TerraVox mods for all major platforms: **Linux**, **Windows**, and **macOS**.  
It builds `.so`, `.dll`, and `.dylib` versions of your mod in one command, making mod development easy and portable.

---

## 📦 Installation

```bash
cd modbuild/
cargo build --release
```

---

## 🚀 Usage

Run this in your mod project directory (where `Cargo.toml` is):

```bash
cargo run -p modbuild
```

Or use the built binary:

```bash
./target/release/modbuild
```

---

## 📁 Mod Project Setup

Your mod must be a Rust crate with the following in `Cargo.toml`:

```toml
[lib]
crate-type = ["cdylib"]
```

---

## 📂 Example Output

```
🔧 Building for linux...
✅ Built linux: target/x86_64-unknown-linux-gnu/release/libhello.so
🔧 Building for windows...
✅ Built windows: target/x86_64-pc-windows-gnu/release/libhello.dll
🔧 Building for mac...
✅ Built mac: target/x86_64-apple-darwin/release/libhello.dylib
```

---

## 🧠 Requirements

### Linux & Windows targets

Install additional targets:

```bash
rustup target add x86_64-pc-windows-gnu
```

### macOS target (2 options):

#### Option 1: On macOS

Just install the mac target:

```bash
rustup target add x86_64-apple-darwin
```

#### Option 2: On Linux (with osxcross)

1. Install [`osxcross`](https://github.com/tpoechtrager/osxcross)
2. Export toolchain paths:

```bash
export PATH="$HOME/osxcross/target/bin:$PATH"
export CC=o64-clang
export CXX=o64-clang++
```

Or install [`cargo-zigbuild`](https://github.com/messense/cargo-zigbuild):

```bash
cargo install cargo-zigbuild
```

---

## 🎯 What `modbuild` Does

- Compiles your mod for:
  - `x86_64-unknown-linux-gnu` → `.so`
  - `x86_64-pc-windows-gnu` → `.dll`
  - `x86_64-apple-darwin` → `.dylib`
- Detects if macOS cross-compiler (`osxcross` or `zig`) is available
- Outputs clear build results

---

## 🔒 Compatibility

- Rust 1.74+
- TerraVox mods using `extern "C"` and `TerraVoxApi`

---

## 📜 License

MIT — do whatever you want, as long as you include the license.

---
