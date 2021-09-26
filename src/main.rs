use diaglint::LintRunner;

fn main() {
    LintRunner::default()
        .register_default_rules()
        .run(include_str!("../assets/bad/E1017.json"))
        .iter()
        .for_each(|output| println!("{}", output));
}
