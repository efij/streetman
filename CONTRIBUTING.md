# Contributing to streetman

Thanks for wanting to help crown streetman. Here's how to contribute without breaking the accuracy gate.

---

## The rules that don't bend

1. **100% technical accuracy is non-negotiable.** Every PR runs the 1,440-call bench matrix in CI. Accuracy regression = merge blocked. No exceptions.
2. **Every claim must cite a snapshot.** No hand-edited numbers in `README.md` or `CLAIMS.md`. Pre-commit hook enforces.
3. **No predecessor references.** Streetman is clean-room. Don't mention prior compression skills by name anywhere in the repo (bench control arms reference rules from external config only).
4. **Bench-first on new features.** New compression rules must ship with: (a) bidirectional test, (b) accuracy fixture, (c) measured savings delta.

---

## Quick setup

```bash
git clone https://github.com/yourorg/streetman.git
cd streetman
rustup default stable
cargo build --workspace
cargo test --workspace
```

**Run the bench locally** (uses your API keys):
```bash
export ANTHROPIC_API_KEY=sk-...
export OPENAI_API_KEY=sk-...
streetman bench run --trials 1 --models claude-sonnet-4-6
```

Full 1,440-call matrix ~$20 in API credits. CI runs the full matrix on every PR; local runs can use `--trials 1 --models one` for fast feedback.

---

## What to contribute

### ✅ High-value contributions

| Type | What | Required |
|---|---|---|
| **Shortcut** | Add entry to `lexicons/shortcuts.toml` | Bidirectional test + accuracy fixture |
| **Domain profile** | SQL/JSON/k8s/docs extension | Accuracy fixture + bench delta |
| **Host adapter** | New IDE / gateway / proxy | Integration test + install doc |
| **Accuracy rubric rule** | New claim extractor (e.g. JWT format) | Test fixture showing regression caught |
| **Bench prompt** | New real-world dev scenario | Accuracy trap if applicable |
| **Hook** | New host-specific automation | Persistence test |

### ⚠️ Requires design review first (open an issue)

- Changes to the skeleton engine (affects every word)
- Changes to the accuracy rubric (affects every release)
- New intensity levels beyond lite/full/ultra
- Changes to the ship gate thresholds
- Commercial-tier features (those live in a separate closed repo)

### ❌ Please don't

- Hand-edit the benchmarks table in `README.md` (auto-rendered)
- Add compression rules that sacrifice accuracy for savings
- Mention prior compression skills by name
- Commit API keys or real production prompts

---

## Workflow

1. **Open an issue first** for non-trivial changes. Quick fixes can skip this.
2. **Fork + branch** — branch name `type/short-desc` (e.g. `feat/k8s-domain-profile`).
3. **Add tests** — bidirectional + accuracy fixture if applicable.
4. **Run local bench** — `streetman bench run --trials 1 --models claude-sonnet-4-6`. Confirm no regression.
5. **Open PR** — fill out the template. Link the issue. Describe bench impact.
6. **CI runs full bench** — 1,440 calls, bootstrap CI, accuracy rubric. Takes ~30min.
7. **Merge criteria:**
   - All tests pass
   - Bench savings ≥ current committed snapshot (within 95% CI)
   - Accuracy = 100% across all 1,440 outputs
   - At least 1 maintainer approval

---

## Commit message conventions

Streetman's own `streetman-commit` skill generates these. Or write manually — Conventional Commits format:

```
feat(lexicon): add 'tbh/ngl/fr' to high-value shortcuts

Verified on 30-prompt bench: +0.3% savings, accuracy 100/100.
See results/bench-20260421.json.

Closes #42
```

Types: `feat`, `fix`, `refactor`, `perf`, `docs`, `test`, `chore`, `build`, `ci`, `style`, `revert`.

Subject ≤50 chars. Imperative mood. No "this commit does X" — the diff says what.

---

## Accuracy rubric contributions

The accuracy rubric is the most important part of streetman. Adding a new extractor:

```rust
// crates/streetman-core/src/accuracy.rs

pub fn extract_jwt_claims(text: &str) -> Vec<String> {
    // Extract every JWT structure mentioned in normal answer,
    // so rubric can verify they survive compression
}
```

Every extractor needs:
- Unit test with 5+ positive + 3+ negative cases
- Integration test: normal answer → extract → compressed answer → verify all claims present
- Regression test: historical accuracy failure this catches

---

## Reporting accuracy regressions

Found a prompt where streetman drops a technical detail? Open a bug:

**Title:** `acc regression: <short desc>`

**Body:**
```
Prompt: [full prompt text]
Model: [claude-opus-4-7 | claude-sonnet-4-6 | ...]
Intensity: [lite | full | ultra]
Normal answer: [verbatim]
Streetman answer: [verbatim]
Missing claim: [specific thing dropped]
```

Rubric contribution PR to catch it is massively appreciated.

---

## Code of conduct

See [CODE_OF_CONDUCT.md](./CODE_OF_CONDUCT.md).

## Questions?

- Bugs / features: GitHub Issues
- Discussion: GitHub Discussions
- Security: See [SECURITY.md](./SECURITY.md)
- Commercial: [BUSINESS.md](./BUSINESS.md) contact info
