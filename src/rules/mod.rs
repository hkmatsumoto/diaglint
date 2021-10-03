pub mod capitalized_messages;
pub mod illegals;
pub mod punctuated_endings;

use crate::json::Diagnostic;
use crate::lint::LintCtx;

#[derive(Default)]
pub struct RuleStore {
    pub rules: Vec<Box<dyn Rule>>,
}

impl RuleStore {
    pub fn register_rules(&mut self, rules: impl IntoIterator<Item = Box<dyn Rule>>) {
        self.rules.extend(rules);
    }

    pub fn register_default_rules(&mut self) {
        self.rules.append(&mut default_rules())
    }

    pub fn unregister_rules(&mut self, rules: &Vec<String>) {
        self.rules
            .retain(|rule| !rules.contains(&rule.name().to_owned()));
    }
}

fn default_rules() -> Vec<Box<dyn Rule>> {
    vec![
        Box::new(capitalized_messages::CapitalizedMessages {}),
        Box::new(illegals::Illegals {}),
        Box::new(punctuated_endings::PunctuatedEndings {}),
    ]
}

pub trait Rule {
    fn name(&self) -> &'static str;

    fn check(&self, diag: &Diagnostic, ctx: &mut LintCtx);
}
