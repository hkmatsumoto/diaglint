use annotate_snippets::snippet::{Annotation, AnnotationType, Slice, Snippet, SourceAnnotation};

pub(crate) struct CapitalizedMessages {}

impl super::Rule for CapitalizedMessages {
    fn name(&self) -> &'static str {
        "capitalized-messages"
    }

    fn check(&self, diag: &crate::json::Diagnostic, ctx: &mut crate::lint::LintCtx) {
        if diag.message.starts_with(|c: char| c.is_ascii_uppercase()) {
            let snip = Snippet {
                title: Some(Annotation {
                    label: Some("diagnostic messages should start with lowercase letters"),
                    id: None,
                    annotation_type: AnnotationType::Warning,
                }),
                footer: vec![],
                slices: vec![Slice {
                    source: diag.message.as_str(),
                    line_start: 1,
                    origin: None,
                    fold: false,
                    annotations: vec![SourceAnnotation {
                        label: "this is an uppercase letter",
                        range: (0, 1),
                        annotation_type: AnnotationType::Warning,
                    }],
                }],
                ..Default::default()
            };

            ctx.emit(snip);
        }
    }
}
