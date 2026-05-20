//! neocmakelsp LSP 集成模块。
//!
//! 通过 neocmakelsp 提供 CMake 语言支持，支持双重安装
//!（PATH + GitHub 下载）和双重配置（.neocmake.toml + settings.json）。

pub mod config;
pub mod download;
pub mod init_options;
pub mod server;

pub use server::command_from_worktree;
