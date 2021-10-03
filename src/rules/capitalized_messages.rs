use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
};

pub(crate) struct CapitalizedMessages {}

impl super::Rule for CapitalizedMessages {
    fn name(&self) -> &'static str {
        "capitalized-messages"
    }

    fn check(&self, diag: &crate::json::Diagnostic, ctx: &mut crate::lint::LintCtx) {
        if diag.message.starts_with(|c: char| c.is_ascii_uppercase()) {
            let file = SimpleFile::new("<diagnostic>".to_owned(), diag.message.clone());
            let diag = Diagnostic::warning()
                .with_message("diagnostic messages should start with lowercase letters")
                .with_labels(vec![
                    Label::primary((), 0..1).with_message("this is an uppercase letter")
                ]).with_notes(vec![
                    "for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>".to_owned()
                ]);

            ctx.emit(&file, diag);
        }
    }
}
