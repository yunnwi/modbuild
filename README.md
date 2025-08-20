# 🛠️ modbuild - Cross-platform Mod Builder for Freven

**modbuild** is a tool used by [Freven](https://discord.gg/zKY3Tkk837) to compile Rust-based mods for **Linux**, **Windows**, and **macOS**.  
It produces shared libraries (`.so`, `.dll`, `.dylib`) for multiple platforms from a single command.

---

## 💡 Installation

Build once:

```bash
cd modbuild/
cargo build --release
```

Or install globally:

```bash
cargo install --path .
```

---

## 🚀 Usage

Build all targets for your mod:

```bash
./target/release/modbuild build --path /path/to/your/mod --out ./dist
```

Specify targets explicitly:

```bash
./target/release/modbuild build --path /path/to/your/mod --out ./dist --targets linux,windows-gnu,windows-msvc,mac-intel,mac-arm64
```

List all supported targets:

```bash
./target/release/modbuild list-targets
```

---

## 📁 Setting Up Your Mod

In your `Cargo.toml`, configure your library as a dynamic library:

```toml
[lib]
crate-type = ["cdylib"]
```

This allows Rust to produce `.so`, `.dll`, or `.dylib` files.

---

## ✅ Example Output

```bash
Building for linux...
Built linux successfully.
Copied to ./dist/libmy_mod-linux.so
Building for windows-gnu...
Built windows-gnu successfully.
Copied to ./dist/my_mod-windows-gnu.dll
Building for windows-msvc...
Built windows-msvc successfully.
Copied to ./dist/my_mod-windows-msvc.dll
Building for mac-intel...
Built mac-intel successfully.
Copied to ./dist/libmy_mod-mac-intel.dylib
Building for mac-arm64...
Built mac-arm64 successfully.
Copied to ./dist/libmy_mod-mac-arm64.dylib
```

---

## 🧠 Requirements

### Linux & Windows

Install the Windows targets:

```bash
rustup target add x86_64-pc-windows-gnu
rustup target add x86_64-pc-windows-msvc
```

### macOS builds

#### Option 1: Build on macOS

```bash
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
```

#### Option 2: Cross-build on Linux

Install [cargo-zigbuild](https://github.com/messense/cargo-zigbuild) or [osxcross](https://github.com/tpoechtrager/osxcross):

```bash
cargo install cargo-zigbuild
export CC=o64-clang
export CXX=o64-clang++
```

`modbuild` will detect the cross-compilation tools automatically.

---

## ⚙️ How It Works

- Uses `cargo build` or `cargo zigbuild` per target
- Detects macOS cross-compilation automatically
- Outputs shared libraries to the `--out` directory, named like:
  - `lib<crate>-linux.so`
  - `<crate>-windows-gnu.dll`
  - `<crate>-windows-msvc.dll`
  - `lib<crate>-mac-intel.dylib`
  - `lib<crate>-mac-arm64.dylib`

---

## 🧩 Compatibility

- Rust 1.74+
- Freven mods using `extern "C"` and `FrevenApi`
- Works on Linux, Windows, and macOS

---

## 📜 License

MIT - use freely, modify freely, include the license.