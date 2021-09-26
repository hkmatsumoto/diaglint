use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
};

pub(crate) struct PunctuatedEndings {}

impl super::Rule for PunctuatedEndings {
    fn name(&self) -> &'static str {
        "punctuated endings"
    }

    fn check(&self, diag: &crate::json::Diagnostic, ctx: &mut crate::lint::LintCtx) {
        if diag.message.ends_with(|c: char| c == '.') {
            let len = diag.message.len();

            let file = SimpleFile::new("<diagnostic>".to_owned(), diag.message.clone());
            let diag = Diagnostic::warning()
                .with_message("diagnostic messages should not end with punctuations")
                .with_labels(vec![
                    Label::primary((), len - 1..len).with_message("this is a punctuation")
                ]).with_notes(vec![
                    "for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>".to_owned()
                ]);

            ctx.emit(&file, diag);
        }
    }
}
