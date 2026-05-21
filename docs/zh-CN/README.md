# Zed MSVC C++ Assistant

Windows C++ CMake 项目的 MSVC 和 clangd 助手，适用于 Zed 编辑器。

## 版本 0.5.0

### 功能特性

- **V0.1**: MSVC 工具链探测 (Visual Studio 2022+, MSVC v143+, Windows SDK)
- **V0.2**: CMake `compile_commands.json` 自动探测
- **V0.3**: CMake 命令生成基础设施
- **V0.4**: `.zed/tasks.json` 自动生成，包含 CMake 任务
- **V0.5**: neocmakelsp 集成，提供 CMake 语言支持

## 文档

- **[使用说明 (docs/USAGE.md)](../USAGE.md)** - 安装、配置和使用指南
- **[测试指南 (docs/TESTING.md)](../TESTING.md)** - 单元测试和集成测试说明
- **[English Documentation](../..)** - 英文文档

## 本扩展的功能

当你在 CMake 项目中打开 C/C++ 文件时，本扩展会自动：

1. **探测 MSVC 环境** - 查找 Visual Studio 2022+、MSVC 工具链和 Windows SDK 路径
2. **生成 `.clangd` 配置** - 为 clangd 创建正确的 MSVC 包含路径
3. **探测编译数据库** - 查找 `compile_commands.json` 以进行准确的代码分析
4. **生成 CMake 任务** - 创建包含配置、构建和运行任务的 `.zed/tasks.json`
5. **启用 CMake LSP** - 下载并配置 neocmakelsp 以支持 `CMakeLists.txt`

## 快速开始

### 安装

```bash
# 编译
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --release

# 安装到 Zed
mkdir -p "$USERPROFILE/.zed/extensions/zed-msvc-toolkit"
cp target/wasm32-unknown-unknown/release/zed_msvc_toolkit.wasm "$USERPROFILE/.zed/extensions/zed-msvc-toolkit/"
cp extension.toml "$USERPROFILE/.zed/extensions/zed-msvc-toolkit/"
```

### 使用扩展

1. 在 Zed 中打开 CMake 项目（包含 `CMakeLists.txt`）
2. 打开任意 `.c` 或 `.cpp` 文件
3. 扩展会自动：
   - 使用 MSVC 包含路径配置 clangd
   - 生成包含 CMake 任务的 `.zed/tasks.json`
   - 通过 neocmakelsp 启用 CMake 语言支持

4. 通过 `Ctrl+Shift+T` 运行任务：
   - `CMake: Configure (Debug)`
   - `CMake: Build (Debug)`
   - `CMake: Build Target: <target>`
   - `CMake: Run: <target>`

### CMake 语言支持

扩展包含 [neocmakelsp](https://github.com/neocmakelsp/neocmakelsp) 用于 CMake 语言支持。

**安装：**
- 如果 `neocmakelsp` 在 `PATH` 中，会直接使用
- 否则，扩展会自动从 GitHub 下载最新版本

**配置：**

可以通过 `.zed/settings.json` 配置 neocmakelsp：

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

对于项目级配置，在项目根目录创建 `.neocmake.toml`（由 neocmakelsp 本身读取）。

## 系统要求

- Windows 10 或 11
- Visual Studio 2022+，包含"使用 C++ 的桌面开发"工作负载
- PATH 中有 clangd（来自 LLVM）
- PATH 中有 CMake（可选，用于任务）
- 包含 `CMakeLists.txt` 的 CMake 项目

## 构建目录命名

扩展使用 CLion 风格的构建目录：
- `cmake-build-debug/` 用于 Debug 构建
- `cmake-build-release/` 用于 Release 构建
- `cmake-build-relwithdebinfo/` 用于 RelWithDebInfo 构建
- `cmake-build-minsizerel/` 用于 MinSizeRel 构建

## 许可证

MIT
