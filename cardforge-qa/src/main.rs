use cardforge_qa::checks;
use cardforge_qa::rules;

fn field(line: &str, key: &str) -> String {
    let pat = format!("\"{key}\":");
    let i = match line.find(&pat) {
        Some(i) => i + pat.len(),
        None => return String::new(),
    };
    let rest = line[i..].trim_start();
    if rest.starts_with("null") {
        return String::new();
    }
    if rest.starts_with('"') {
        let end = rest[1..].find('"').map(|j| j + 1).unwrap_or(rest.len());
        rest[1..end].to_string()
    } else {
        rest.split(',').next().unwrap_or("").trim().to_string()
    }
}

fn main() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("repo root");
    let rules_path = root.join("specs/qa/bar1-rules.yaml");
    let fixture_path = root.join("specs/qa/fixtures/bar1-parity.jsonl");
    let rules = rules::load(rules_path.to_str().unwrap());

    println!(
        "rules_version {} (exit codes {}/{}/{})",
        rules.version, rules.exit_clean, rules.exit_fails, rules.exit_warns
    );

    let text = std::fs::read_to_string(&fixture_path).expect("fixture corpus");
    let mut checked = 0usize;
    let mut pending = 0usize;
    let mut mismatches = 0usize;

    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        let c = checks::Case {
            id: field(line, "case_id"),
            mode: field(line, "mode"),
            pack_type: field(line, "pack_type"),
            note_type: field(line, "note_type"),
            term: field(line, "term"),
            definition: field(line, "definition"),
            example: field(line, "example"),
            cloze_sent: field(line, "cloze_sent"),
            extra: field(line, "extra"),
            expected: field(line, "expected_verdict"),
            implemented: line.contains("\"implemented\":true"),
        };
        if !c.implemented {
            pending += 1;
            continue;
        }
        let v = checks::run(&rules, &c);
        let got = if !v.fails.is_empty() {
            "fail"
        } else if !v.warns.is_empty() {
            "warn"
        } else {
            "pass"
        };
        checked += 1;
        if got != c.expected {
            mismatches += 1;
            println!(
                "MISMATCH {} expected {} got {} ({})",
                c.id,
                c.expected,
                got,
                v.fails.join(", ")
            );
        } else {
            println!(
                "ok {} {} exit {} {}",
                c.id,
                got,
                checks::exit_code(&rules, &v),
                v.fails.join(", ")
            );
        }
    }

    println!("checked {checked}  pending {pending}  mismatches {mismatches}");
    if mismatches > 0 {
        std::process::exit(1);
    }
}
