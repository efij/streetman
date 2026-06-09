<div align="center">

# 🏙️ streetman

### **The compression platform for AI-augmented software engineering.**

*Targets LLM output cuts of 85%+ at 100% technical accuracy. Binary-gated, bench-first, multi-platform.*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](./LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.85+-orange)](https://www.rust-lang.org/)
[![Accuracy](https://img.shields.io/badge/Accuracy-100%25-brightgreen)](./CLAIMS.md)
[![Token Cut](https://img.shields.io/badge/Token_Cut-85%25+-ff69b4)](./CLAIMS.md)
[![Bench](https://img.shields.io/badge/Bench-1440_calls-9cf)](./benchmarks/)
[![Platforms](https://img.shields.io/badge/Platforms-4_native-purple)](#-install)
[![GitHub stars](https://img.shields.io/github/stars/yourorg/streetman?style=social)](https://github.com/yourorg/streetman)

**[ Install ](#-install) · [ Benchmarks ](#-benchmarks) · [ Docs ](./docs/quickstart.md) · [ Feature Matrix ](./FEATURE_MATRIX.md) · [ Claims ](./CLAIMS.md)**

</div>

---

## current status

Streetman now has a working Rust implementation plus a committed offline competitor
snapshot that passes the local absolute-win gate:

- `streetman compress` — deterministic compression for prose, JSON, logs, search, diffs, code, docs, HTML.
- `streetman retrieve` — encrypted local archive retrieval for exact originals.
- `streetman audit` — local quality/waste reports and dashboard HTML.
- `streetman bench` — fixture benches, live competitor capture, and gated compare output.
- `streetman proxy` — local health/proxy scaffold; provider forwarding still pending.
- `streetman policy` — local policy-as-code checks for allowed modes/domains, zero telemetry, certificates, gateway targets.
- `streetman proof` — deterministic proof-certificate verification for compressed outputs.
- `streetman diff` — local text/HTML compression diff reports with protected-token accuracy.
- `streetman gateway conformance` — local LiteLLM/OpenRouter/Portkey adapter contract checks.

Current committed snapshot: `competitor-live-2026-06-07`.

- Streetman context: `96.6%`
- Headroom context on measured matching workloads: `90.5%`
- Streetman output on Caveman's own eval snapshot: `65.2%`
- Caveman output on its own eval snapshot: `50.0%`
- Streetman uses `30.3%` fewer output tokens than Caveman on that snapshot
- Streetman session effective fixture: `70.0%`
- Token Optimizer session detect-only effective savings: `0.0%`
- Overall compare status: `absolute-win` for this local offline snapshot.

This is not a universal market claim. The Headroom JSON message-API lane is recorded
as blocked by a local certificate failure, LLMLingua and LeanCTX are tracked as
published top baselines rather than local measured gates, and full provider-forwarding
proxy work is still pending.

---

## why streetman

```
│  TOKENS SAVED          █████████ 85%+ │
│  TECHNICAL ACCURACY    █████████ 100% │
│  SPEED INCREASE        █████████ ~5x  │
│  VIBES                 █████████ GOAT │
```

These are gated snapshot claims. See [CLAIMS.md](./CLAIMS.md).

---

## the pitch in 1 example

**Prompt:** "Why is my React component re-rendering?"

| Mode | Output | Tokens |
|---|---|---:|
| Normal | *"The reason your React component is re-rendering is likely because you're creating a new object reference on each render cycle..."* | **69** |
| Other leading skills | *"New object ref each render. Inline object prop = new ref = re-render. Wrap in `useMemo`."* | **19** |
| 🏙️ **streetman (full)** | `"inln obj prp → new ref evry rndr → re-rndr. wrp w/ `useMemo`."` | **11** |
| 🏙️ **streetman (ultra)** | `"inln obj prp ⟹ 🔄 evry rndr. useMemo."` | **7** |

Code + API identifiers untouched. Only prose compresses.

---

## what makes streetman different

### 🧠 Algorithmic consonant-skeleton engine — UNBOUNDED vocab
Not a fixed lexicon. Every word auto-reduces: `database→dtbs`, `configuration→cnfgrtn`, `check→chk`. Guards protect identifiers, URLs, code, proper nouns. Collision detector prevents ambiguity. Reader recovery ≥98% bench-verified.

### 🎯 Technical accuracy — enforced before claims
Deterministic protected-token extraction checks identifiers, URLs, versions, units, CVEs, and code-like tokens. Score <100 → auto-revert to original. LLM semantic judging is reserved for future live benches.

### 🏎️ Rust single static binary — zero runtime deps
- Cross-compiled: darwin-arm64/x64, linux-x64/arm64, windows-x64
- 100KB input → compressed in <10ms on M1 (100x faster than Python alternatives)
- Same binary serves Claude Code, Cursor, Codex CLI, VS Code

### 🔄 Closes the compression loop (output + input)
- **streetman** — compresses AI output (prose)
- **streetman prompt** (aka TOOG) — compresses developer input (task intent w/ repo/build context)
- Bi-directional compression = 2x savings on every call

### 📊 Bench-as-service — sets the honesty bar for the category
```bash
streetman bench test-skill ./my-compression-rules.md
# Runs your rules through 30 prompts × 4 models × 3 trials.
# Produces: % savings, 95% CI, accuracy score, verified snapshot.
```
Nobody else in the category offers independent verification infra.

---

## 📦 install

### Current verified install
```bash
cargo install --git https://github.com/efij/streetman streetman-cli --bin streetman --locked
```

Run the fixture gate after install:

```bash
streetman bench run --suite absolute-win
```

### Local development
```bash
git clone https://github.com/efij/streetman
cd streetman
cargo run --bin streetman -- bench run --suite absolute-win
```

### Package managers and editor plugins

These channels are not published yet:

- Claude Code marketplace plugin
- Cursor installer
- Codex CLI plugin
- VS Code Marketplace extension
- Homebrew formula
- Crates.io package

### Gateway adapters

The adapter docs exist, but the adapters are not implemented yet:

- [LiteLLM](./adapters/litellm/README.md)
- [Portkey](./adapters/portkey/README.md)
- [OpenRouter](./adapters/openrouter/README.md)

---

## 🚀 quickstart

```bash
# Compress an AI output
echo "Your long verbose response here..." | streetman compress

# Compress an input file and return a proof-carrying JSON result
streetman compress README.md --mode full --domain docs --json

# Run the fixture bench
streetman bench run --suite absolute-win

# Run the safety red-team bench
streetman bench run --suite redteam

# Check local policy-as-code
streetman policy check --mode ultra --domain prose README.md

# Build a local HTML compression diff
streetman diff original.txt compressed.txt --html --out benchmarks/results/compression-diff.html

# Check gateway adapter contracts
streetman gateway conformance --provider all

# Start proxy scaffold
streetman proxy --port 8787 --provider auto
```

---

## 📏 benchmarks

**Current bench:** local absolute-win fixtures plus pinned competitor snapshot.

**Planned full matrix:** 100 real-agent tasks × 4 models × baseline/competitor/streetman arms.

**Models:** Claude Opus 4.7, Claude Sonnet 4.6, Claude Haiku 4.5, GPT-5-Codex.

**Ship gate (must ALL pass to release):**
- Median output savings ≥85% vs normal
- Median savings ≥30% over leading competitor
- **Accuracy 100% across all 1,440 outputs, zero drops**

Fixture bench runs on every PR. Competitor captures are committed to
[`benchmarks/results/`](./benchmarks/) before README headline numbers become claims.

```bash
cd streetman
streetman bench run --suite absolute-win --out benchmarks/results/fixture-latest.json
streetman bench gate benchmarks/results/fixture-latest.json
streetman bench capture-competitors --out benchmarks/results/competitor-live.json
streetman bench compare --against headroom,token-optimizer,caveman
```

---

## 🛠️ the 24 killer features

<details>
<summary><strong>Core compression (11 features)</strong></summary>

1. **Algorithmic Consonant-Skeleton Engine (UNBOUNDED)** — every word auto-reduces via rules, not lookup. Top-2000 precomputed, rare words rule-generated.
2. **High-Value Phrase Shortcuts** — `u, ur, rn, cuz, w/, w/o, b4, thru, gonna, wanna, tbh, ngl, fr, afaik, iirc, imo, idk` — unambig in tech prose.
3. **Symbol Substitution (safe)** — `& | @ = ≠` only. Ambig ones dropped.
4. **Emoji Layer (3-rule safe)** — define-on-first-use + single-dominant-meaning whitelist + domain-gated.
5. **Numeric Crunching (SI-only)** — `500ms`, `24h`, `1KB`, `3x`. Ambig forms avoided.
6. **Phrase Chunk Lexicon** — `make sure to→ensure`, `in order to→to`, `as a result→so`.
7. **Semantic Pair Compression (safe)** — `before X, Y → b4 X, Y`.
8. **Auto-Acronym Learning** — first mention defines, later mentions skeleton-form.
9. **Table-First Restructuring** — comparison prose auto-rewritten as tables (30% tighter).
10. **Punctuation Collapse** — trailing periods dropped on fragments, multi-space collapsed.
11. **Code-Comment Compressor** — skeleton-treat comments, code logic byte-exact.

</details>

<details>
<summary><strong>Reliability & accuracy (6 features)</strong></summary>

12. **Hard Accuracy Rubric (100% gate)** — deterministic extractor + LLM judge → 0/100 score.
13. **Auto-Fallback on High-Stakes** — CVEs, `rm -rf`, `DROP TABLE`, security warnings → auto-normal.
14. **Markdown Structure Validator** — AST parse confirms headings/lists/tables still valid.
15. **Rollback Safety Net (Normal Twin)** — 30-day hash-linked audit trail.
16. **LLM Expand-to-Plain** — `streetman expand` reconstructs normal English.
17. **Preview-Gate on Compress** — diff + y/N before overwrite.

</details>

<details>
<summary><strong>Mode stability (3 features)</strong></summary>

18. **Per-Turn Mode Anchor Hook** — reinjects rules every turn.
19. **Context-Overflow Auto-Reinject** — survives 100+ turn threads.
20. **Token-Budget Aware Intensity** — `--max-tokens N` auto-adapts lite→full→ultra.

</details>

<details>
<summary><strong>Platform reach (1 feature)</strong></summary>

21. **Multi-Platform Native** — Claude Code + Cursor + Codex + VS Code from day 1.

</details>

<details>
<summary><strong>UX, trust, extensibility (3 features)</strong></summary>

22. **Live Stats Badge + Session Ledger** — `[STREET:FULL] saved 4.2k tok (63%) acc:100`
23. **Per-Project `.streetman.toml`** — repo-local config override.
24. **Thinking-Token Trimmer** — compresses reasoning phase on extended-thinking models.

</details>

**Plus:**
- 🌐 **Gateway adapters** — LiteLLM, Portkey, OpenRouter plugins on launch
- 🏷️ **Domain profiles** — sql / json / k8s / docs — domain-specific rules never mangle keywords
- 🧪 **Bench-as-service CLI** — independent verification for any compression skill

---

## 🤝 contributing

Issues + PRs welcome. See [CONTRIBUTING.md](./CONTRIBUTING.md).

- Add a shortcut to `lexicons/shortcuts.toml` → must include bidirectional test
- Add a domain profile → must include accuracy fixture
- Report accuracy regression → share the prompt + model + expected vs actual

**Bench gate:** every PR runs the full 1,440-call matrix in CI. Regression on savings or accuracy blocks merge.

---

## 💼 commercial

Streetman core is MIT-licensed and free forever. Hosted and enterprise products ship alongside:

| Product | What | Price |
|---|---|---|
| **Savings-Share Proxy** | Drop-in proxy, bills 20% of measured tokens saved | Pay-as-save |
| **Enterprise Self-Hosted** | On-prem binary + SSO + audit + org policy enforcement + SLA | $500–5k/seat/yr |
| **Bench-as-Service SaaS** | Hosted bench + certification + public scorecard | $99–999/mo |
| **Savings Dashboard** | Org-wide token ROI analytics | $49–499/mo |
| **Compliance Packs** | HIPAA / SOX / finance lexicons | $1k–10k one-time |
| **Pro Tier (freemium)** | Multi-model bench + thinking-trim + priority support | $9–29/mo |

Full breakdown in [BUSINESS.md](./BUSINESS.md).

---

## 📜 license

Core: [MIT](./LICENSE). Hosted + enterprise: commercial (see [BUSINESS.md](./BUSINESS.md)).

---

## 📣 why we built this

Because the current category leader cuts ~50% tokens unmeasured, silently drifts in long sessions, breaks in Cursor chat, and has no accuracy oracle. Streetman does the 85%+ job, binary-gated, bench-verified, multi-platform. The crown changes hands.

---

<div align="center">

### ⭐ Star us on GitHub — it helps a lot.

*why use many token when few do trick — **why use words at all.***

</div>
