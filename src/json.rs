//! These structs are copied from `compiletest::json`.

use std::path::PathBuf;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Diagnostic {
    pub message: String,
    pub code: Option<DiagnosticCode>,
    pub level: String,
    pub spans: Vec<DiagnosticSpan>,
    pub children: Vec<Diagnostic>,
    rendered: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ArtifactNotification {
    #[allow(dead_code)]
    artifact: PathBuf,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiagnosticSpan {
    file_name: String,
    line_start: usize,
    line_end: usize,
    column_start: usize,
    column_end: usize,
    is_primary: bool,
    label: Option<String>,
    suggested_replacement: Option<String>,
    expansion: Option<Box<DiagnosticSpanMacroExpansion>>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiagnosticSpanMacroExpansion {
    /// span where macro was applied to generate this code
    span: DiagnosticSpan,

    /// name of macro that was applied (e.g., "foo!" or "#[derive(Eq)]")
    macro_decl_name: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DiagnosticCode {
    /// The code itself.
    code: String,
    /// An explanation for the code.
    explanation: Option<String>,
}
