use crate::environment::tools::require_clangd;
use crate::error::{ToolkitError, ToolkitResult};
use zed_extension_api as zed;

pub const LANGUAGE_SERVER_ID: &str = "msvc-cpp-clangd";

pub fn clangd_args() -> Vec<String> {
    vec!["--header-insertion=never".to_string()]
}

pub fn validate_language_server_id(id: &str) -> ToolkitResult<()> {
    if id == LANGUAGE_SERVER_ID {
        Ok(())
    } else {
        Err(ToolkitError::UnsupportedLanguageServer(id.to_string()))
    }
}

pub fn build_clangd_command(command: String, env: Vec<(String, String)>) -> zed::Command {
    zed::Command {
        command,
        args: clangd_args(),
        env,
    }
}

pub fn command_from_worktree(worktree: &zed::Worktree) -> ToolkitResult<zed::Command> {
    let clangd = require_clangd(worktree.which("clangd"))?;
    Ok(build_clangd_command(clangd, worktree.shell_env()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn clangd_args_disable_header_insertion() {
        assert_eq!(clangd_args(), vec!["--header-insertion=never"]);
    }

    #[test]
    fn accepts_expected_language_server_id() {
        assert_eq!(validate_language_server_id("msvc-cpp-clangd"), Ok(()));
    }

    #[test]
    fn rejects_unexpected_language_server_id() {
        let error = validate_language_server_id("other-lsp").unwrap_err();

        assert_eq!(
            error,
            ToolkitError::UnsupportedLanguageServer("other-lsp".to_string())
        );
    }
}
