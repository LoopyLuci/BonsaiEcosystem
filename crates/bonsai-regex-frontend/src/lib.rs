use async_trait::async_trait;
use bonsai_lair::{LairModule, LairFunction, LairValueType, LairValue, LairLiteral, ModuleMetadata, Effect};
use bonsai_language_frontend::{LanguageFrontend, Result};
use regex::Regex;
use std::path::Path;

pub struct RegexFrontend {
    name: String,
    extensions: Vec<String>,
    function_patterns: Vec<Regex>,
    comment_patterns: Vec<Regex>,
}

impl RegexFrontend {
    pub fn new(name: &str, extensions: Vec<String>) -> Self {
        let function_patterns = Self::get_function_patterns(name);
        let comment_patterns = Self::get_comment_patterns(name);
        Self {
            name: name.to_string(),
            extensions,
            function_patterns,
            comment_patterns,
        }
    }

    fn get_function_patterns(lang: &str) -> Vec<Regex> {
        let lang_lower = lang.to_lowercase();
        let patterns: Vec<&str> = match lang_lower.as_str() {
            // C-like languages
            "c" | "c++" | "c#" | "java" | "javascript" | "typescript" | "go" | "rust" |
            "swift" | "kotlin" | "dart" | "scala" | "zig" | "nim" | "d" => vec![
                r"(?:func|fn|fun|function|def|sub|void|int|bool|string|var|let|const|public|private|static)\s+(\w+)\s*\(",
                r"(\w+)\s*:\s*\(.*?\)\s*->",
            ],
            // Python-like
            "python" | "ruby" | "julia" | "r" | "lua" => vec![
                r"def\s+(\w+)\s*\(",
                r"async\s+def\s+(\w+)\s*\(",
                r"class\s+(\w+)",
            ],
            // Lisp-like
            "clojure" | "common lisp" | "scheme" | "racket" | "elisp" => vec![
                r"\(\s*defun\s+(\w+)",
                r"\(\s*define\s*\(\s*(\w+)",
                r"\(\s*defn\s+(\w+)",
            ],
            // ML-like
            "haskell" | "ocaml" | "f#" | "elm" | "purescript" | "gleam" | "roc" => vec![
                r"(\w+)\s*::\s*",
                r"let\s+(\w+)\s*=",
            ],
            // Shell
            "shell" | "bash" | "zsh" | "perl" | "awk" | "tcl" => vec![
                r"function\s+(\w+)\s*\{",
                r"(\w+)\s*\(\s*\)\s*\{",
            ],
            // SQL-like
            "sql" | "plsql" => vec![
                r"(?i)(?:CREATE|ALTER)\s+(?:FUNCTION|PROCEDURE)\s+(\w+)",
            ],
            // Assembly
            "assembly" | "asm" | "x86" => vec![
                r"(\w+):\s*(?:;|$)",
            ],
            // Default: any word followed by brackets or colon-equals
            _ => vec![
                r"(\w+)\s*\(",
                r"(\w+)\s*:=",
            ],
        };
        patterns.into_iter().filter_map(|p| Regex::new(p).ok()).collect()
    }

    fn get_comment_patterns(_lang: &str) -> Vec<Regex> {
        vec![
            Regex::new(r"//.*").ok(),
            Regex::new(r"#.*").ok(),
            Regex::new(r"--.*").ok(),
            Regex::new(r";.*").ok(),
            Regex::new(r"/\*.*?\*/").ok(),
        ].into_iter().flatten().collect()
    }

    fn clean_source(&self, source: &str) -> String {
        let mut cleaned = source.to_string();
        for pattern in &self.comment_patterns {
            cleaned = pattern.replace_all(&cleaned, "").to_string();
        }
        cleaned
    }
}

#[async_trait]
impl LanguageFrontend for RegexFrontend {
    fn language_name(&self) -> &str {
        &self.name
    }

    fn file_extensions(&self) -> &[&str] {
        Box::leak(self.extensions.iter().map(|s| s.as_str()).collect::<Vec<_>>().into_boxed_slice())
    }

    async fn parse(&self, source: &str, _path: &Path) -> Result<LairModule> {
        let cleaned = self.clean_source(source);
        let mut functions = Vec::new();
        let mut seen = std::collections::HashSet::new();

        for pattern in &self.function_patterns {
            for cap in pattern.captures_iter(&cleaned) {
                if let Some(name_match) = cap.get(1) {
                    let func_name = name_match.as_str().to_string();
                    if seen.insert(func_name.clone()) && func_name.len() <= 50 {
                        functions.push(LairFunction {
                            name: func_name,
                            params: vec![],
                            return_type: LairValueType::Unit,
                            body: LairValue::Literal(LairLiteral::Unit),
                            effects: vec![Effect::Io],
                            hot_reload_enabled: false,
                        });
                    }
                }
            }
        }

        Ok(LairModule {
            name: format!("{}_module", self.name.to_lowercase()),
            functions,
            types: vec![],
            constants: vec![],
            metadata: ModuleMetadata {
                imports: vec![],
                exports: vec![],
                source_language: Some(self.name.clone()),
            },
        })
    }
}
