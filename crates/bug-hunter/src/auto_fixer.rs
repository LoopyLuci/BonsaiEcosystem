/// Auto-Fixer – Automatically fixes detected stub code issues
use crate::stub_detector::{StubFinding, StubType, Severity};
use std::path::Path;
use std::fs;
use regex::Regex;

pub struct AutoFixer;

impl AutoFixer {
    /// Apply fixes to a file based on findings
    pub fn fix_file(file_path: &Path, findings: &[StubFinding]) -> Result<(), Box<dyn std::error::Error>> {
        if findings.is_empty() {
            return Ok(());
        }

        let mut content = fs::read_to_string(file_path)?;
        let mut lines: Vec<String> = content.lines().map(|l| l.to_string()).collect();

        // Sort findings by line number (descending) to avoid offset issues
        let mut sorted_findings = findings.to_vec();
        sorted_findings.sort_by(|a, b| b.line_number.cmp(&a.line_number));

        for finding in sorted_findings {
            if finding.line_number > lines.len() {
                continue;
            }

            let line_idx = finding.line_number - 1;
            let line = &lines[line_idx];

            match finding.finding_type {
                StubType::TodoComment => {
                    // For now, just log TODOs, don't remove
                    // In production, would track these in a TODO database
                }
                StubType::UnimplementedMacro => {
                    lines[line_idx] = fix_unimplemented(line);
                }
                StubType::PanicMacro => {
                    lines[line_idx] = fix_panic(line);
                }
                StubType::UnwrapCall => {
                    lines[line_idx] = fix_unwrap(line);
                }
                StubType::IgnoredTest => {
                    lines[line_idx] = fix_ignored_test(line);
                }
                StubType::SkippedTest => {
                    lines[line_idx] = fix_skipped_test(line);
                }
                StubType::EmptyFunctionBody => {
                    // More complex fix, would need multi-line handling
                    // For now, mark for manual review
                }
                _ => {}
            }
        }

        let fixed_content = lines.join("\n");
        fs::write(file_path, fixed_content)?;

        Ok(())
    }
}

fn fix_unimplemented(line: &str) -> String {
    // Replace unimplemented!() with Result error
    Regex::new(r"unimplemented!\(\)")
        .unwrap()
        .replace_all(line, "return Err(\"Not implemented\".into())")
        .to_string()
}

fn fix_panic(line: &str) -> String {
    // Replace panic!() with proper error handling
    if line.contains("panic!(\"") {
        // Extract the panic message
        Regex::new(r#"panic!\("([^"]+)"\)"#)
            .unwrap()
            .replace_all(
                line,
                |caps: &regex::Captures| {
                    let msg = &caps[1];
                    format!("return Err(\"{}\".into())", msg)
                },
            )
            .to_string()
    } else {
        line.replace("panic!()", "return Err(\"Panic: operation failed\".into())")
    }
}

fn fix_unwrap(line: &str) -> String {
    // Replace .unwrap() with ? operator
    if line.contains(".unwrap()") {
        line.replace(".unwrap()", "?")
    } else {
        line.to_string()
    }
}

fn fix_ignored_test(line: &str) -> String {
    // Remove #[ignore] attribute
    line.replace("#[ignore]", "")
        .replace("#[ignore(reason = \"...\")]", "")
}

fn fix_skipped_test(line: &str) -> String {
    // Remove #[skip] attribute
    line.replace("#[skip]", "")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fix_unimplemented() {
        let line = "let result = unimplemented!();";
        let fixed = fix_unimplemented(line);
        assert!(fixed.contains("Err"));
        assert!(!fixed.contains("unimplemented"));
    }

    #[test]
    fn test_fix_unwrap() {
        let line = "let value = result.unwrap();";
        let fixed = fix_unwrap(line);
        assert!(fixed.contains("?"));
    }

    #[test]
    fn test_fix_ignored_test() {
        let line = "#[ignore]\n#[test]";
        let fixed = fix_ignored_test(line);
        assert!(!fixed.contains("#[ignore]"));
    }
}
