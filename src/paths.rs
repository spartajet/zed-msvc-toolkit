use std::cmp::Ordering;

pub fn compare_version_like(left: &str, right: &str) -> Ordering {
    version_parts(left).cmp(&version_parts(right))
}

pub fn highest_version_dir<'a>(dirs: impl IntoIterator<Item = &'a str>) -> Option<&'a str> {
    dirs.into_iter()
        .max_by(|left, right| compare_version_like(left, right))
}

pub fn clangd_include_arg(path: &str) -> String {
    format!("/I{}", path.replace('\\', "/"))
}

fn version_parts(value: &str) -> Vec<u32> {
    value
        .split(|ch: char| !ch.is_ascii_digit())
        .filter(|part| !part.is_empty())
        .map(|part| part.parse::<u32>().unwrap_or(0))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selects_highest_numeric_version() {
        let selected =
            highest_version_dir(["14.9.99999", "14.38.33130", "14.40.33807", "14.10.25017"]);

        assert_eq!(selected, Some("14.40.33807"));
    }

    #[test]
    fn formats_windows_include_path_for_clangd() {
        let arg = clangd_include_arg(
            r"C:\Program Files\Microsoft Visual Studio\2022\Community\VC\Tools\MSVC\14.40.33807\include",
        );

        assert_eq!(
            arg,
            "/IC:/Program Files/Microsoft Visual Studio/2022/Community/VC/Tools/MSVC/14.40.33807/include"
        );
    }
}
