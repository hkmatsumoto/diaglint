use codespan_reporting::{
    diagnostic::{Diagnostic, Label},
    files::SimpleFile,
};
use nom::Finish;

use crate::parse::{Element, parse_message};

pub(crate) struct PunctuatedEndings {}

impl super::Rule for PunctuatedEndings {
    fn name(&self) -> &'static str {
        "punctuated endings"
    }

    fn check(&self, diag: &crate::json::Diagnostic, ctx: &mut crate::lint::LintCtx) {
        let sentences = parse_message(&diag.message).finish().unwrap().1;
        if sentences.len() > 1 {
            return;
        }

        if let Some(sentence) = sentences.first() {
            if let Some(last) = sentence.last() {
                if matches!(last.inner(), Element::Period) {
                    let offset = last.span().location_offset();

                    let file = SimpleFile::new("<diagnostic>".to_owned(), diag.message.clone());
                    let diag = Diagnostic::warning()
                        .with_message("diagnostic messages should not end with punctuations")
                        .with_labels(vec![
                            Label::primary((), offset..offset+1).with_message("this is a punctuation")
                        ])
                        .with_notes(
                            vec![
                                "for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>".to_owned()
                            ]
                        );
                    ctx.emit(&file, diag);
                }
            }
        }
    }
}
