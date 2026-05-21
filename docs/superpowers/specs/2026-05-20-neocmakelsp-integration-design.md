# neocmakelsp 集成设计文档

**日期**: 2026-05-20
**状态**: 设计阶段
**作者**: XRZB

## 1. 概述

本文档描述将 neocmakelsp（CMake LSP）集成到 zed-msvc-toolkit 扩展中的设计和实现方案。

### 1.1 目标

- 为 CMake 文件（CMakeLists.txt）提供专业的 LSP 支持
- 与现有的 clangd LSP 并存，各自处理对应的文件类型
- 提供灵活的配置选项，支持原生配置和 Zed 配置

### 1.2 范围

- 新增 `msvc-cmake-neocmake` LSP ID
- 实现 neocmakelsp 的查找和下载逻辑
- 实现配置文件读取和合并
- 修改现有架构以支持多 LSP

### 1.3 非目标

- 修改现有的 clangd LSP 行为
- 替换 CMake 语法高亮（假设已有 tree-sitter 支持）
- 实现额外的 CMake 工具集成（如任务模板）

## 2. 架构设计

### 2.1 模块结构

```
src/lsp/
├── mod.rs              # 主模块，导出公共接口
├── server.rs           # clangd 服务逻辑（现有）
├── clangd_config.rs    # clangd 配置（现有）
├── workspace_config.rs # 工作区配置（现有）
└── neocmake/           # 新增 neocmake 子模块
    ├── mod.rs          # 模块入口
    ├── server.rs       # neocmakelsp 命令构建
    ├── download.rs     # GitHub Releases 下载
    ├── config.rs       # 配置读取和合并
    └── init_options.rs # LSP 初始化选项
```

### 2.2 LSP 路由

在 `src/lib.rs` 中的 `language_server_command` 方法：

```rust
match language_server_id {
    "msvc-cpp-clangd" => lsp::server::command_from_worktree(worktree),
    "msvc-cmake-neocmake" => lsp::neocmake::server::command_from_worktree(worktree),
    _ => Err(...)
}
```

### 2.3 扩展清单配置

在 `extension.toml` 中添加：

```toml
[language_servers.msvc-cmake-neocmake]
name = "MSVC NeoCMake"
languages = ["CMake"]
```

## 3. neocmake 模块详细设计

### 3.1 server.rs - 命令构建

**职责**: 构建 neocmakelsp 的启动命令

**函数签名**:
```rust
pub fn command_from_worktree(worktree: &zed::Worktree) -> zed::Result<zed::Command>
```

**流程**:
1. 查找 neocmakelsp 二进制
2. 如果未找到，触发下载
3. 构建命令：`neocmakelsp stdio`
4. 返回 `zed::Command`

### 3.2 download.rs - 下载逻辑

**职责**: 从 GitHub Releases 下载 neocmakelsp

**常量**:
```rust
const GITHUB_REPO: &str = "neocmakelsp/neocmakelsp";
const BINARY_NAME: &str = "neocmakelsp";
```

**函数**:
```rust
pub fn download_binary() -> zed::Result<String>
```

**流程**:
1. 调用 `zed::latest_github_release(GITHUB_REPO)`
2. 解析 release assets，找到对应平台的二进制
3. 调用 `zed::download_file` 下载到扩展工作目录
4. 设置可执行权限 `zed::make_file_executable`
5. 返回二进制路径

**平台映射**:
- Windows: `neocmakelsp-x86_64-pc-windows-msvc.zip`
- Linux: `neocmakelsp-x86_64-unknown-linux-gnu`
- macOS: `neocmakelsp-x86_64-apple-darwin`

### 3.3 config.rs - 配置管理

**职责**: 读取和合并配置

**数据结构**:
```rust
pub struct NeocmakeConfig {
    pub format: FeatureConfig,
    pub lint: FeatureConfig,
    pub scan_cmake_in_package: bool,
    pub semantic_token: bool,
}

pub struct FeatureConfig {
    pub enable: bool,
}
```

**函数**:
```rust
pub fn load_config(worktree: &zed::Worktree) -> NeocmakeConfig
```

**优先级**:
1. 读取项目根目录的 `.neocmake.toml`
2. 读取工作区的 `.zed/settings.json` 中的 `lsp.msvc-cmake-neocmake` 配置
3. 合并：settings.json 覆盖 .neocmake.toml 的值
4. 使用默认值作为最终回退

**默认值**:
```rust
const DEFAULT_CONFIG: NeocmakeConfig = NeocmakeConfig {
    format: FeatureConfig { enable: true },
    lint: FeatureConfig { enable: true },
    scan_cmake_in_package: true,
    semantic_token: false,
};
```

### 3.4 init_options.rs - 初始化选项

**职责**: 将配置转换为 LSP 初始化选项

**函数**:
```rust
pub fn build_init_options(config: &NeocmakeConfig) -> serde_json::Value
```

**输出格式**:
```json
{
  "format": { "enable": true },
  "lint": { "enable": true },
  "scan_cmake_in_package": true,
  "semantic_token": false
}
```

## 4. 错误处理

### 4.1 错误类型

```rust
pub enum NeocmakeError {
    BinaryNotFound,
    DownloadFailed(String),
    ConfigParseError(String),
    InvalidBinary,
}
```

### 4.2 错误处理策略

| 场景 | 处理方式 | 用户体验 |
|------|----------|----------|
| neocmakelsp 未找到 | 记录日志，尝试下载 | 显示 "Downloading neocmakelsp..." |
| 下载失败 | 返回错误，设置 LSP 状态为 Failed | 显示具体错误原因 |
| 配置文件解析失败 | 记录警告，使用默认配置 | 静默处理，不阻止启动 |
| LSP 启动失败 | 返回错误 | 显示错误详情 |

## 5. 实现计划

### Phase 1: 基础结构
- 创建 `src/lsp/neocmake/` 模块结构
- 实现基本的命令构建逻辑（仅 PATH 查找）
- 添加 LSP ID 验证和路由

### Phase 2: 下载功能
- 实现 `download.rs`
- 添加平台检测和 asset 匹配
- 集成到命令构建流程

### Phase 3: 配置系统
- 实现 `config.rs`
- 实现配置合并逻辑
- 实现 `init_options.rs`

### Phase 4: 集成测试
- 在真实 CMake 项目中测试
- 验证配置覆盖机制
- 验证下载和安装流程

## 6. 测试策略

### 6.1 单元测试

**config.rs 测试**:
- 默认配置
- .neocmake.toml 解析
- settings.json 覆盖
- 合并优先级

**download.rs 测试**:
- 版本解析
- 平台匹配逻辑
- 使用 mock GitHub release

### 6.2 集成测试

**端到端流程**:
- PATH 查找成功
- PATH 未找到触发下载
- 配置正确传递到 LSP

## 7. 依赖和限制

### 7.1 外部依赖

- neocmakelsp 二进制（GitHub Releases）
- GitHub API（通过 `zed::latest_github_release`）
- 网络连接（用于下载）

### 7.2 平台支持

- Windows: 完全支持
- Linux: 完全支持（如果需要）
- macOS: 完全支持（如果需要）

### 7.3 性能考虑

- 下载仅在首次或找不到二进制时发生
- 配置文件读取是同步的，但文件很小，影响可忽略
- LSP 启动与 clangd 并行，无额外开销

## 8. 未来改进

- 支持本地构建的 neocmakelsp
- 支持特定版本锁定
- 提供配置 UI（如果 Zed 支持）
- 与 CMake 任务模板的更深度集成
