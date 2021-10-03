use codespan_reporting::{diagnostic::Diagnostic, files::Files};
use serde_json;

use crate::rules::{Rule, RuleStore};
use crate::{json, report::ReportManager};

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

    pub fn unregister_rules(&'a mut self, rules: &Vec<String>) -> &'a mut LintRunner {
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
    pub diags: Vec<Diagnostic<()>>,
    pub outputs: Vec<String>,
    report_manager: ReportManager,
}

impl LintCtx {
    pub fn emit<'f>(&mut self, files: &'f impl Files<'f, FileId = ()>, diag: Diagnostic<()>) {
        let output = self.report_manager.emit(files, &diag);
        self.outputs.push(output);
    }
}
