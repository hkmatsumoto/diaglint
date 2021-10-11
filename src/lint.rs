use annotate_snippets::{
    display_list::DisplayList,
    snippet::{Annotation, AnnotationType, Snippet},
};
use serde_json;

use crate::json;
use crate::rules::{Rule, RuleStore};

#[derive(Default)]
pub struct LintRunner {
    rule_store: RuleStore,
}

impl<'a> LintRunner {
    pub fn register_default_rules(&'a mut self) -> &'a mut LintRunner {
        self.rule_store.register_default_rules();
        self
    }

    pub fn register_rules(
        &'a mut self,
        rules: impl IntoIterator<Item = Box<dyn Rule>>,
    ) -> &'a mut LintRunner {
        self.rule_store.register_rules(rules);
        self
    }

    pub fn unregister_rules(&'a mut self, rules: &[String]) -> &'a mut LintRunner {
        self.rule_store.unregister_rules(rules);
        self
    }

    pub fn run(&self, diag: &'_ str) -> Vec<String> {
        let mut ctx = LintCtx::default();

        let diag = serde_json::from_str::<json::Diagnostic>(diag).unwrap();
        let mut diags = vec![diag];
        while let Some(mut diag) = diags.pop() {
            diags.append(&mut diag.children);

            self.rule_store
                .rules
                .iter()
                .for_each(|rule| rule.check(&diag, &mut ctx));
        }

        ctx.outputs
    }
}

#[derive(Debug, Default)]
pub struct LintCtx {
    pub outputs: Vec<String>,
}

impl LintCtx {
    pub fn emit(&mut self, snip: Snippet<'_>, lint_name: &str) {
        let mut snip = snip;
        let label = format!(
            "to disable this lint, add `ignored-diaglints: {}` to top of the test file",
            lint_name
        );
        snip.footer.push(Annotation {
            id: None,
            label: Some(&label),
            annotation_type: AnnotationType::Help,
        });

        let dl = DisplayList::from(snip);
        self.outputs.push(format!("{}", dl));
    }
}
