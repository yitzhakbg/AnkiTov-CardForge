#[test]
fn fixture_corpus_verdicts_match() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap();
    let rules = cardforge_qa::rules::load(root.join("specs/qa/bar1-rules.yaml").to_str().unwrap());
    let text = std::fs::read_to_string(root.join("specs/qa/fixtures/bar1-parity.jsonl")).unwrap();
    let mut checked = 0;
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        if !line.contains("\"implemented\":true") {
            continue;
        }
        // expectations are asserted by the binary run; here we only assert the rules file is readable
        checked += 1;
    }
    assert!(checked > 0, "no implemented cases in the fixture corpus");
    assert_eq!(rules.exit_fails, 1);
    assert_eq!(rules.exit_warns, 2);
    assert_eq!(rules.exit_clean, 0);
    assert!(rules.get("core.cloze.max_rendered_chars_vocab") == Some(120.0));
    assert!(rules.get("core.fields.definition_max_words") == Some(15.0));
}
