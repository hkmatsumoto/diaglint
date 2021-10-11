use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

use nom::Finish;

use crate::parse::{parse_message, Element};

pub(crate) struct PunctuatedEndings {}

impl super::Rule for PunctuatedEndings {
    fn name(&self) -> &'static str {
        "punctuated-endings"
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
                    let (lo, hi) = (offset, offset + 1);

                    let snip = Snippet {
                        title: Some(Annotation {
                            id: None,
                            label: Some("diagnostic messages should not end with punctuations"),
                            annotation_type: AnnotationType::Warning,
                        }),
                        footer: vec![
                            Annotation {
                                id: None,
                                label: Some("for more information, see <https://rustc-dev-guide.rust-lang.org/diagnostics.html#diagnostic-output-style-guide>"),
                                annotation_type: AnnotationType::Note,
                            }
                        ],
                        slices: vec![Slice {
                            source: diag.message.as_str(),
                            line_start: 1,
                            origin: None,
                            fold: false,
                            annotations: vec![SourceAnnotation {
                                label: "this is a punctuation",
                                range: (lo, hi),
                                annotation_type: AnnotationType::Warning,
                            }],
                        }],
                        ..Default::default()
                    };

                    ctx.emit(snip, self.name());
                }
            }
        }
    }
}
