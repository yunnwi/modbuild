# 🛠️ modbuild - Cross-platform Mod Builder for Freven

**modbuild** is the official tool used by [Freven](https://discord.gg/zKY3Tkk837) to compile mods for **Linux**, **Windows**, and **macOS**.  
It turns your Rust-based mod into shared libraries (`.so`, `.dll`, `.dylib`) with just one command - no manual setup or cross-compilation headaches.

---

## 💡 What is this?

Freven uses dynamic `.so`/`.dll`/`.dylib` files to load mods at runtime.  
This tool builds those files for all platforms at once - so your mod works everywhere without needing a Mac or Windows machine.

---

## 📦 Installation

Build it once:

```bash
cd modbuild/
cargo build --release
```

---

## 🚀 Usage

Inside your mod crate (where `Cargo.toml` is), run:

```bash
cargo run -p modbuild
```

Or use the compiled binary:

```bash
./target/release/modbuild
```

You'll see a clean report showing `.so`, `.dll`, and `.dylib` outputs.

---

## 📁 How to Set Up Your Mod

In your `Cargo.toml`, make sure this is set:

```toml
[lib]
crate-type = ["cdylib"]
```

This makes Rust compile your mod as a dynamic library.

---

## ✅ Example Output

```text
🔧 Building for linux...
✅ Built linux: target/x86_64-unknown-linux-gnu/release/libhello.so
🔧 Building for windows...
✅ Built windows: target/x86_64-pc-windows-gnu/release/libhello.dll
🔧 Building for mac...
✅ Built mac: target/x86_64-apple-darwin/release/libhello.dylib
```

---

## 🧠 Requirements

### Linux & Windows builds

Install the Windows target:

```bash
rustup target add x86_64-pc-windows-gnu
```

### macOS builds (2 options)

#### Option 1: Build on macOS

```bash
rustup target add x86_64-apple-darwin
```

#### Option 2: Build on Linux (with osxcross or zig)

Use [osxcross](https://github.com/tpoechtrager/osxcross) and set:

```bash
export PATH="$HOME/osxcross/target/bin:$PATH"
export CC=o64-clang
export CXX=o64-clang++
```

Or install [cargo-zigbuild](https://github.com/messense/cargo-zigbuild):

```bash
cargo install cargo-zigbuild
```

`modbuild` will detect this automatically.

---

## ⚙️ How It Works

- Calls `cargo build` or `cargo zigbuild` for each target
- Auto-detects macOS cross-compilation tools
- Outputs shared libraries to:
  - `target/x86_64-unknown-linux-gnu/release/lib*.so`
  - `target/x86_64-pc-windows-gnu/release/lib*.dll`
  - `target/x86_64-apple-darwin/release/lib*.dylib`

---

## 🧩 Compatibility

- Rust 1.74+
- Freven mods using `extern "C"` and the `FrevenApi`
- Works on Linux/macOS (Windows coming soon)

---

## 📜 License

MIT - use freely, modify freely, just include the license.

---