//! CMake 集成模块。
//!
//! V0.3 实现 CMake configure/build 命令支持。
//! V0.4 实现 .zed/tasks.json 生成。
//! V0.5 实现 neocmakelsp CMake LSP 集成。

pub mod compile_db;
pub mod tasks;
pub mod tools;

pub use compile_db::discover_compile_database;
#[allow(dead_code)]
pub use compile_db::has_cmake_lists;
pub use tasks::{CmakeTarget, TaskOptions, generate_tasks_json};
// 预留功能：CMake configure/build 命令（待未来实现）
#[allow(dead_code)]
pub use tools::{
    CmakeBuildType, CmakeConfigureOptions, CmakeGenerator, build_build_command,
    build_configure_command, discover_cmake, select_generator,
};
