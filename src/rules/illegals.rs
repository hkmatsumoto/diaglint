use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};
use nom::Finish;

use crate::parse::{parse_message, Element};

pub(crate) struct Illegals {}

impl super::Rule for Illegals {
    fn name(&self) -> &'static str {
        "illegals"
    }

    fn check(&self, diag: &crate::json::Diagnostic, ctx: &mut crate::lint::LintCtx) {
        let sentences = parse_message(&diag.message).finish().unwrap().1;

        for elem in sentences.into_iter().flatten() {
            if let Element::Normal(n) = elem.inner() {
                if let Some(idx) = n.find("illegal") {
                    let offset = elem.span().location_offset() + idx;
                    let (lo, hi) = (offset, offset + "illegal".len());

                    let snip = Snippet {
                        title: Some(Annotation {
                            label: Some("the word `illegal` is illegal"),
                            id: None,
                            annotation_type: AnnotationType::Warning,
                        }),
                        footer: vec![],
                        slices: vec![Slice {
                            source: diag.message.as_str(),
                            line_start: 1,
                            fold: false,
                            origin: None,
                            annotations: vec![SourceAnnotation {
                                label: "consider using more specific word, like `invalid`",
                                range: (lo, hi),
                                annotation_type: AnnotationType::Warning,
                            }],
                        }],
                        ..Default::default()
                    };

                    ctx.emit(snip);
                }
            }
        }
    }
}
