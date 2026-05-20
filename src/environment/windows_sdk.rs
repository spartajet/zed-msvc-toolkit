use crate::environment::tools::{CommandRunner, powershell_list_directory_names};
use crate::paths::highest_version_dir;

const SDK_INCLUDE_KINDS: [&str; 3] = ["ucrt", "um", "shared"];

pub fn select_windows_sdk_includes<'a>(
    versions: impl IntoIterator<Item = &'a str>,
    kits_include_root: &str,
) -> Vec<String> {
    let Some(version) = highest_version_dir(versions) else {
        return Vec::new();
    };

    SDK_INCLUDE_KINDS
        .iter()
        .map(|kind| {
            format!(
                r"{root}\{version}\{kind}",
                root = kits_include_root.trim_end_matches('\\'),
                version = version,
                kind = kind
            )
        })
        .collect()
}

pub fn discover_windows_sdk_includes(runner: &impl CommandRunner) -> Vec<String> {
    let kits_include_root = r"C:\Program Files (x86)\Windows Kits\10\Include";
    crate::debug::log_message(&format!(
        "discovering Windows SDK includes under: {kits_include_root}"
    ));
    match powershell_list_directory_names(runner, kits_include_root) {
        Ok(versions) => {
            crate::debug::log_message(&format!("Windows SDK versions found: {versions:?}"));
            let Some(version) = highest_version_dir(versions.iter().map(String::as_str)) else {
                crate::debug::log_message("no Windows SDK version directory selected");
                return Vec::new();
            };
            let version_root = format!(r"{kits_include_root}\{version}");
            let Ok(children) = powershell_list_directory_names(runner, &version_root) else {
                crate::debug::log_message(&format!(
                    "failed to list selected Windows SDK directory: {version_root}"
                ));
                return Vec::new();
            };
            crate::debug::log_message(&format!(
                "Windows SDK include children for {version}: {children:?}"
            ));
            if SDK_INCLUDE_KINDS.iter().all(|kind| {
                children
                    .iter()
                    .any(|child| child.eq_ignore_ascii_case(kind))
            }) {
                let includes = select_windows_sdk_includes([version], kits_include_root);
                crate::debug::log_message(&format!("Windows SDK includes selected: {includes:?}"));
                includes
            } else {
                crate::debug::log_message(
                    "selected Windows SDK directory is missing ucrt/um/shared include folders",
                );
                Vec::new()
            }
        }
        Err(error) => {
            crate::debug::log_error("Windows SDK directory listing failed", &error);
            Vec::new()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::environment::tools::{CommandOutput, CommandRunner};
    use crate::error::ToolkitResult;
    use std::cell::RefCell;
    use std::collections::VecDeque;

    #[test]
    fn selects_highest_sdk_include_group() {
        let includes = select_windows_sdk_includes(
            ["10.0.19041.0", "10.0.22621.0"],
            r"C:\Program Files (x86)\Windows Kits\10\Include",
        );

        assert_eq!(
            includes,
            vec![
                r"C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\ucrt",
                r"C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\um",
                r"C:\Program Files (x86)\Windows Kits\10\Include\10.0.22621.0\shared",
            ]
        );
    }

    #[test]
    fn returns_empty_includes_when_sdk_versions_are_missing() {
        let includes =
            select_windows_sdk_includes([], r"C:\Program Files (x86)\Windows Kits\10\Include");

        assert!(includes.is_empty());
    }

    struct QueueRunner {
        outputs: RefCell<VecDeque<CommandOutput>>,
    }

    impl QueueRunner {
        fn new(outputs: impl IntoIterator<Item = CommandOutput>) -> Self {
            Self {
                outputs: RefCell::new(outputs.into_iter().collect()),
            }
        }
    }

    impl CommandRunner for QueueRunner {
        fn run_command(&self, _command: &str, _args: &[String]) -> ToolkitResult<CommandOutput> {
            self.outputs.borrow_mut().pop_front().ok_or_else(|| {
                crate::error::ToolkitError::IoMessage("unexpected command".to_string())
            })
        }
    }

    #[test]
    fn returns_empty_includes_when_selected_sdk_version_is_incomplete() {
        let runner = QueueRunner::new([
            CommandOutput {
                status: Some(0),
                stdout: "10.0.22621.0\n".to_string(),
                stderr: String::new(),
            },
            CommandOutput {
                status: Some(0),
                stdout: "ucrt\nshared\n".to_string(),
                stderr: String::new(),
            },
        ]);

        let includes = discover_windows_sdk_includes(&runner);

        assert!(includes.is_empty());
    }
}
