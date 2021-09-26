use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
};

pub(crate) struct Illegals {}

impl super::Rule for Illegals {
    fn name(&self) -> &'static str {
        "illegals"
    }

    fn check(&self, diag: &crate::json::Diagnostic, ctx: &mut crate::lint::LintCtx) {
        if let Some(idx) = diag.message.find("illegal") {
            let file = SimpleFile::new("<diagnostic>".to_owned(), diag.message.clone());
            let diag = Diagnostic::warning()
            .with_message("the word `illegal` is illegal")
            .with_labels(vec![
                Label::primary((), idx..idx+"illegal".len()).with_message("consider using more specific word, like `invalid`")
            ]).with_notes(vec![
                "for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>".to_owned()
            ]);

            ctx.emit(&file, diag);
        }
    }
}
