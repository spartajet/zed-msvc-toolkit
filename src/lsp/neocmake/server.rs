//! neocmakelsp server 命令构建器。

use crate::debug::log_message;
use crate::error::{ToolkitError, ToolkitResult};
use crate::lsp::neocmake::config::load_config;
use crate::lsp::neocmake::download::get_or_download_binary;
use crate::lsp::neocmake::init_options::build_init_options;
use zed_extension_api as zed;

pub const LANGUAGE_SERVER_ID: &str = "msvc-cmake-neocmake";

/// 验证 neocmake language server ID。
pub fn validate_language_server_id(id: &str) -> ToolkitResult<()> {
    if id == LANGUAGE_SERVER_ID {
        Ok(())
    } else {
        Err(ToolkitError::UnsupportedLanguageServer(id.to_string()))
    }
}

/// 构建带配置的 neocmakelsp 命令。
pub fn command_from_worktree(
    worktree: &zed::Worktree,
    language_server_id: &zed::LanguageServerId,
) -> ToolkitResult<zed::Command> {
    log_message("构建 neocmakelsp 命令");

    let binary_path = get_or_download_binary(worktree, language_server_id)?;
    log_message(&format!("neocmakelsp 二进制: {binary_path}"));

    let config = load_config(worktree);
    let init_options = build_init_options(&config);

    log_message(&format!("neocmakelsp 初始化选项: {init_options}"));

    let init_options_json = serde_json::to_string(&init_options)
        .unwrap_or_else(|_| "{}".to_string());

    Ok(zed::Command {
        command: binary_path,
        args: vec![
            "stdio".to_string(),
            format!("--init-options={init_options_json}"),
        ],
        env: Default::default(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn accepts_neocmake_language_server_id() {
        assert_eq!(validate_language_server_id("msvc-cmake-neocmake"), Ok(()));
    }

    #[test]
    fn rejects_unexpected_language_server_id() {
        let error = validate_language_server_id("other-lsp").unwrap_err();
        assert!(matches!(error, ToolkitError::UnsupportedLanguageServer(_)));
    }
}
