//! The shared structural core — the only part parity is required on.
//! Every numeric threshold comes from the rules file, never from a literal here.

use crate::rules::Rules;

pub struct Case {
    pub id: String,
    pub mode: String,
    pub pack_type: String,
    pub note_type: String,
    pub term: String,
    pub definition: String,
    pub example: String,
    pub cloze_sent: String,
    pub extra: String,
    pub expected: String,
    pub back: String,
    pub required_terms: String,
    pub implemented: bool,
}

pub struct Verdict {
    pub fails: Vec<String>,
    pub warns: Vec<String>,
}

pub fn word_count(s: &str) -> usize {
    s.split_whitespace().count()
}

pub fn rendered(cloze: &str) -> String {
    cloze.replace("{{c1::", "").replace("}}", "")
}

pub fn run(rules: &Rules, c: &Case) -> Verdict {
    let mut fails: Vec<String> = Vec::new();
    let mut warns: Vec<String> = Vec::new();

    if let Some(max) = rules.get("core.fields.definition_max_words") {
        if word_count(&c.definition) as f64 > max {
            fails.push("core.fields.definition_max_words".into());
        }
    }
    if c.definition.contains(';') {
        fails.push("core.fields.definition_forbidden (semicolon)".into());
    }
    for tok in [
        "alternatively",
        "can also denote",
        "in older literary texts",
    ] {
        if c.definition.to_lowercase().contains(tok) {
            fails.push(format!("core.fields.definition_forbidden ({tok})"));
        }
    }
    if c.definition
        .chars()
        .filter(|c| matches!(c, '.' | '?' | '!'))
        .count()
        > 1
    {
        fails.push("core.fields.definition_single_sentence".into());
    }
    if let Some(max) = rules.get("core.fields.definition_max_clause_boundaries") {
        if c.definition.matches(',').count() as f64 > max {
            fails.push("core.fields.definition_max_clause_boundaries".into());
        }
    }
    if let Some(max) = rules.get("core.fields.example_max_words") {
        if word_count(&c.example) as f64 > max {
            fails.push("core.fields.example_max_words".into());
        }
    }
    if !c.example.to_lowercase().contains(&c.term.to_lowercase()) {
        fails.push("core.fields.example_must_contain_term".into());
    }
    if let Some(max) = rules.get("core.fields.back_max_chars") {
        if c.extra.chars().count() as f64 > max {
            fails.push("core.fields.back_max_chars".into());
        }
    }

    if c.note_type == "cloze" {
        if c.cloze_sent.matches("{{c1::").count() != 1 {
            fails.push("core.cloze.exactly_one_deletion".into());
        }
        let key = if c.pack_type == "vocab" {
            "core.cloze.max_rendered_chars_vocab"
        } else {
            "core.cloze.max_rendered_chars_rules"
        };
        if let Some(max) = rules.get(key) {
            if rendered(&c.cloze_sent).chars().count() as f64 > max {
                fails.push(key.to_string());
            }
        }
    }

    if c.definition.contains('*') || c.example.contains('*') {
        fails.push("core.markup_and_hygiene.no_markdown".into());
    }
    if c.definition
        .chars()
        .any(|c| ('\u{0660}'..='\u{0669}').contains(&c))
    {
        fails.push("core.markup_and_hygiene.western_digits_mandatory".into());
    }

    // core.type_in — the prompt must not contain the answer
    if c.note_type == "type_in" && c.definition.to_lowercase().contains(&c.term.to_lowercase()) {
        fails.push("core.type_in.must_not_self_answer".into());
    }

    // core.fields.back_max_chars — measured on the back field
    if let Some(max) = rules.get("core.fields.back_max_chars") {
        if c.back.chars().count() as f64 > max {
            fails.push("core.fields.back_max_chars".into());
        }
    }

    // institutional — runs only in curriculum mode, never in personal mode
    if c.mode == "curriculum" && !c.required_terms.is_empty() {
        for term in c.required_terms.split(',') {
            let t = term.trim().to_lowercase();
            if !t.is_empty() && !c.definition.to_lowercase().contains(&t) {
                warns.push(format!("institutional.curriculum_terms (missing: {t})"));
            }
        }
    }

    Verdict { fails, warns }
}

pub fn exit_code(rules: &Rules, v: &Verdict) -> u32 {
    if !v.fails.is_empty() {
        rules.exit_fails
    } else if !v.warns.is_empty() {
        rules.exit_warns
    } else {
        rules.exit_clean
    }
}
