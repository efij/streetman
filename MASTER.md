# STREETMAN + TOOG + DEVBOOSTER — Master Strategic Plan

> The crystal-clear, evidence-backed, patent-aware plan.
> Read this first. Everything else (PLAN.md, BUSINESS.md, README.md) supports this.

---

## TL;DR — the 3-layer architecture

There are **three distinct products** under one umbrella. They are NOT the same thing. Conflating them is why the earlier docs were confusing.

| Layer | Product | License | What it does | Who owns it |
|---|---|---|---|---|
| **L1** | **streetman** | OSS (MIT) | Compresses AI **output** prose (what the model says back). Rust binary. Self-contained. | Public — drives adoption, GitHub flywheel. |
| **L2** | **TOOG** | Spec = OSS (CC-BY). Compiler = closed + patent-pending. | Compresses developer **input** intent. Task grammar parsed + repo/build/IDE-aware compiled into optimized prompts. | DevBooster Inc. — spec public for adoption, compiler closed for moat. |
| **L3** | **DevBooster** | Closed commercial | IDE extension suite (VSCode / Cursor / VS2022 / Codex). Orchestrates streetman + TOOG + verification loop + telemetry + patch execution + benchmarks. | DevBooster Inc. — the real company. |

**Positioning one-liners (memorize):**
- *Streetman compresses what the AI says back.*
- *TOOG compresses what the dev says in.*
- *DevBooster runs the whole loop — intent → context → compile → execute → verify → measure.*

---

## Why 3 layers, not 1

Three separate products because each solves a different problem, has different buyers, and has different IP strategy:

| Axis | Streetman | TOOG | DevBooster |
|---|---|---|---|
| **Direction** | AI → human (output prose) | Human → AI (input intent) | Full loop orchestration |
| **Artifact** | Compressed text | Compiled prompt w/ repo context | Applied patch + verified build |
| **License** | MIT, free forever | Spec public, compiler closed | Closed commercial |
| **Patent** | None (lexicon + rules = prior art) | **YES** — compilation method, verification loop | **YES** — IDE/build-aware optimization loop, measurement-driven prompt opt |
| **Buyer** | Any developer using LLMs | Orgs wanting reproducible agent behavior | Engineering teams wanting measured ROI |
| **Revenue** | $0 (adoption wedge) | $0 direct (drives DevBooster adoption) | **$$$ — all revenue** |

---

## The patent layer (what's actually protectable)

**Do NOT patent:**
- "A compression skill for LLM output" → prior art (caveman, LLMLingua)
- "A prompt DSL" → too generic, prior art exists
- "Short keywords for prompts" → too trivial

**DO patent (provisional filing priority):**

### Patent 1 — Context-Aware Developer-Intent Compilation
> *System and method for translating structured software-engineering intent into context-aware, policy-constrained, verifiable AI-agent actions using repository, build, and IDE metadata.*

Covers:
- TOOG AST parsing
- Context resolver (repo graph, build logs, changed files, project refs)
- Context minimizer (select only relevant lines/files/dependencies)
- Policy validator (allowed action? blast radius? forbidden paths?)
- Prompt generator (constraint-injected, ambiguity-removed)
- Execution-mode selector (suggest / patch / patch+verify / PR)

### Patent 2 — IDE/Build-Aware Optimization Loop
> *Method for fusing IDE settings, repository configuration, build state, and test results into AI-assisted code-modification recommendations with verification and rollback.*

Covers:
- Findings engine (detects optimizable configs: `.gitignore`, `Directory.Build.props`, `.vscode/settings.json`)
- Safe-patch generator w/ idempotency guarantees
- Build/test post-verification loop
- Rollback mechanism
- Cost/perf before-after benchmarking integrated w/ IDE

### Patent 3 — Measurement-Driven Prompt Optimization
> *Method for adaptive optimization of AI-agent prompts based on observed retry counts, token consumption, build-verification outcomes, and task-success metrics across a software engineering workflow.*

Covers:
- Closed-loop telemetry (task event → metrics → prompt adaptation)
- Success-correlated prompt template evolution
- Per-repo, per-dev, per-task fingerprinting

**IP strategy:**
1. File provisional on all 3 patents BEFORE public TOOG spec release (priority date locked)
2. TOOG spec (syntax + AST format) → public OSS — drives adoption, becomes de facto standard
3. TOOG compiler + context resolver + verification loop → closed proprietary (DevBooster Inc.)
4. Streetman stays MIT — no patent needed, adoption is the goal
5. Defensive publication for streetman rules (prevent competitors patenting)

---

## Hardcore evidence plan — every claim cites a bench

No claim ships without a reproducible bench snapshot. Pre-commit CI rejects hand-edited numbers.

### Evidence Bench 1 — Streetman output compression
**Claim:** "Cuts 85% output tokens vs normal, ≥30% over leader, 100% accuracy"
**Method:** 30 prompts × 4 models × 4 arms × 3 trials = 1,440 calls
**Metrics:** output tokens, % savings vs terse control, 95% CI, accuracy 0/100
**Snapshot:** `benchmarks/results/bench-streetman-YYYYMMDD.json`
**Gate:** ≥85% savings vs normal AND ≥30% over leader AND 100% accuracy
**Cost:** ~$20 per full run

### Evidence Bench 2 — TOOG intent compression
**Claim:** "Cuts 25–50% input tokens vs natural-language prompts, reduces retries 30–50%"
**Method:** 50 dev tasks × 4 models × 3 arms (NL, terse-NL, TOOG) × 5 trials = 3,000 calls
**Metrics:** input tokens, output tokens, retry count, task success rate, time-to-green
**Arms:**
- `__nl__` — natural language prompt
- `__nl-terse__` — concise NL prompt
- `__toog__` — TOOG compiled via DevCore
**Snapshot:** `benchmarks/results/bench-toog-YYYYMMDD.json`
**Gate:** ≥25% input-token reduction AND ≥30% retry reduction AND ≥task-success parity
**Cost:** ~$100 per full run

### Evidence Bench 3 — Full-loop DevBooster (intent → verify)
**Claim:** "20–45% effective total token reduction per task when retries + verification included"
**Method:** 20 real-repo tasks × 4 models × 2 flows (raw Cursor vs DevBooster) × 5 trials = 800 runs
**Metrics:** end-to-end tokens, wall-clock time, retries, first-attempt success, final build+test pass rate
**Snapshot:** `benchmarks/results/bench-devbooster-e2e-YYYYMMDD.json`
**Gate:** ≥20% total-flow token reduction AND ≥30% first-attempt success improvement
**Cost:** ~$200 per full run

### Evidence Bench 4 — Customer ROI study
**Claim:** "$X saved per developer per month on real org workloads"
**Method:** 3 design-partner orgs, 30-day before-after study, real prompts + real commits
**Metrics:** actual LLM spend, dev time saved, feature-delivery velocity, retry rate
**Snapshot:** `benchmarks/results/roi-study-YYYYMMDD.md` (anonymized)
**Gate:** ≥10% measurable LLM cost reduction per design partner
**Cost:** $0 (customer-funded beta)

### Evidence Bench 5 — Accuracy-regression fixtures (always-on)
**Claim:** "Streetman never drops a technical identifier"
**Method:** 500+ fixture pairs (normal answer → streetman answer), every PR runs all fixtures
**Metrics:** claim-extraction coverage (must be 100% per fixture)
**Snapshot:** `tests/accuracy/fixtures/*.json` + CI output
**Gate:** 100% on every fixture, every commit
**Cost:** $0 (regex + cached semantic judge)

---

## OSS vs Commercial — HARDCORE CLEAR SPLIT

### ✅ OSS — free on GitHub, MIT or CC-BY license

**Streetman (all of it):**
- Rust binary (`streetman` CLI)
- `crates/streetman-core` (lexicons, skeleton engine, accuracy rubric)
- `crates/streetman-bench` (harness — runs locally with your API keys)
- All lexicons (shortcuts, symbols, phrases, numerics, emojis)
- All domain profiles (sql, json, k8s, docs)
- `skills/streetman-commit`, `-review`, `-compress` (Claude Code/Cursor/Codex/VS Code skills)
- Plugin wrappers (4 hosts)
- Gateway adapters (LiteLLM, Portkey, OpenRouter)
- Hooks (activate, reanchor, compaction, ledger)
- Basic MCP server
- Documentation

**TOOG (spec only):**
- `docs/toog-spec-v0.1.md` — syntax + grammar + AST format (CC-BY so it becomes a standard)
- Reference parser (MIT) — parses TOOG text → AST JSON (NOT the compiler)
- Grammar test fixtures

**Why give this away:**
- Distribution = oxygen. Without free OSS, LiteLLM/Portkey will integrate OSS competitors (LLMLingua) instead.
- TOOG-as-standard captures the category. Open syntax, closed compiler = textbook platform strategy (HTTP is open; Cloudflare isn't).
- Every install is an ad for the paid layer.

### 💰 Closed commercial — where the money is

**TOOG Compiler (patent-protected, closed-source):**
- Context resolver (repo graph traversal, build-log ingestion, IDE-state reader)
- Context minimizer (relevance-ranked file/line extraction)
- Policy validator (blast-radius enforcement, forbidden-path detection)
- Prompt generator (constraint-injection templates)
- Execution-mode selector

**DevCore runtime (closed):**
- Executor (applies patches to temp workspace)
- Verifier (runs build + tests, compares API surface)
- Telemetry recorder (tokens, retries, success, latency per task event)
- Rollback engine

**DevBooster IDE extensions (closed):**
- VSCode extension
- Cursor integration
- Visual Studio 2022 extension
- Codex CLI wrapper
- Findings panel, Apply panel, Benchmarks panel, AI Efficiency panel

**Hosted services:**
- Savings-Share Proxy (the meter — bills 20% of measured tokens saved)
- Enterprise self-hosted (SSO, audit, policy enforcement, SLA)
- Bench-as-service SaaS
- Savings Dashboard SaaS
- Hosted MCP server (team-shared lexicons + centralized accuracy judge)

**Paid add-ons:**
- Compliance packs (HIPAA, SOX, finance, legal — pre-audited lexicons + TOOG policies)
- Pro Plugin tier (thinking-token trimmer, multi-model bench, priority support)
- Certification Mark license ($500/skill/yr for third-party skills)

### Same binary, feature-flagged tiers
No code fork. Binary detects license tier at runtime:
```
streetman run                    # free: OSS lexicons, basic compression
streetman run --pro              # paid: thinking-trim, multi-model bench
streetman run --enterprise       # org-gated: policy enforce, audit, SSO
devbooster run                   # closed binary (separate repo): full TOOG compiler + verifier + telemetry
```

---

## Revenue model — crystal clear

**Streetman = $0 revenue.** Distribution only. Every install is a lead.
**TOOG spec = $0 revenue.** Standard-setting. Every adopter is a lead.
**DevBooster = 100% of revenue.** All monetization lives here.

### DevBooster revenue streams (ranked by scale)

| # | Product | Unit price | Target scale | Month to ship |
|---|---|---|---|---|
| 1 | Savings-Share Proxy | 20% of measured savings | $1–5M ARR per F500 | Month 3 |
| 2 | Enterprise Self-Hosted | $500–5k/seat/yr | $500k–5M per F500 | Month 4 |
| 3 | Pro Tier (freemium) | $9–29/mo/user | $500k–2M ARR | Month 2 |
| 4 | Compliance Packs | $1k–10k one-time | $100k–500k ARR | Month 5 |
| 5 | Bench-as-Service SaaS | $99–999/mo | $100k–500k ARR | Month 3 |
| 6 | Savings Dashboard SaaS | $49–499/mo | $100k–500k ARR | Month 4 |
| 7 | Hosted MCP | $19–99/team/mo | $50k–200k ARR | Month 2 |
| 8 | Cert Mark License | $500/skill/yr | $25k–100k ARR | Month 3 |

### Per-F500 math (validates thesis)
Target: F500, 5,000 engineers, $10M/yr Claude+OpenAI spend.

| Tier | Capture | ARR from 1 F500 |
|---|---|---|
| Free OSS (streetman + TOOG spec) | Discovery | $0 |
| DevBooster Pro ($19/mo × 500 power users) | Individual pros | $114k |
| Hosted MCP (20 teams × $99/mo) | Team buyers | $24k |
| Savings Dashboard | Eng leadership | $6k |
| **Savings-Share Proxy** (20% × 50% saved × $10M) | **Flagship** | **$1,000,000** |
| Enterprise Self-Hosted (500 sec-sensitive × $1k/seat) | Compliance | $500,000 |
| Compliance Pack (SOX + HIPAA) | Regulated BU | $20k |
| **Total per F500** | | **~$1.66M ARR** |

20 F500s = **$33M ARR.** That's the company.

---

## Month-by-month roadmap — AI velocity

### Month 0 (now) — Planning + patent prep
- ✅ Strategic plan locked (this doc)
- Provisional patent drafting (3 patents above) — **CRITICAL: file before public release**
- Design partner outreach begins (2–3 mid-market + 1 F500 warm intro)

### Month 1 — Streetman v1 OSS launch + patent filings
- Week 1: Phase 0 bench scaffold → commit ground-truth snapshot
- Week 2: Phase 1 lexicons + skeleton engine → bench gate passes
- Week 3: Phase 2 bench streetman → accuracy gate 100% → Phase 3 variants + hooks + 4 platforms
- Week 4: GitHub launch + HN + Reddit + LiteLLM/Portkey/OpenRouter PRs
- **Patent filings submitted end of week 2** (BEFORE public release)
- Target: 3k GitHub stars by end of month

### Month 2 — TOOG v0.1 spec + compiler + DevBooster beta
- Week 1: TOOG spec v0.1 public release (CC-BY) + reference parser (MIT)
- Week 2: TOOG compiler v0.1 closed-source (DevBooster Inc.) — top-5 tasks (FIX:BUILD, GEN:TEST, REFACTOR:SCOPED, REVIEW:DIFF, EXPLAIN:CODE)
- Week 3: DevBooster VSCode extension v0.1 beta — findings panel + apply patches + TOOG command palette
- Week 4: Pro Tier launch ($19/mo), Hosted MCP launch ($49/mo team), Bench-as-Service beta
- Target: 500 Pro subscribers, 3 design partners signed

### Month 3 — Savings-Share Proxy + VS2022 extension
- Savings-Share Proxy beta — 3–5 anchor design partners
- Visual Studio 2022 extension (enterprise wedge)
- Cursor + Codex CLI wrappers full parity
- Cert Mark program launches
- Target: $250k–1M ARR, 10 paying orgs

### Month 4 — Enterprise tier + compliance pack v1
- Enterprise Self-Hosted binary (SSO, audit, policy)
- HIPAA compliance pack
- SOX compliance pack
- Savings Dashboard GA
- Target: 2 F500 design partners signed, $1M ARR

### Month 5–6 — Scale design-partner motion
- SOC 2 Type 1 audit passes
- Additional compliance packs (finance, legal)
- TOOG v0.5 — full grammar (20+ tasks)
- Target: 5 F500 design partners, $3M ARR

### Month 7–12 — F500 sales motion + Series A
- F500 named-account sales motion
- TOOG v1 GA w/ full telemetry + adaptive optimization (Patent 3 live)
- Series A raise $5–15M
- Target: 10 F500 logos, $10M+ ARR end of year 1

---

## Risks + mitigations

| Risk | Mitigation |
|---|---|
| Competitor clones streetman | Accuracy-gate moat + bench-as-service authority. Clones can't claim better savings w/o our bench verifying. |
| Someone files TOOG-adjacent patent first | **File provisional TOMORROW**. Pre-release priority date = moat. |
| OSS community forks commercial tier | Commercial tier requires hosted meter + compliance + SSO infra — not forkable. Same playbook as HashiCorp/GitLab. |
| F500 legal blocks "bill 20% of savings" | Offer flat-rate tier alternative ($10k–100k/mo based on token volume). |
| Anthropic/OpenAI ship native compression | Streetman's bench proves we're ≥30% better. Plus TOOG + DevBooster sit above the model — can't be obsoleted by model vendors. |
| Accuracy regression in the wild | 100% rubric CI gate + normal-twin audit + live accuracy counter. Every release proves it. |
| TOOG adoption too slow | Streetman's OSS momentum pulls TOOG along. DevBooster IDE extensions make TOOG usage invisible to end dev (UI generates TOOG under the hood). |
| Patent rejections | File 3 provisional patents — if 1 rejected, 2 still cover the moat. Plus defensive publication for rest. |

---

## Why this wins (one-page summary)

1. **Three-layer architecture** — OSS wedge (streetman) + open standard (TOOG spec) + closed product (DevBooster). Textbook platform play.
2. **Patent moat** — 3 provisional patents on compilation + verification loop + measurement-driven optimization. Filed before public release.
3. **Evidence-backed** — 5 reproducible benches covering every claim. Pre-commit CI rejects hand-edited numbers.
4. **Revenue concentrated in closed layer** — streetman = $0 (distribution), TOOG spec = $0 (standard), DevBooster = 100% of revenue.
5. **AI velocity** — ships month 1 (streetman), month 2 (TOOG + DevBooster beta), month 3 (Savings-Share Proxy), F500-ready month 4.
6. **$33M ARR at 20 F500s** — per-customer math validated. No fantasy numbers.
7. **IP strategy aligned with open-core playbook** — HashiCorp, GitLab, Docker, Redis all hit $100M+ ARR with this exact split.
8. **Positioning defensible** — TOOG compresses intent; TOON compresses data; streetman compresses output. Orthogonal to all.

---

## Immediate next actions (this week)

1. **Lock this plan** — user confirms MASTER.md is the source of truth. PLAN/BUSINESS/README updated to match.
2. **Start provisional patent draft** — 3 patents above. Use a patent lawyer. $3–5k/patent filing cost.
3. **Phase 0 streetman bench** — fire $20 bench run, commit ground-truth snapshot.
4. **Design partner outreach** — 5 warm intros for Savings-Share Proxy beta.
5. **TOOG spec v0.1 draft** — public-facing grammar doc (can start in parallel with streetman).

Once user confirms MASTER.md, existing PLAN.md and BUSINESS.md get updated to reference this as the root doc.
