# AnkiTov-CardForge

Apache-2.0. Portable card-generation and card-QA engine for spaced repetition.

This is its own repository on purpose. AnkiTov's monorepo is `AGPL-3.0-only`; a community
Apache-2.0 project inside it would mix licenses and blur AGPL §13. This repo also carries no
AnkiTov-private infrastructure hosts and no private paths — it runs on any machine with any
OpenAI-compatible endpoint.

## What lives here

| Crate | Purpose |
|---|---|
| `cardforge-core` | Contract types, blueprint builder, synthesis client, inline linter, conformance, compiler |
| `cardforge-qa` | The BAR 1 gate — the shared structural core |
| `cardforge-cli` | The `cardforge` binary: `blueprint`, `run`, `rescue`, `qa`, `package`, `studio` |
| `deck-studio-web` | Single-user local studio (BYO key), embedded SPA assets |

## The gate contract (anti-drift)

Both implementations — this repo's `cardforge-qa` and AnkiTov's Python gate — read the same rules
file: `specs/qa/bar1-rules.yaml`. Neither may hardcode a number the file declares.

- **Shared core** (parity-required): single `{{c1::…}}`, cloze length ceilings, back cap, clause
  and word caps, script purity, Western digits, non-self-answering `type_in`, per-card readability
  proxies for all 13 locales.
- **Institutional layer** (AnkiTov-only, additive): citation grounding, curriculum-term banks,
  native sign-off. Not expected to exist here.
- **Modes are fields, not branches.** `personal` runs the core only; it can never disable a
  structural check.
- **Parity is proven by fixture, not by claim.** `specs/qa/fixtures/bar1-parity.jsonl` is run
  through both gates in CI, asserting identical verdicts and identical exit codes
  (`0` clean, `1` fails, `2` warns).
- **Versioned.** `rules_version` is stamped per pack; a version bump re-runs shipped packs and
  flags newly-failing ones rather than retro-approving them.

The canonical rules file is this repo's copy. AnkiTov-C vendors a mirror at
`specs/qa/bar1-rules.yaml`, and a CI check asserts the two are byte-identical.

## Readability claims

A preset that relaxes syllable or word-length ceilings (e.g. `technical_specialist` for medicine,
law, or software engineering) means the deck carries **no** grade-7-8 claim. The badge must say so.

## Model-modified content

Cards the engine rewrites — including a normalize pass over an imported deck — are model-modified
content and must be labelled as such, on personal decks too.

## Status

Skeleton only. `cardforge-core` has the contract types (`GateMode`, `SourceType`, `NoteType`,
`Difficulty`, `ItemSpec`, `CardDraft`) and the `concept_id` / `unit` validators, dependency-free so
it builds anywhere. `cargo check` passes. Everything else is specified, not built.

Spec of record: AnkiTov-C `specs/plans/2026-11-16-cardforge-deck-studio.md`.
Work card: Buzz hive **B-0038** (gate parity harness) and **B-0037** (core scaffold).

## License

Apache-2.0. AnkiTov's own backend remains AGPL-3.0-only; this repo never depends on AnkiTov.
