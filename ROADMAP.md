# streetman — OSS Roadmap

> Semver-based versioned scope for the **streetman OSS project only**.
> Commercial tier (DevBooster) is NOT on this roadmap — see [MASTER.md](./MASTER.md).

**Scope rule:** Streetman OSS is the **output-compression engine + bench harness + host plugins + gateway adapters**. Nothing else. If a feature belongs in DevBooster (commercial), it's marked ❌ below and stays out of this repo.

---

## 🎯 Caveman kill matrix — every caveman weakness addressed ≤ v1.0

The crown changes hands at v1.0. By week 4, streetman wins on every axis simultaneously. No caveman weakness survives past v1.0.

| # | Caveman weakness | Caveman state | Streetman fix | Version |
|---|---|---|---|---|
| 1 | Overstated token savings (claims 65–75%, actual ~50%) | unmeasured, no CI | Committed 1,440-call bench, pre-commit claim audit, auto-rendered README | **v0.1** |
| 2 | No accuracy oracle (no fidelity rubric) | aspirational only | Hard 100% rubric (regex + LLM-judge) CI-enforced, blocks merges | **v0.1** |
| 3 | Article/filler-only compression (lookup table) | fixed lexicon | **UNBOUNDED algorithmic consonant-skeleton engine** — every word reduces via rules | **v0.2** |
| 4 | Ambiguous emoji layer (🚀 = rocket/launch/deploy/ship) | all-or-nothing | 3-rule safe: define-on-first-use + single-meaning whitelist + domain-gated | **v0.3** |
| 5 | Code comments never compressed | ignored | Code-comment compressor: prose → skeleton, code byte-exact (+5–10% on code-dense files) | **v0.3** |
| 6 | Destructive/security content treated like prose | vague auto-clarity rule | Hard auto-fallback on CVEs, `rm -rf`, `DROP TABLE`, security warnings → normal mode | **v0.3** |
| 7 | Breaks markdown rendering | no post-check | Post-compression AST validator; rejects outputs that break renders | **v0.3** |
| 8 | No domain profiles — SQL keywords / JSON keys mangled | one-size-fits-all | `streetman-sql`, `-json`, `-k8s`, `-docs` domain profiles | **v0.3** |
| 9 | Compress skill silently fails (issue #237, Windows) | zero-byte backup, "passed" validation | Preview-gate: diff + token delta + accuracy score + y/N before overwrite | **v0.5** |
| 10 | Ultra mode slips in long sessions (issue #233) | one-shot SessionStart hook | `UserPromptSubmit` per-turn reanchor hook reinjects every turn | **v0.5** |
| 11 | Rules pruned on context overflow (100+ turn threads) | no safeguard | `PreCompact` hook reinjects before compaction finalizes | **v0.5** |
| 12 | Cursor chat mode unsupported (issue #222, ~25% of editor users excluded) | command-palette-only | Cursor chat native support from day 1 | **v0.5** |
| 13 | No normal-twin / audit trail for accuracy proof | trust the vibes | Hash-linked normal-twin stored 30 days locally, auditable | **v0.5** |
| 14 | Python runtime (pip install, fragile) | ~200ms on 1k-line CLAUDE.md | Rust single static binary, ~2ms (100x faster), zero deps | **v1.0** |
| 15 | Only VSCode-ish reach | 1.5 platforms practically | 4 platforms day 1: Claude Code + Cursor + Codex + VS Code | **v0.5** → **v1.0** |
| 16 | No gateway adapters — can't drop into LiteLLM/Portkey | manual skill only | Day-1 adapters for LiteLLM, Portkey, OpenRouter | **v1.0** |
| 17 | No MCP server for programmatic access | skill-only | `streetman serve` MCP server (compress/expand/accuracy tools) | **v1.0** |
| 18 | No stats visibility for users | mode badge only | Live ledger: `[STREET:FULL] saved 4.2k tok (63%) acc:100` — savings + accuracy visible | **v0.5** |
| 19 | No independent verification of competitors' claims | the category has no authority | **Bench-as-Service CLI** — `streetman bench test-skill RULES.md` sets honesty bar | **v1.0** |
| 20 | Cross-platform inconsistency, Python pinning hell | dependency fragility | Cross-compiled binaries (darwin/linux/windows, arm64+x64) | **v1.0** |
| 21 | Hand-edited README numbers (no claim audit) | "caveman saves up to 75%" | Auto-rendered from bench snapshot; pre-commit hook rejects hand edits | **v1.0** |
| 22 | Only 10 bench prompts × 1 model × 1 trial (statistically weak) | n=10, no CI | 30 prompts × 4 models × 3 trials = 1,440 calls, 95% bootstrap CI | **v0.1** → **v1.0** |
| 23 | No i18n beyond wenyan (untested, unvalidated) | wenyan claimed but unbenched | v1.0 ships English-only w/ measured bench; i18n lands v2.0 w/ per-language benches (no hand-wave claims) | **v1.0** / v2.0 |
| 24 | Token-budget awareness missing | fixed intensity | `--max-tokens N` adaptive intensity (lite → full → ultra) per prompt | v2.0 |
| 25 | Thinking tokens untouched (60% of spend on extended-thinking models, issue #244) | explicit disclaimer, no fix | Thinking-token trimmer — compresses reasoning phase too | v2.0 |

**Kills ≤ v1.0:** 22 of 25. Every kill-shot feature ships in the first 4 weeks.
**Kills in v2.0:** 3 remaining (token-budget, thinking-token, i18n) — caveman doesn't have these either, so not required to win. Added later as "further stretching the lead."

### v1.0 exit condition (the coronation)
- All 22 kill-shots live
- Bench beats leader by ≥30% AND holds 100% accuracy
- Public release on GitHub, HN, Reddit
- LiteLLM / Portkey / OpenRouter PRs merged
- Distribution in Claude Code marketplace, VS Code marketplace, Homebrew, crates.io
- Day 1 of public availability: caveman becomes the old king. No takebacks.

---

## Version strategy

- Pre-1.0 (`v0.x`) → internal/private → rapid iteration, breaking changes OK
- `v1.0` → public OSS launch on GitHub → semver stability begins
- `v1.x` → backward-compatible additions + community contributions
- `v2.0` → major version (new architecture / breaking changes)
- `v3.0` → longer-horizon features

Every release tagged, signed, attached to a committed bench snapshot. No release ships without accuracy gate passing.

---

## `v0.1` — bench-first alpha *(private, week 1)*

**Goal:** measurement infra locked before one rule is written.

✅ In scope:
- Rust workspace scaffold (`crates/streetman-core`, `-bench`, `-cli`)
- `streetman-bench` harness (4 arms × 4 models × 3 trials)
- 30 bench prompts + 3 accuracy-trap prompts
- `accuracy_rubric` Rust module (regex extractor + LLM semantic judge)
- Ground-truth snapshot committed: `benchmarks/results/bench-phase0.json`
  - `__baseline__` arm (no system prompt)
  - `__terse__` arm ("Answer concisely.")
  - `__leader__` arm (opaque competitor rules, loaded from external config)
- CLI stub: `streetman bench run`, `streetman bench render`, `streetman bench audit`

❌ Out of scope: any streetman compression rules, any host integration, any variants.

**Ship gate:** harness deterministic across 2 runs. Ground-truth snapshot committed.

---

## `v0.2` — skeleton engine beta *(private, week 2)*

**Goal:** first streetman compression pass, bench gate passes.

✅ In scope:
- `skeleton.rs` — algorithmic consonant-skeleton engine (unbounded)
- `skeleton_guards.rs` — protect identifiers, URLs, code, proper nouns
- `skeleton_collide.rs` — collision detector
- `skeleton_freq.rs` — top-2000 precomputed frequency table
- `shortcuts.rs` — high-value phrase shortcuts (`u, ur, rn, cuz, w/, w/o, b4, thru, gonna, wanna, tbh, ngl, fr, afaik, iirc, imo, idk`)
- `symbols.rs` — narrow-safe substitution (`& | @ = ≠` only)
- `numerics.rs` — SI-only unit crunching
- `phrases.rs` — phrase chunks
- `punctuation.rs` — collapse rules
- `accuracy.rs` — 100% rubric enforcement
- Streetman arm runs full bench (360 calls: 30 × 4 models × 3 trials)
- CLI: `streetman compress`, `streetman expand`

❌ Out of scope: emoji layer, acronym learning, code-comment compressor, table-first restructure, domain profiles, variants, hooks, platform integration.

**Ship gate:** ≥85% savings vs normal AND ≥30% over leader AND 100% accuracy on all 360 streetman-arm outputs.

---

## `v0.3` — completeness beta *(private, week 2.5)*

**Goal:** remaining core compression features.

✅ In scope:
- `emojis.rs` — 3-rule-safe emoji layer (define-on-first-use + whitelist + domain-gated)
- `acronyms.rs` — auto-acronym learning
- `comments.rs` — code-comment compressor (prose in comments → skeleton, code byte-exact)
- `semantic.rs` — safe pair rules (`before X, Y → b4 X, Y`)
- `highstakes.rs` — auto-fallback on CVE / `rm -rf` / `DROP TABLE` / security warnings
- `markdown.rs` — post-compression MD AST validator
- Table-first restructuring
- Domain profiles: `sql`, `json`, `k8s`, `docs`

❌ Out of scope: variants, hooks, platform wrappers.

**Ship gate:** bench still passes gate after adding all features. No accuracy regression.

---

## `v0.5` — platform beta *(private, week 3)*

**Goal:** complete surface for dev use, not yet public.

✅ In scope:
- Variants (separate skills):
  - `streetman-commit` — Conventional Commits w/ street lexicon in body
  - `streetman-review` — `L42: 🔴 null ref. add guard.` one-liners
  - `streetman-compress` — input-file compression w/ preview-gate (diff + y/N before overwrite)
- Hooks (JS shims calling Rust binary):
  - `streetman-activate.js` (SessionStart)
  - `streetman-reanchor.js` (UserPromptSubmit, per-turn reinject)
  - `streetman-compaction.js` (PreCompact)
  - `streetman-ledger.js` (statusline: `[STREET:FULL] saved 4.2k tok (63%) acc:100`)
- Rollback safety net (normal-twin hash-linked, 30-day local store)
- Plugin wrappers:
  - Claude Code (`.claude-plugin/plugin.json` + skills)
  - Cursor (`.cursor/skills/`)
  - Codex CLI (`codex plugin` manifest)
  - VS Code (extension package)

❌ Out of scope: gateway adapters, MCP server, token-budget adaptive mode, thinking-token trimmer.

**Ship gate:** all 4 platforms install + run; bench still passes.

---

## `v1.0` — 🚀 **OSS public launch** *(week 4)*

**Goal:** GitHub public, HN/Reddit launch, first 5k stars.

✅ In scope:
- Everything from v0.1 → v0.5
- Gateway adapters (drop-in for existing proxies):
  - `adapters/litellm/`
  - `adapters/portkey/`
  - `adapters/openrouter/`
- **Bench-as-Service CLI** — `streetman bench test-skill <path-to-rules.md>` — runs any third-party compression skill through streetman's 30-prompt × 4-model × 3-trial matrix + accuracy rubric. Produces verified scorecard + public snapshot. **This is the category-defining feature — nobody else has it.**
- Basic MCP server (`streetman serve`) exposing `compress`, `expand`, `accuracy_check` tools
- Cross-compiled release binaries:
  - darwin-arm64, darwin-x64
  - linux-x64, linux-arm64
  - windows-x64
- GitHub Actions CI:
  - bench gate on every PR
  - accuracy-rubric regression
  - cross-platform build
  - release automation
- Complete public docs:
  - `README.md` (auto-rendered from bench snapshot)
  - `PLAN.md`, `BUSINESS.md`, `MASTER.md`, `ROADMAP.md`
  - `CLAIMS.md` (every number cites snapshot)
  - `CONTRIBUTING.md`, `SECURITY.md`, `CODE_OF_CONDUCT.md`
  - `docs/quickstart.md`, `docs/intensity-levels.md`
- Distribution:
  - GitHub Releases w/ signed binaries
  - Homebrew tap
  - `cargo install streetman`
  - Claude Code marketplace
  - VS Code marketplace
  - LiteLLM / Portkey plugin PRs merged

❌ Out of scope (parked for v1.x / v2.0): i18n, streaming mode, thinking-token trimmer, token-budget adaptive, JetBrains support, bench-as-service CLI.

**Ship gate:**
- Full 1,440-call bench passes (≥85% savings, ≥30% over leader, 100% accuracy)
- All 4 platforms verified running on clean install
- `CLAIMS.md` auto-audit green (every number cites snapshot)
- README, docs, install paths all working end-to-end
- Tag `v1.0.0`, sign release, publish blog post

---

## `v1.x` — community phase *(month 2-3, post-launch)*

**Goal:** community-driven depth. Backward-compatible additions only.

✅ Community-contributed (PRs welcome, bench gate enforced):
- More domain profiles (rust, go, python-ml, terraform, ansible, react, svelte, dotnet-csproj, etc.)
- More platform integrations:
  - JetBrains IDEs (IntelliJ, GoLand, PyCharm, WebStorm) — `v1.1`
  - Vim/Neovim — `v1.2`
  - Emacs — `v1.3`
  - Helix — `v1.4`
- Additional gateway adapters:
  - Helicone — `v1.1`
  - Cloudflare AI Gateway — `v1.2`
  - Braintrust / LangSmith pre-processor — `v1.3`
- Shell integrations (bash/zsh/fish prompt wrappers) — `v1.5`
- More bench prompts (community-submitted real-world scenarios)
- Accuracy-regression fixtures (every reported issue becomes a fixture)
- Performance optimizations (criterion benches show improvements)
- Bug fixes

❌ Stays out (v2.0 or DevBooster):
- Any breaking API/CLI change
- New intensity levels (requires re-bench)
- Changes to accuracy rubric methodology

---

## `v2.0` — i18n + streaming + advanced features *(month 4-6)*

**Goal:** major version. Breaking changes allowed. Lexicon v2 format.

✅ In scope:
- **i18n compression** — first-class support for non-English languages
  - Spanish (`es`), Chinese (`zh`), Japanese (`ja`), German (`de`), French (`fr`), Portuguese (`pt`)
  - Each w/ own bench matrix (30 prompts × 4 models × 3 trials per language)
  - Per-language skeleton rules (Chinese radicals, Japanese kana compression, German compound words)
- **Streaming compression mode** — compress tokens as they generate (real-time, before user sees verbose output)
  - Requires rewrite of engine hot path for incremental tokens
  - Benchmarked separately: latency overhead must be <5ms per 100 tokens
- **Per-project `.streetman.toml` full config surface**
  - Custom lexicons
  - Domain profile override
  - Per-project ship gate (e.g., "require ≥90% accuracy, ≥70% savings")
  - Forbidden-emoji list
  - Protected-identifier list (regex)
- **Token-budget-aware adaptive intensity**
  - `streetman compress --max-tokens 200 input.md` → auto-selects lite/full/ultra per section to hit exact budget
- **Thinking-token trimmer**
  - Extended-thinking models (Opus w/ thinking, o1-class)
  - Explicit rule for compressing reasoning phase
  - Measured separately: thinking-token reduction bench
- **Per-turn mode anchor hook** (if not already in v0.5 — promoted to core in v2.0)
- **Context-overflow auto-reinject** (PreCompact enhanced for 200+ turn threads)
- **Lexicon v2 format** (TOML → extensibility annotations, versioning, source attribution)
- **Migration tool** — `streetman migrate-config 1.x 2.0` auto-upgrades user configs

❌ Stays out:
- Adaptive per-user learning (v3.0)
- Structured output compression (v3.0)
- Plugin API for third-party engine extensions (v3.0)

**Ship gate:**
- All v1.x platforms still work (backward-compat for user configs)
- i18n bench passes gate for each new language (100% accuracy enforced per-language)
- Streaming mode benchmarked at <5ms latency overhead
- Migration tool tested on 100+ real-world configs

---

## `v3.0` — adaptive + structured *(month 7-12)*

**Goal:** next-gen compression. Plugin API opens engine to third parties.

✅ In scope:
- **Adaptive lexicon learning** — per-user / per-repo / per-org lexicon refinement
  - Observes patterns in accepted compressions, suggests new shortcuts
  - User-controlled (never auto-applies new rules without confirmation)
  - Privacy: all learning local unless explicitly synced via DevBooster
- **Structured output compression** — JSON / YAML / XML / TOML schema-aware
  - Preserves schema semantics while compressing prose within string fields
  - Integrates w/ TOON-style structural compression
- **Full duplex streaming** — compress input prompt + output response in single stream
  - Requires TOOG v1 integration (input side compresses too)
  - Note: TOOG compiler itself stays in DevBooster, but streetman exposes hooks for the compiled input to pass through output-compression path
- **Multi-modal compression** — images / diagrams markdown-ized
  - Mermaid diagrams auto-generated from compressed descriptions
  - Image alt-text compression
- **Plugin API (`streetman-plugin` crate)**
  - Third-party crates can register custom lexicons, patterns, domain profiles
  - Stable ABI for loaded plugins
  - Security: sandbox + signature verification
- **Extended MCP server**
  - Streaming tool calls
  - Multi-turn compression state
  - Session accuracy metrics exposed

❌ Still out (stays in DevBooster):
- TOOG compiler itself
- Patch execution + verification loop
- IDE UI surfaces
- Telemetry collection / central analytics
- Compliance pack content (HIPAA/SOX/finance lexicons — commercial-only)

---

## Hard out-of-scope (streetman OSS will NEVER ship these — they live in DevBooster)

| Feature | Why it's commercial |
|---|---|
| TOOG compiler (parsing → repo/build context resolution → prompt generation) | Patent-pending. Core moat. |
| IDE UI (findings panel, patch apply panel, benchmarks panel) | DevBooster VSCode/Cursor/VS2022 extension product |
| Savings-Share billing meter | Closed commercial SaaS |
| Centralized telemetry / analytics | Closed commercial (enterprise buyer) |
| Org policy enforcement (`.streetman.json` org-wide) | Enterprise tier |
| SSO / audit log / RBAC | Enterprise tier |
| Compliance packs (HIPAA, SOX, finance, legal lexicons) | Commercial add-on |
| Hosted bench-as-service | Commercial SaaS |
| Adaptive learning sync across users/orgs | Requires hosted backend — commercial |
| Certification mark issuance (`streetman-certified` badge) | Commercial licensing program |

If a PR proposes any of the above, it's closed w/ pointer to DevBooster. Keeps OSS repo lean + commercial layer protected.

---

## Release cadence

| Window | Cadence | Example |
|---|---|---|
| Pre-1.0 | internal, weekly | v0.1 → v0.2 → v0.3 → v0.5 |
| v1.x | bi-weekly minor, monthly stable | v1.1 → v1.2 → v1.2.1 |
| v2.0 → v3.0 | major every ~3 months | v2.0 month 4, v3.0 month 7 |
| Patch releases | as needed (accuracy regressions, CVEs) | v1.0.1 within 24h of report |

---

## Deprecation policy

- Deprecations announced in minor release (e.g., `v1.3`)
- Deprecated feature removed in next major (`v2.0`)
- Lexicon entries never removed without community vote + 2 version grace period
- CLI flags deprecated w/ warning for 2 minor versions before removal

---

## Versioning the bench itself

The bench matrix evolves. To prevent "moving goalposts" accusations:

- Bench matrix version pinned per streetman version (e.g., `v1.0` uses `bench-matrix-v1.json`)
- New prompts added as `bench-matrix-v2.json` in `v2.0`
- Historical snapshots remain reproducible (old matrix can be re-run on new streetman)
- Cross-version comparisons use overlap prompt set

---

## Summary — streetman OSS scope at a glance

| Version | When | What | Public? |
|---|---|---|---|
| `v0.1` | Week 1 | Bench harness + rubric + ground-truth | ❌ private |
| `v0.2` | Week 2 | Skeleton engine + lexicons pass gate | ❌ private |
| `v0.3` | Week 2.5 | Full compression feature set | ❌ private |
| `v0.5` | Week 3 | Variants + hooks + 4 platforms | ❌ private beta |
| **`v1.0`** | **Week 4** | **Gateway adapters + MCP + cross-platform binaries** | ✅ **GitHub launch** |
| `v1.x` | Month 2-3 | Community contributions, more platforms/adapters | ✅ rolling |
| `v2.0` | Month 4-6 | i18n + streaming + adaptive intensity + v2 lexicon format | ✅ major |
| `v3.0` | Month 7-12 | Adaptive learning + structured output + plugin API | ✅ major |
| `v4.0+` | Year 2+ | Community-driven | ✅ |

**Anything beyond `v3.0` is community-roadmap. Streetman core stabilizes; DevBooster (commercial) absorbs all strategic roadmap.**
