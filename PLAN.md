# STREETMAN — Technical Plan

> "why use many token when few do trick" — old king.
> "why use words at all" — streetman.

**See also:** [BUSINESS.md](./BUSINESS.md) for competitive + monetization + GTM.

---

## 0. Context

Clean-room token-compression skill + binary that beats the current leader by **≥30% additional cut** while holding **100% technical accuracy**. Rust implementation — single static binary, cross-platform, zero runtime deps.

**The bar to beat:**
```
│  TOKENS SAVED          ████████ 75%   │
│  TECHNICAL ACCURACY    ████████ 100%  │
│  SPEED INCREASE        ████████ ~3x   │
│  VIBES                 ████████ OOG   │
```

**Streetman's target:**
```
│  TOKENS SAVED          █████████ 85%+ │
│  TECHNICAL ACCURACY    █████████ 100% │
│  SPEED INCREASE        █████████ ~5x  │
│  VIBES                 █████████ GOAT │
```

**Hard rules:**
1. Bench designed + committed BEFORE a single rule-file line written.
2. Zero visible lineage to any predecessor. Clean-room identity.
3. Technical accuracy floor is **100%** — binary gate. Facts, code, URLs, numbers, identifiers, error strings preserved exactly.
4. ≥30% additional cut on top of the 75% leader — measured, not claimed.
5. Full delivery surface (mode + commit + review + compress + bench) from day one.
6. Language: **Rust**.

---

## 1. Target numbers (bench-first)

| Arm | Avg out tokens | vs normal | vs leader |
|---|---:|---:|---:|
| Normal (control) | ~1200 | — | — |
| Terse control | ~900 | −25% | — |
| Leader (current king, full) | ~300 | −75% | — |
| **Streetman (full)** | **~180** | **−85%** | **−40%** |
| Streetman (ultra) | ~120 | −90% | −60% |

**Accuracy floor:** 100% across every (prompt × model × arm × trial). Zero tolerance.

**Ship gate (must ALL be true):**
- Median savings ≥85% vs normal
- Median savings ≥30% over leader arm
- Accuracy 100% across 30-prompt × 4-model × 3-trial matrix (1,440 outputs, zero drops)

Miss any → iterate rules, rerun, do not publish.

---

## 2. Bench design (Phase 0 — no skill code until this lands)

**Arms (4):**
1. `__baseline__` — no system prompt
2. `__terse__` — "Answer concisely."
3. `__leader__` — reigning competitor rules, opaque string from external config (never committed)
4. `streetman` — baseline + streetman SKILL.md

**Prompts (30 + 3):**
- 5 debugging, 5 architecture, 5 devops, 5 code-review, 5 explain, 5 refactor
- 3 accuracy traps: security warning, destructive migration, multi-step recipe

**Models (4):**
- `claude-opus-4-7`
- `claude-sonnet-4-6`
- `claude-haiku-4-5`
- `gpt-5-codex` (or current via Codex CLI)

**Trials:** 3 per (prompt × arm × model) → **30 × 4 × 4 × 3 = 1,440 calls**. 95% CI via bootstrap.

**Metrics:**
- Output tokens (actual API response count — not tokenizer approx)
- % savings vs terse control
- 95% CI (bootstrap resample)
- **Accuracy (0/100 binary):** regex-extract identifiers / URLs / numbers / error strings / CVE / flags / versions from normal answer, match against compressed answer. LLM-as-judge for semantic equivalence.
- Thinking tokens measured separately when extended thinking enabled

**`streetman-bench` binary:**
```
streetman-bench run --arms all --models all --trials 3 --out results/bench-$(date +%Y%m%d).json
streetman-bench render --input results/bench-latest.json --template README.tmpl --out README.md
streetman-bench audit --input README.md    # pre-commit: every claim cites snapshot
```

---

## 3. 24 Killer Diffs (what eradicates the old king)

### Core compression (far beyond article-dropping)
1. **Algorithmic Consonant-Skeleton Engine (UNBOUNDED)** — every word auto-reduces to consonant skeleton. Not a lookup — a rule engine handling arbitrary vocabulary. `can→cn`, `turn→trn`, `short→shrt`, `database→dtbs`, `configuration→cnfgrtn`. Rules:
   - Drop interior vowels when skeleton unambiguous
   - Keep first + last consonant cluster
   - Words ≤3 chars untouched (`is`, `be`, `it`)
   - Guards: backticks, code blocks, URLs, proper nouns, identifiers, numbers — never reduced
   - Collision detector: if skeleton ambiguous vs another reduced word in same paragraph, retain one vowel
   - Frequency-tuned: top-2000 English words precomputed (O(1)), rare words rule-generated
   - Reader recovery ≥98% bench-verified
2. **High-Value Phrase Shortcuts** — `u, ur, rn, cuz, w/, w/o, b4, thru, gonna, wanna, tbh, ngl, fr, afaik, iirc, imo, idk`. Unambiguous in tech prose.
3. **Symbol Substitution Engine (narrow + safe)** — keep only: `and→&`, `or→|`, `at→@`, `equals→=`, `not equals→≠`. Dropped ambig: `→ ⟹ ∴ ∵ 2 4 #`.
4. **Emoji Compression Layer (100%-gated, 3-rule safe)**:
   - **Rule A** — Define-on-first-use: `deploy (🚀)` first, `🚀` after. Key always in reader scope.
   - **Rule B** — Single-dominant-meaning whitelist: `✅ ❌ ⚠️ 🔒 ⏱️ 🔴 🟡 🟢 🔵`. Others only via define-on-first-use.
   - **Rule C** — Domain-gated: active in `streetman-review` only (PR convention exists). General prose compression doesn't emoji-swap.
5. **Numeric Crunching (SI-only)** — `500 milliseconds→500ms`, `24 hours→24h`, `1 kilobyte→1KB`, `3 times→3x`, `first→1st`. Dropped ambig: `15m` (million/meter/minute) → use `15min`; `2w` (with/watts) → `2wk`.
6. **Phrase Chunk Lexicon** — `make sure to→ensure`, `in order to→to`, `at this point→now`, `a lot of→many`, `as a result→so`, `due to the fact that→because`.
7. **Semantic Pair Compression (safe subset)** — `before X, Y → b4 X, Y`. Dropped risky `if X then Y → X? Y.` form (? ambig w/ question).
8. **Auto-Acronym Learning** — first mention defines (`database (dtbs)`), later mentions skeleton-form. Deterministic.
9. **Table-First Restructuring** — comparison prose auto-rewritten as 2-col table. Tables tokenize ~30% tighter.
10. **Punctuation Collapse** — drop trailing periods on fragments, collapse multi-space.
11. **Code-Comment Compressor** — inline `//` `#` `/* */` skeleton-treated, code logic byte-exact. +5-10% on code-dense files.

### Reliability & accuracy
12. **Hard Accuracy Rubric (100% gate)** — regex-extracted claims + LLM semantic judge → 0/100 per response. <100 auto-reverts + retries in lite. CI-enforced.
13. **Auto-Fallback on High-Stakes Content** — CVEs, destructive ops (`rm -rf`, `DROP TABLE`), security warnings, financial/legal/medical → auto-normal mode. Hard rule.
14. **Markdown Structure Validator** — post-compression AST parse confirms headings/lists/tables/code-blocks still valid.
15. **Rollback Safety Net (Normal Twin)** — every compressed output hash-linked to full-fat normal version, stored 30 days locally. Audit trail.
16. **LLM Expand-to-Plain Mode** — `streetman expand <text>` reconstructs normal English. Honest label: reconstructive, not byte-reversible.
17. **Preview-Gate on Compress** — input-file compression shows diff + token delta + accuracy score, requires `y/N`. No silent overwrite.

### Mode stability
18. **Per-Turn Mode Anchor Hook** — `UserPromptSubmit` hook reinjects rules every turn. Fixes long-session drift.
19. **Context-Overflow Auto-Reinject** — `PreCompact` hook reinjects before compaction finalizes. Survives 100+ turn threads.
20. **Token-Budget Aware Intensity** — `--max-tokens N` auto-adapts intensity (lite→full→ultra) per prompt.

### Platform reach
21. **Multi-Platform Native** — Claude Code + Cursor chat + Codex CLI + VS Code extension. Same Rust binary, four host wrappers. Day-one.

### UX, trust, extensibility
22. **Live Stats Badge + Session Ledger** — statusline: `[STREET:FULL] saved 4.2k tok (63%) acc:100`. Cumulative savings + accuracy displayed.
23. **Per-Project `.streetman.toml`** — repo-local: custom lexicons, domain profile override, per-project ship gate, forbidden-emoji list.
24. **Thinking-Token Trimmer** — explicit rule for extended-thinking models to compress reasoning phase.

### Ecosystem
25. **Bench-as-Service CLI** — `streetman-bench test-skill <rules.md>` runs any compression skill against our 30-prompt × 4-model × 3-trial bench + accuracy rubric. Sets honesty bar for category.

**Plus domain profiles** — `streetman-sql`, `streetman-json`, `streetman-k8s`, `streetman-docs` — SQL keywords / JSON keys / kubectl shortforms / MD frontmatter never mangled.

---

## 4. Delivery surface

| Skill | Purpose |
|---|---|
| `streetman` | Core output mode (lite / full / ultra) |
| `streetman-commit` | Conventional commits, street lexicon in body only (subject strict ≤50ch imperative) |
| `streetman-review` | `L42: 🔴 null ref. add guard.` one-line PR comments |
| `streetman-compress` | Input-file compression w/ preview-gate |
| `streetman-bench` | Standalone bench harness (reusable against any compression skill) |

---

## 5. Comparison example (the sell)

**Prompt:** "Why is my React component re-rendering?"

| Mode | Output | Tokens |
|---|---|---:|
| Normal | "The reason your React component is re-rendering is likely because you're creating a new object reference on each render cycle. When you pass an inline object as a prop, React's shallow comparison sees it as a different object every time, which triggers a re-render. I'd recommend using useMemo to memoize the object." | 69 |
| Leader (full) | "New object ref each render. Inline object prop = new ref = re-render. Wrap in `useMemo`." | 19 |
| Streetman (lite) | "ur cmpnt re-rndrs cuz u crt new obj ref evry rndr. inline obj prop = new ref. wrap w/ `useMemo`." | 17 |
| **Streetman (full)** | **"inln obj prp → new ref evry rndr → re-rndr. wrp w/ `useMemo`."** | **11** |
| Streetman (ultra) | "inln obj prp ⟹ 🔄 evry rndr. `useMemo`." | 7 |

Leader cuts 72%. Streetman full 84% (−42% over leader). Ultra 90% (−63% over leader). Code/API identifier untouched every mode — skeleton engine only touches prose.

---

## 6. Implementation language: Rust

**Why Rust (not Python / Go / TypeScript):**
- **Single static binary** — ships as plugin w/ zero runtime deps. Python caveman-compress needs `pip install`; streetman doesn't.
- **Speed** — `aho-corasick` matches lexicon over 10k-char input in microseconds. Python caveman-compress ~200ms on 1k-line CLAUDE.md; streetman Rust ~2ms.
- **Cross-platform** — cross-compile for darwin-arm64/x64, linux-x64/arm64, windows-x64 from CI. Same binary in every host.
- **Safety** — `Result<T,E>` + borrow checker → accuracy rubric bugs caught at compile time.
- **Tokenizer ecosystem** — HuggingFace `tokenizers` crate gives real Claude/GPT tokenization via `tokenizer.json`. No `tiktoken o200k_base` approximation.
- **Parallel bench** — `rayon` trivially parallelizes 1,440 calls w/ bounded concurrency.

**Binary surface:**
```
streetman compress <file>       # input-file compression w/ preview-gate
streetman expand <text>         # reconstruct normal English
streetman bench run             # full 4-arm × 4-model × 3-trial matrix
streetman bench test-skill RULES.md   # bench any rule file
streetman serve                 # MCP server (compress/accuracy tools)
streetman format commit < DIFF  # conventional commit generator
streetman format review < DIFF  # one-line PR comments
```

**Crates picked:**
| Crate | Role |
|---|---|
| `clap` | CLI w/ derive macros |
| `serde` + `serde_json` + `toml` | config, snapshots, lexicons |
| `aho-corasick` | multi-pattern lexicon matcher (millions of ops/sec) |
| `regex` | pattern rules (semantic pair, numeric crunch) |
| `tokenizers` | actual model tokenizers (not approx) |
| `reqwest` + `tokio` | async API calls to Anthropic + OpenAI |
| `rayon` | parallel bench fanout |
| `similar` | unified diff for preview-gate |
| `pulldown-cmark` | markdown AST for structure validator |
| `indicatif` | live stats badge + progress bar |
| `blake3` | hash-linked normal-twin store |
| `criterion` | microbenches for lexicon hot path |

---

## 7. File structure

```
/Users/efi.jeremiah/projects/streetman/
├── PLAN.md                         # this file
├── BUSINESS.md                     # competitive + monetization + GTM
├── README.md                       # auto-rendered from bench snapshot
├── CLAIMS.md                       # every claim → snapshot citation
├── LICENSE                         # MIT (core)
├── CONTRIBUTING.md
├── CODE_OF_CONDUCT.md
├── SECURITY.md
├── Cargo.toml                      # workspace
├── Cargo.lock
├── rust-toolchain.toml             # pinned 1.85+
├── crates/
│   ├── streetman-core/             # compression engine, rubric
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── skeleton.rs         # UNBOUNDED consonant-skeleton engine
│   │       ├── skeleton_guards.rs  # protect identifiers / URLs / code / short words
│   │       ├── skeleton_collide.rs # collision detector
│   │       ├── skeleton_freq.rs    # top-2000 precomputed frequency table
│   │       ├── shortcuts.rs        # high-value phrase shortcuts (u, ur, rn, cuz, w/)
│   │       ├── symbols.rs          # narrow symbol substitution
│   │       ├── emojis.rs           # 3-rule-safe emoji layer
│   │       ├── numerics.rs         # SI-only unit crunching
│   │       ├── phrases.rs          # phrase chunks
│   │       ├── semantic.rs         # safe semantic pair rules
│   │       ├── acronyms.rs         # auto-acronym learning
│   │       ├── punctuation.rs      # collapse
│   │       ├── comments.rs         # code-comment compressor
│   │       ├── highstakes.rs       # auto-fallback detector
│   │       ├── domain.rs           # domain profile dispatch
│   │       ├── accuracy.rs         # 100% rubric
│   │       ├── markdown.rs         # AST validator
│   │       └── budget.rs           # token-budget-aware intensity
│   ├── streetman-cli/              # binary entry (clap)
│   ├── streetman-bench/            # 4-arm × 4-model × 3-trial harness
│   │   └── src/
│   │       ├── main.rs
│   │       ├── arms.rs
│   │       ├── models.rs           # anthropic + openai clients
│   │       ├── prompts.rs
│   │       ├── accuracy_rubric.rs
│   │       ├── bootstrap.rs        # CI calc
│   │       └── render.rs           # README table templating
│   └── streetman-mcp/              # MCP server
├── skills/                         # Claude Code / Cursor / Codex skill defs
│   ├── streetman/SKILL.md
│   ├── streetman-commit/SKILL.md
│   ├── streetman-review/SKILL.md
│   └── streetman-compress/SKILL.md
├── lexicons/                       # data-driven rules (embedded at compile)
│   ├── shortcuts.toml
│   ├── symbols.toml
│   ├── emojis.toml
│   ├── phrases.toml
│   ├── numerics.toml
│   └── domains/
│       ├── sql.toml
│       ├── json.toml
│       ├── k8s.toml
│       └── docs.toml
├── hooks/                          # JS shims calling streetman binary
│   ├── streetman-activate.js       # SessionStart
│   ├── streetman-reanchor.js       # UserPromptSubmit
│   ├── streetman-compaction.js     # PreCompact
│   └── streetman-ledger.js         # statusline badge
├── benchmarks/
│   ├── prompts.json                # 30 + 3 traps
│   └── results/
│       └── bench-YYYYMMDD.json
├── templates/
│   └── README.tmpl                 # auto-rendered README source
├── tests/
│   ├── lexicon/                    # per-entry bidirectional tests
│   ├── accuracy/                   # rubric coverage
│   ├── compress/                   # before/after fixtures
│   └── e2e/                        # full-pipeline tests
├── plugins/                        # host wrappers
│   ├── claude-code/
│   ├── cursor/
│   ├── codex/
│   └── vscode/
├── adapters/                       # gateway integrations
│   ├── litellm/
│   ├── portkey/
│   └── openrouter/
├── docs/
│   ├── quickstart.md
│   ├── intensity-levels.md
│   ├── writing-accuracy-rubrics.md
│   └── architecture.md
├── .claude-plugin/plugin.json
├── .github/
│   ├── workflows/
│   │   ├── bench-ci.yml            # bench on every PR, block gate regression
│   │   ├── release.yml             # cross-compile binaries
│   │   └── audit-claims.yml        # every README number cites snapshot
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
└── .gitignore
```

**Clean-room rule:** no file inside `streetman/` references predecessor by name. Leader arm loads rules from external config path supplied at runtime only.

---

## 8. Execution phases

**Phase 0 — Bench scaffold (no skill code):**
1. Scaffold Rust workspace
2. Build `streetman-bench` crate — 4 arms, 4 models, 3 trials, bootstrap CI
3. Build `accuracy_rubric.rs` — deterministic extractor + LLM semantic judge
4. Write 30-prompt + 3-trap set
5. Dry-run (no API calls) — validate config + count tokens locally
6. Confirm $ budget w/ user before firing real calls
7. Run baseline + terse + leader arms → commit `bench-phase0.json`. **Ground truth locked before streetman rules exist.**

**Phase 1 — Lexicons + compression engine:**
1. Write `lexicons/*.toml`. Every entry bidirectionally tested for 100% reader recovery.
2. Build `streetman-core` — aho-corasick matcher + pattern rules + accuracy rubric + markdown validator
3. Write `skills/streetman/SKILL.md` referencing binary
4. Domain profiles
5. `cargo test` — every lexicon entry paired test

**Phase 2 — Bench streetman + iterate to gate:**
1. Run streetman arm (360 calls: 30 × 4 models × 3 trials)
2. Accuracy rubric each output
3. If gate missed (savings <30% over leader OR accuracy <100): tune lexicons, rerun. Loop until pass.
4. Commit `bench-final.json`. Auto-render README table.

**Phase 3 — Variants + hooks + platform surface:**
1. `streetman-commit`, `streetman-review`, `streetman-compress` skills
2. Preview-gate on compress (diff + y/N via `similar`)
3. Hooks: activate, reanchor, compaction, ledger
4. Plugin wrappers: Claude Code, Cursor, Codex, VS Code
5. Gateway adapters: LiteLLM, Portkey, OpenRouter
6. MCP server crate

**Phase 4 — Publish:**
1. Auto-generate `README.md` + `CLAIMS.md` from snapshot
2. `.claude-plugin/plugin.json` marketplace manifest
3. `docs/quickstart.md`
4. CI: bench runs every PR, blocks merge if gate regression
5. Release: cross-compile binaries (5 targets)
6. Post to HN / Reddit / LiteLLM community / Claude Code Discord

---

## 9. Verification (how we prove the crown)

1. **Bench reproducibility** — `streetman bench run --trials 3` diff against committed, match within CI band
2. **Lexicon unit tests** — `cargo test -p streetman-core` — every shortcut/symbol/emoji/phrase/numeric bidirectionally verified for 100% recovery
3. **Accuracy regression** — `cargo test --test accuracy` — rubric must score 100 across every fixture
4. **Hook persistence** — synthetic 60-turn conversation, verify reanchor fires each turn. Tokens turn 1 vs turn 60 stay within ±10%
5. **Preview-gate** — `streetman compress tests/compress/sample.md` shows diff + savings + accuracy + y/N. No silent overwrite.
6. **Live demo** — 5 hand-picked prompts × 4 models
7. **Claim audit** — `streetman bench audit README.md` — every number cites snapshot. Pre-commit hook enforces.
8. **Perf bench** — `cargo bench` (criterion) — lexicon matcher processes 100KB in <10ms on M1

---

## 10. Locked decisions

| Decision | Value |
|---|---|
| Language | **Rust** 1.85+ |
| Name | **streetman** (locked) |
| Ship gate | ≥30% over leader AND 100% accuracy |
| Bench matrix | 30 prompts × 4 models × 4 arms × 3 trials = 1,440 calls |
| Models | Opus 4.7 + Sonnet 4.6 + Haiku 4.5 + recent Codex |
| Delivery | streetman + commit + review + compress + bench |
| Host platforms | Claude Code + Cursor + Codex + VS Code — day 1 |
| Distribution | LiteLLM / Portkey / OpenRouter adapters on launch |
| Lineage | Clean-room, zero predecessor refs |
| Bench-first | No rule code until Phase 0 snapshot committed |
| License | Core MIT (OSS) + commercial tier (see BUSINESS.md) |
| Feature count | 24 (emoji restored w/ 3-rule safeguards) |

---

## 11. Phase 0 greenlight checklist

- [x] Rust toolchain (1.94.1 installed)
- [x] PLAN.md finalized
- [x] BUSINESS.md finalized
- [ ] ~$20 Phase 0 bench budget greenlit
- [ ] Codex CLI installed + API key confirmed (4th model arm)
- [ ] `ANTHROPIC_API_KEY` + `OPENAI_API_KEY` in env
- [ ] MCP server scope: v1 or v2?

Once all checked → execute Phase 0.
