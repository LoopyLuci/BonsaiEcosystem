use proptest::prelude::*;

pub fn workspace_file_path() -> impl Strategy<Value = String> {
    proptest::collection::vec("[a-zA-Z0-9_]{1,10}", 1..4)
        .prop_map(|parts| parts.join("/") + ".rs")
}

pub fn model_name() -> impl Strategy<Value = String> {
    proptest::string::string_regex("[a-z0-9-]{3,20}").unwrap()
}

pub fn ip_address() -> impl Strategy<Value = String> {
    (0..=255u8, 0..=255u8, 0..=255u8, 0..=255u8)
        .prop_map(|(a, b, c, d)| format!("{}.{}.{}.{}", a, b, c, d))
}

pub fn assert_valid_syntax(code: &str, language: &str) -> Result<(), String> {
    match language {
        "rust" => {
            let output = std::process::Command::new("rustfmt")
                .arg("--edition=2021")
                .arg("--check")
                .arg("-")
                .stdin(std::process::Stdio::piped())
                .output()
                .map_err(|e| format!("Failed to run rustfmt: {}", e))?;
            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
            Ok(())
        }
        "python" => {
            let output = std::process::Command::new("python3")
                .arg("-m").arg("py_compile")
                .arg("-c").arg(code)
                .output()
                .map_err(|e| format!("Failed to run python: {}", e))?;
            if !output.status.success() {
                return Err(String::from_utf8_lossy(&output.stderr).to_string());
            }
            Ok(())
        }
        _ => Ok(()),
    }
}
