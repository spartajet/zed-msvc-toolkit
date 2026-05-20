//! Debug adapter 集成模块边界。
//!
//! V0.1 不注册或启动 vsdbg。

#[cfg(target_arch = "wasm32")]
const LOG_FILE_NAME: &str = "zed-msvc-toolkit.log";

#[cfg(target_arch = "wasm32")]
fn powershell_quote(value: &str) -> String {
    format!("'{}'", value.replace('\'', "''"))
}

/// 追加一行诊断日志到 `%TEMP%\zed-msvc-toolkit.log`。
///
/// Zed extension API 目前没有稳定的原生日志入口；这里用已声明 capability
/// 的 PowerShell 写入临时目录。日志失败时必须静默，避免影响语言服务器启动。
pub fn log_message(message: &str) {
    #[cfg(not(target_arch = "wasm32"))]
    {
        let _ = message;
    }

    #[cfg(target_arch = "wasm32")]
    {
        use zed_extension_api as zed;

        let escaped_file_name = powershell_quote(LOG_FILE_NAME);
        let escaped_message = powershell_quote(message);
        let script = format!(
            "$ErrorActionPreference='SilentlyContinue'; \
         $path = Join-Path $env:TEMP {escaped_file_name}; \
         $timestamp = (Get-Date).ToString('o'); \
         Add-Content -LiteralPath $path -Encoding UTF8 -Value (\"[$timestamp] \" + {escaped_message})"
        );

        let mut command = zed::Command {
            command: "powershell".to_string(),
            args: vec![
                "-NoProfile".to_string(),
                "-ExecutionPolicy".to_string(),
                "Bypass".to_string(),
                "-Command".to_string(),
                script,
            ],
            env: Vec::new(),
        };

        let _ = command.output();
    }
}

pub fn log_error(context: &str, error: &crate::error::ToolkitError) {
    log_message(&format!("{context}: {}", error.user_message()));
}
