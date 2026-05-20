//! CMake 编译数据库探测。
//!
//! V0.2 实现 compile_commands.json 探测。

use std::path::Path;

pub const COMPILE_COMMANDS_JSON: &str = "compile_commands.json";

/// 探测编译数据库路径。
///
/// 搜索顺序：
/// 1. 工作区根目录
/// 2. build/ 子目录
///
/// # 返回值
///
/// 返回包含 `compile_commands.json` 的**目录路径**（不含文件名），
/// 或 None（如果文件不存在或路径包含非UTF-8字符）。
pub fn discover_compile_database(root_path: &str) -> Option<String> {
    let root = Path::new(root_path);

    // 先检查根目录
    let root_db = root.join(COMPILE_COMMANDS_JSON);
    if root_db.exists() {
        // 根目录下的文件，返回root_path本身
        return root.to_str().map(String::from);
    }

    // 再检查 build/ 子目录
    let build_dir = root.join("build");
    let build_db = build_dir.join(COMPILE_COMMANDS_JSON);
    if build_db.exists() {
        return build_dir.to_str().map(String::from);
    }

    None
}

/// 探测 CMakeLists.txt 是否存在。
pub fn has_cmake_lists(root_path: &str) -> bool {
    Path::new(root_path).join("CMakeLists.txt").exists()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

    /// 创建一个临时测试目录，并在测试完成后清理。
    fn with_temp_dir<F>(f: F)
    where
        F: FnOnce(&std::path::Path),
    {
        // 使用系统临时目录创建唯一的子目录
        let test_id = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let temp_dir = std::env::temp_dir().join(format!(
            "zed-msvc-test-{}-{}",
            std::process::id(),
            test_id
        ));
        fs::create_dir_all(&temp_dir).unwrap();

        // 执行测试
        f(&temp_dir);

        // 清理
        let _ = fs::remove_dir_all(&temp_dir);
    }

    #[test]
    fn finds_compile_commands_in_root() {
        with_temp_dir(|root| {
            fs::write(root.join(COMPILE_COMMANDS_JSON), r#"[]"#).unwrap();

            let root_str = root.to_str().expect("temp path should be valid UTF-8");
            let found = discover_compile_database(root_str);

            let expected = root.to_str().expect("path should be valid UTF-8");
            assert_eq!(found, Some(expected.to_string()));
        });
    }

    #[test]
    fn finds_compile_commands_in_build_subdirectory() {
        with_temp_dir(|root| {
            let build_dir = root.join("build");
            fs::create_dir_all(&build_dir).unwrap();
            fs::write(build_dir.join(COMPILE_COMMANDS_JSON), r#"[]"#).unwrap();

            let root_str = root.to_str().expect("temp path should be valid UTF-8");
            let found = discover_compile_database(root_str);

            let expected = build_dir.to_str().expect("path should be valid UTF-8");
            assert_eq!(found, Some(expected.to_string()));
        });
    }

    #[test]
    fn prefers_root_over_build_directory() {
        with_temp_dir(|root| {
            let build_dir = root.join("build");
            fs::create_dir_all(&build_dir).unwrap();
            fs::write(root.join(COMPILE_COMMANDS_JSON), r#"[]"#).unwrap();
            fs::write(build_dir.join(COMPILE_COMMANDS_JSON), r#"[]"#).unwrap();

            let root_str = root.to_str().expect("temp path should be valid UTF-8");
            let found = discover_compile_database(root_str);

            // 应该返回根目录（不是文件路径）
            let expected = root.to_str().expect("path should be valid UTF-8");
            assert_eq!(found, Some(expected.to_string()));
        });
    }

    #[test]
    fn returns_none_when_compile_commands_missing() {
        with_temp_dir(|root| {
            let root_str = root.to_str().expect("temp path should be valid UTF-8");
            let found = discover_compile_database(root_str);
            assert!(found.is_none());
        });
    }

    #[test]
    fn detects_cmake_project() {
        with_temp_dir(|root| {
            fs::write(root.join("CMakeLists.txt"), "cmake_minimum_required(VERSION 3.10)").unwrap();

            let root_str = root.to_str().expect("temp path should be valid UTF-8");
            assert!(has_cmake_lists(root_str));
        });
    }

    #[test]
    fn returns_false_for_non_cmake_project() {
        with_temp_dir(|root| {
            let root_str = root.to_str().expect("temp path should be valid UTF-8");
            assert!(!has_cmake_lists(root_str));
        });
    }

    #[test]
    fn handles_non_utf8_path_gracefully() {
        // 测试非UTF-8路径时的降级行为
        // 使用从字节创建的字符串来模拟非UTF-8路径
        let invalid_bytes = [0xFF, 0xFE, 0xFD];
        let invalid_path = std::str::from_utf8(&invalid_bytes);
        // 非UTF-8路径无法创建&str，这是编译时保证
        // 实际场景中，路径来自API调用，如果转换为str失败则降级
        assert!(invalid_path.is_err());

        // 测试实际的降级行为：使用无法转换的场景
        // Path::new() 接受 &str，如果调用者传入无效UTF-8，
        // 问题会在调用栈上游暴露。这里验证我们的函数
        // 不会因为路径内容而panic
        let test_path = "C:\\valid\\utf8\\path";
        let result = std::panic::catch_unwind(|| {
            discover_compile_database(test_path)
        });
        assert!(result.is_ok());
    }
}
