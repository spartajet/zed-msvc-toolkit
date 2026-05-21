# Zed MSVC C++ Assistant

Windows-specific MSVC toolkit for Zed. Automatically detects and configures MSVC toolchain, Windows SDK, and clangd for C/C++ CMake projects.

> **Note**: This extension is designed specifically for Windows + MSVC development. If you need CMake support on other platforms, consider the [neocmake](https://github.com/k0tran/zed_neocmake) extension.

## Version 0.5.0

### Features

- **V0.1**: MSVC toolchain detection (Visual Studio 2022+, MSVC v143+, Windows SDK)
- **V0.2**: CMake `compile_commands.json` auto-detection
- **V0.3**: CMake command generation infrastructure
- **V0.4**: `.zed/tasks.json` auto-generation with CMake tasks
- **V0.5**: neocmakelsp integration for CMake language support

## Documentation

- **[Usage Guide (docs/USAGE.md)](docs/USAGE.md)** - Installation, configuration, and usage guide
- **[Testing Guide (docs/TESTING.md)](docs/TESTING.md)** - Unit and integration testing instructions
- **[中文文档 (docs/zh-CN/)](docs/zh-CN/)** - Chinese documentation

## What This Extension Does

When you open a C/C++ file in a CMake project, this extension automatically:

1. **Detects MSVC environment** - Finds Visual Studio 2022+, MSVC toolchain, and Windows SDK paths
2. **Generates `.clangd` configuration** - Creates proper MSVC include paths for clangd
3. **Detects compile database** - Finds `compile_commands.json` for accurate code analysis
4. **Generates CMake tasks** - Creates `.zed/tasks.json` with configure, build, and run tasks
5. **Enables CMake LSP** - Downloads and configures neocmakelsp for `CMakeLists.txt` support

## Quick Start

### Installation

```bash
# Build
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

# Install to Zed
mkdir -p "$USERPROFILE/.zed/extensions/zed-msvc-toolkit"
cp target/wasm32-unknown-unknown/release/zed_msvc_toolkit.wasm "$USERPROFILE/.zed/extensions/zed-msvc-toolkit/"
cp extension.toml "$USERPROFILE/.zed/extensions/zed-msvc-toolkit/"
```

### Using the Extension

1. Open a CMake project (containing `CMakeLists.txt`) in Zed
2. Open any `.c` or `.cpp` file
3. The extension automatically:
   - Configures clangd with MSVC include paths
   - Generates `.zed/tasks.json` with CMake tasks
   - Enables CMake language support via neocmakelsp

4. Run tasks via `Ctrl+Shift+T`:
   - `CMake: Configure (Debug)`
   - `CMake: Build (Debug)`
   - `CMake: Build Target: <target>`
   - `CMake: Run: <target>`

### CMake Language Support

The extension includes [neocmakelsp](https://github.com/neocmakelsp/neocmakelsp) for CMake language support.

**Installation:**
- If `neocmakelsp` is in `PATH`, it's used directly
- Otherwise, the extension downloads the latest release from GitHub automatically

**Configuration:**

neocmakelsp can be configured via `.zed/settings.json`:

```json
{
  "lsp": {
    "msvc-cmake-neocmake": {
      "format": { "enable": false },
      "lint": { "enable": true }
    }
  }
}
```

For project-level configuration, create `.neocmake.toml` in your project root (read by neocmakelsp itself).

## Requirements

- Windows 10 or 11
- Visual Studio 2022+ with "Desktop development with C++" workload
- clangd (from LLVM) in PATH
- CMake (optional, for tasks) in PATH
- CMake project with `CMakeLists.txt`

## Build Directory Naming

The extension uses CLion-style build directories:
- `cmake-build-debug/` for Debug builds
- `cmake-build-release/` for Release builds
- `cmake-build-relwithdebinfo/` for RelWithDebInfo builds
- `cmake-build-minsizerel/` for MinSizeRel builds

## License

MIT
