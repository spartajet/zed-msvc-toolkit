use zed_extension_api as zed;

mod cmake;
mod debug;
mod environment;
mod error;
mod lsp;
mod paths;

#[derive(Default)]
struct MsvcToolkitExtension;

impl zed::Extension for MsvcToolkitExtension {
    fn new() -> Self {
        debug::log_message("extension instance created");
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let language_server_id = language_server_id.as_ref();
        let root_path = worktree.root_path();
        debug::log_message(&format!(
            "language_server_command called: id={language_server_id}, root={root_path}"
        ));

        if let Err(error) = lsp::server::validate_language_server_id(language_server_id) {
            debug::log_error("language server id validation failed", &error);
            return Err(error.user_message());
        }
        debug::log_message("language server id validation succeeded");

        if let Err(error) = lsp::server::prepare_workspace_config_from_worktree(worktree) {
            debug::log_error("workspace config preparation failed", &error);
            return Err(error.user_message());
        }
        debug::log_message("workspace config preparation succeeded");

        match lsp::server::command_from_worktree(worktree) {
            Ok(command) => {
                debug::log_message(&format!(
                    "language server command ready: command={}, args={:?}, env_count={}",
                    command.command,
                    command.args,
                    command.env.len()
                ));
                Ok(command)
            }
            Err(error) => {
                debug::log_error("language server command creation failed", &error);
                Err(error.user_message())
            }
        }
    }
}

zed::register_extension!(MsvcToolkitExtension);
