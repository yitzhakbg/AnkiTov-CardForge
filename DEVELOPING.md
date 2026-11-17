# Developer notes — cardforge-qa

## What this crate is
The shared structural core of the BAR 1 gate. It reads its thresholds from
`specs/qa/bar1-rules.yaml` and must never hardcode a number the rules file declares.

## Rules reader (`src/rules.rs`)
- Indent-path reader, no external YAML dependency. Paths look like
  `core.cloze.max_rendered_chars_vocab`.
- Only numeric values are stored; booleans and strings in the rules file are descriptive.
- **Inline comments are stripped before parsing.** A line like
  `back_max_chars: 400   # strict in every mode` previously failed to parse, so the back cap
  silently stopped firing. That is the exact failure mode this reader must guard against:
  a rule that is present in the file but absent in behaviour.
- Exit codes come from the file (`exit_codes.clean/fails/warns`), not from literals.

## Checks (`src/checks.rs`)
Implemented: definition word cap, single sentence, clause boundary, forbidden sense-chains,
example cap + must-contain-term, back cap, exactly one `{{c1::…}}`, rendered cloze length by
pack type, no markdown, Western digits, `type_in` self-answer, and the institutional
required-terms warn (curriculum mode only).

Not implemented: zh HSK share and ja MEXT kyōiku kanji grade check — both need per-locale data
tables that do not exist in the repo. Fixture cases F10 and F11 are marked `implemented:false`
and are skipped, never assumed.

## Fixture corpus
`specs/qa/fixtures/bar1-parity.jsonl`, one JSON object per case. Expectations must be pinned by
computation (measured lengths), not by assumption — two cases were corrected this way.
Run: `cargo run -q -p cardforge-qa` then `cargo test -p cardforge-qa`.

## Parity with AnkiTov's Python gate
The Python gate must read the same rules file. The parity runner (card B-0039) executes the
corpus through both gates and fails on any disagreement. The Python gate keeps its monorepo
paths (institutional layer); this crate takes paths as arguments and must never carry `.goose/`.
