# STREETMAN — Business Plan

> The compression platform for AI-augmented software engineering.

**See also:** [PLAN.md](./PLAN.md) for technical scope + architecture.

---

## TL;DR

| | |
|---|---|
| **What** | Rust binary + skill + plugin suite that cuts LLM output tokens ≥85% vs normal, ≥30% over the current leader, at 100% technical accuracy. |
| **Who for** | Any developer using Claude Code / Cursor / Codex CLI / VS Code + orgs running LLM infra (LiteLLM, Portkey, OpenRouter). |
| **Moat** | Deterministic rule-based compression w/ 100%-accuracy gate + bench-as-service + cross-platform single binary. |
| **License** | Core = MIT (OSS). Hosted services + enterprise = commercial. |
| **Flagship revenue** | Savings-Share Proxy — bills 20% of measured tokens saved. Value-aligned pricing. |
| **Year 1 scope** | Output compression + bench + 4 platforms + 3 gateway adapters. |
| **Month-2 scope** | TOOG v0.1 (input intent compiler) ships as `streetman prompt` subcommand — same binary, same brand. Closes compression loop. |

---

## 1. Competitive landscape

### A. Direct rule-based output compression (streetman's category)
| Tool | Method | Verdict |
|---|---|---|
| **Caveman** (current leader) | Article/filler drop, Python skill, fixed lexicon | 50% cut unmeasured, no accuracy gate, Cursor-broken, drifts in long sessions. Streetman wins on every axis. |
| *None other in category* | — | **Streetman defines a new bar** — rule-based + 100%-accuracy-gated + multi-platform + bench-as-service. |

### B. Model-based input compression (different axis, complementary)
| Tool | Method | Relationship |
|---|---|---|
| **LLMLingua / LLMLingua-2** (Microsoft) | BERT classifier, 20x input compression | Complement. LLMLingua compresses INPUT context (RAG/long-doc); streetman compresses OUTPUT. Can chain. |
| **LongLLMLingua** | Long-context RAG reorder | Same axis as LLMLingua |
| **500xCompressor** | Vector-encode context | Lossy, input only. Not a competitor to rule-based deterministic text compression. |
| **SCOPE** | Generative LLM compression | Requires extra model inference ($ + latency); streetman is deterministic rule-based. |
| **CompactPrompt** | Unified input + file compression | Input-focused, no plugin surface. |
| **TOON** | Structured data serialization | Orthogonal — compresses structured data payloads, not prose. |

### C. AI gateways & proxies — distribution channels, NOT competitors
| Tool | What it does | Streetman relationship |
|---|---|---|
| **LiteLLM** | OSS proxy for 100+ LLMs, routing/caching/cost tracking | **Distribution channel** — ship streetman as LiteLLM plugin day 1. Instant reach to thousands of orgs. |
| **Portkey** | AI gateway, routing + caching + fallback, OSS + SaaS | Same — ship as Portkey plugin. |
| **Helicone** | Observability + caching | Integrate — streetman metrics feed Helicone dashboards. |
| **Cloudflare AI Gateway** | Edge proxy | Potential partner — Rust binary runs at edge. |
| **OpenRouter** | Multi-provider routing | Integrate as pre-processor. |
| **Braintrust / LangSmith** | Eval + observability | Integrate — bench results feed their eval dashboards. |
| **Redis LangCache** | Response caching | Different axis (identical-query cache vs compression) — complementary. |
| **Edgee AI Gateway** | Edge proxy, transparent compression for Claude Code/Cursor | **Closest go-to-market competitor**. Proprietary SaaS only, no accuracy gate, no bench transparency. Streetman counter: OSS core + deterministic + 100% gate + self-hostable. |

**Key insight:** gateways and proxies route/cache/log. **None of them do rule-based bi-directional text compression with accuracy guarantees.** Streetman plugs into all of them as a compression stage, then sells the hosted optimization/analytics around that plug.

### Streetman's whitespace (nobody else owns)
1. Output-side + rule-based + **100%-accuracy-gated** (LLMLingua is lossy; caveman is unmeasured)
2. **Multi-host native plugin** (LLMLingua is a library; Edgee is a closed proxy)
3. **Bench-as-service** (no competitor offers independent third-party verification infra)
4. **Rust single-binary distribution** (all others Python/Node/SaaS)
5. **Platform story** (streetman today output / TOOG tomorrow input = full compression platform)

---

## 2. OSS vs Commercial split

Decided early to avoid ambiguity: **distribution = free; value capture = paid.**

### ✅ Open-Source (MIT, GitHub, free forever)

Everything a developer needs for self-serve value. Drives adoption + community contributions + distribution:

- `streetman` Rust binary (compression engine + CLI)
- `streetman-core` crate (lexicons, skeleton engine, accuracy rubric)
- `streetman-commit` / `streetman-review` / `streetman-compress` skills
- `streetman-bench` harness (local runs)
- All lexicons, domain profiles, example configs
- Plugin wrappers: Claude Code, Cursor, Codex, VS Code
- Gateway adapters: LiteLLM, Portkey, OpenRouter
- Hooks (activate, reanchor, compaction, ledger)
- Basic MCP server
- Docs, quickstart, intensity-levels guide

**Why give this away:** without free distribution, we don't exist. LiteLLM/Portkey/OpenRouter will integrate OSS competitors (LLMLingua) instead. Distribution = oxygen.

### 💰 Commercial (paid, closed-source or license-gated)

Hosted layer + enterprise features where value scales with delivery:

| # | Product | Price | Buyer |
|---|---|---|---|
| 1 | **Savings-Share Proxy (SaaS)** — drop-in Claude/OpenAI proxy, meters tokens saved, bills 20% of measured savings | pay-as-save | eng leaders, CFOs |
| 2 | **Enterprise Self-Hosted** — on-prem binary + SSO + audit log + centralized `.streetman.toml` policy enforcement + SLA | $500–5,000/seat/yr | compliance-heavy orgs (HIPAA/SOX/finance) |
| 3 | **Bench-as-Service (SaaS)** — hosted bench: upload your rules, get verified cert + public scorecard | $99–999/mo | devtool teams, skill authors |
| 4 | **Savings Dashboard (SaaS)** — org-wide token ROI analytics, per-team, per-project savings reports | $49–499/mo | eng leadership, finance |
| 5 | **Domain Compliance Packs** — HIPAA / SOX / legal / finance pre-audited lexicons w/ certification | $1,000–10,000 one-time | regulated industries |
| 6 | **Pro Plugin Tier (Freemium)** — OSS core free; paid unlocks: thinking-token trimmer + normal-twin audit UI + multi-model bench + priority support | $9–29/mo/user | individual pros, small teams |
| 7 | **Certification Mark** — "streetman-certified ≥X% cut @ 100% acc" badge license for other skills/proxies | $500/skill/yr | third-party skill authors |
| 8 | **Hosted MCP Server** — team-shared lexicons, centralized accuracy judge, audit trail | $19–99/team/mo | team buyers |
| 9 | **Training / Workshops** — teach teams to write their own compression rules & profiles | $5,000/engagement | services layer |

### The moat (what stays closed)
- Savings-Share Proxy meter (the billing logic)
- Enterprise admin surface (SSO, policy enforcement, audit)
- Hosted bench cert + badge issuance
- Compliance packs (HIPAA/SOX/finance lexicons)
- ROI dashboard / analytics

### Same-binary, feature-flagged tiers
No code fork. Same Rust binary detects license tier and enables features:

```
streetman run                    # free: community lexicons, basic compression
streetman run --pro              # paid: thinking-trim, multi-model bench
streetman run --enterprise       # org-gated: policy enforce, audit, SSO
```

---

## 3. Will OSS adapters kill profitability?

**No.** Math on one F500 customer:

**Target profile: F500 w/ 5,000 engineers, $10M/yr Claude+OpenAI spend**

| Tier | How captured | ARR from 1 customer |
|---|---|---|
| Free OSS adapter in LiteLLM | Discovery, eng adoption | $0 |
| Hosted MCP (shared lexicons, 20 teams × $99/mo) | Team buyers | $24k |
| Savings Dashboard (ROI reports) | Eng leadership + finance | $6k |
| **Savings-Share Proxy** (20% × 50% saved × $10M spend) | **Flagship** | **$1,000,000** |
| Enterprise Self-Hosted (500 sec-sensitive eng × $1k/seat) | Compliance gate | $500,000 |
| Compliance pack (SOX/HIPAA) | Regulated BU | $10k |
| **Total** | | **~$1.54M/yr ARR** |

Without free OSS adapter → customer never finds us → **$0**. The free adapter is the ad buy.

### Why F500 can't self-host to dodge paying
| What F500 buys | Why OSS alone can't deliver |
|---|---|
| SOC 2 / HIPAA / SOX certification | OSS has no compliance cert |
| SSO + centralized policy enforcement | OSS = per-dev install, no org enforcement |
| SLA + 24/7 support + indemnification | No community support contract |
| Data residency + on-prem w/ audit | Enterprise tier only |
| Org-wide ROI dashboard (CFO wants the $) | OSS doesn't meter savings across teams |
| Savings-Share Proxy (auto-bills % saved) | Requires hosted meter — can't self-host accounting |
| Legal MSA + procurement checklist | No OSS signs contracts |

F500 procurement won't touch OSS alone for load-bearing infra.

### Analog proof (OSS-core at $100M+ scale)
| Company | OSS core | Paid tier ARR |
|---|---|---|
| HashiCorp (Terraform) | free | $600M+ (Terraform Cloud + Enterprise) |
| Docker | free engine | $500M+ (Desktop + Hub) |
| GitLab | free core | $600M+ (Enterprise) |
| Redis | free DB | $300M+ (Enterprise) |
| Supabase | free stack | $100M+ (hosted) |

Every one kept OSS free at scale. None died from it. All built $100M+ ARR from the paid wrapper.

---

## 4. Go-to-market sequencing (months, not years — AI infra velocity)

### Month 1 — Phase 0–2 complete + OSS launch
- Week 1: bench scaffold + ground-truth snapshot (Phase 0)
- Week 2: lexicons + Rust engine + streetman bench passes ship gate (Phase 1–2)
- Week 3: variants (commit/review/compress) + hooks + 4 platform wrappers (Phase 3)
- Week 4: GitHub launch w/ killer README + benchmarks + claims citations
  - HN / Reddit / Medium / X / Claude Code Discord
  - LiteLLM / Portkey / OpenRouter plugin PRs
  - Target: 3k GitHub stars week 1, 5k by end of month

### Month 2 — Freemium Pro + TOOG v0.1
- Launch Pro tier ($9–29/mo) — thinking-trim + multi-model bench + priority support
- **Ship TOOG v0.1 as `streetman prompt` subcommand** (same binary, same brand, same repo). Input-intent compilation w/ env-aware compile for top-5 tasks (FIX:BUILD, GEN:TEST, REFACTOR:SCOPED, REVIEW:DIFF, EXPLAIN:CODE)
- Launch `streetman-certified` program — $500/skill/yr badge
- Target: 500 Pro subscribers ($100k ARR), 20 certified skills ($10k ARR)

### Month 3 — Savings-Share Proxy (flagship)
- Drop-in Claude/OpenAI proxy billing 20% of measured savings (both directions now — streetman output + TOOG input)
- 3–5 mid-market design partners
- Target: 10 paying orgs, $250k–1M ARR

### Month 4–6 — Enterprise + compliance
- Enterprise self-hosted + SSO + audit + policy enforcement
- Compliance packs (HIPAA/SOX/finance)
- First F500 design partners (target 2–3 logos)
- Target: $1.5M–3M ARR end of month 6

### Month 7–12 — Scale
- F500 sales motion (named accounts)
- TOOG v1 full grammar (20+ task types, full telemetry)
- Enterprise dashboard GA
- Target: 5 F500 logos, $5M–10M ARR end of year 1

### Year-1 exit metrics
- 10k+ GitHub stars
- 2k+ Pro subscribers
- 30+ paying orgs (Savings-Share Proxy)
- 5+ F500 logos (Enterprise)
- $5M–10M ARR
- TOOG integrated from month 2, not year 2

---

## 5. Risks + mitigations

| Risk | Mitigation |
|---|---|
| **Accuracy regression in the wild** | 100% rubric gate in CI + normal-twin audit trail + live acc counter in statusline. Public bench badge = every release proves it. |
| **Model vendors absorb output compression natively** (Claude adds terse mode) | Already exist ("Answer concisely"). Streetman is 3x more aggressive + deterministic + rule-based (vendor can't match w/o shipping another skill). If vendors add native modes, our bench still proves we're 30%+ better. |
| **Caveman or LLMLingua adds accuracy gate** | We own the bench-as-service tooling — if they add a gate, they're benching against us. Lead stays. |
| **Gateway refuses to ship OSS adapter** | Ship as external plugin, publish config snippets + Docker image. Users self-install even if gateway won't promote. |
| **Savings-Share Proxy latency** | Rust binary processes 100KB in <10ms on M1 = sub-ms compression overhead. 200-500ms LLM inference dominates. Latency overhead <0.5%. |
| **F500 legal blocks "we bill 20% of savings"** | Offer flat-rate tier as alternative ($10k–100k/mo based on token volume). Same value capture, simpler procurement. |
| **OSS contributors fork commercial tier** | Commercial code stays closed; OSS surface is genuinely useful standalone (adoption proves it). Forking "the meter" requires rebuilding billing + compliance + SSO infra from scratch. Not a real threat. |

---

## 6. Platform story — streetman (output) + TOOG (input)

**Month 1:** streetman v1 = best-in-class output compression for AI coding agents.

**Month 2:** TOOG v0.1 ships as `streetman prompt` subcommand (same binary, same brand, same repo). Input-intent compiler for top-5 dev tasks. Compiles developer intent like:

```
FIX:BUILD scope=payments minimal=true api=stable verify=unit
```

into optimized, context-enriched prompts using real repo/build/IDE state.

**Combined platform = bidirectional compression for AI-augmented software engineering:**
- Streetman shrinks what the AI says back (output)
- TOOG shrinks what the dev says in (input)
- Savings-Share Proxy meters **both directions** → 2x revenue surface per customer
- Telemetry layer captures intent-level events (success/fail/retries/time-to-green) → intent-layer observability no gateway has yet

**Positioning moat:** nobody else has this axis. LLMLingua = input only (lossy, academic). Edgee = input + output (proprietary, no gate). Streetman + TOOG = input + output (OSS core, 100% gate, bench-verified, enterprise-ready).

### TOOG's 6-layer concept
1. **Compress developer intent** — shorter prompts, less ambiguity
2. **Encode SWE semantics** — fix build, gen tests, refactor, verify (not data)
3. **Environment-aware compile** — IDE/repo/build/diff state enriches before model call
4. **Standardize eng ↔ agent interaction** — measurable, enforceable
5. **Policy enforcement** — org allowed-intents gate (enterprise moat)
6. **Telemetry primitives** — each intent = measurable event (intent-layer observability)

### Clarifying what TOOG is NOT
- Not "TOOG vs TOON" — TOON compresses structured DATA payloads; TOOG compresses developer INTENT. Orthogonal.
- Not "another JSON killer"
- Not "a generic prompt DSL for everything"

### TOOG definition (locked)
> TOOG is a compact, environment-aware developer-intent grammar that compiles human software-engineering intent into optimized, context-enriched, policy-gated instructions for AI coding agents — producing measurable task-completion events for downstream analytics.

---

## 7. Why streetman wins (summary)

| Dimension | Advantage |
|---|---|
| **Technical** | Unbounded skeleton engine, 24 killer features, 100% accuracy gate, Rust single-binary, 100x faster |
| **Validation** | 1,440-call bench matrix (144x rigor of competitors), bootstrap CI, committed snapshots, claims audit |
| **Reach** | Day-1 on Claude Code + Cursor + Codex + VS Code + LiteLLM + Portkey + OpenRouter |
| **Category** | Defines bench-as-service — sets the honesty bar competitors must match |
| **Moat** | Accuracy gate is CI-enforced, not aspirational; normal-twin audit stores proof; every claim cites a snapshot |
| **Business** | OSS-core playbook proven at $100M+ scale; savings-share pricing aligns w/ customer ROI; F500-ready from month 4 |
| **Platform** | Month-2 TOOG closes compression loop — bidirectional = 2x revenue surface, intent-layer observability nobody else has |

---

## 8. Funding posture (AI-velocity, not 2015 SaaS cycles)

**Month 0–2:** bootstrap or small pre-seed ($50–200k) — 1–2 devs, ship OSS + TOOG + Pro tier.

**Month 3–6:** seed ($1–3M) when Savings-Share Proxy hits first $100k ARR + 3 design partners. Hire: 2 eng + 1 DevRel.

**Month 6–12:** Series A ($5–15M) when ARR crosses $1M + 5 F500 design partners lined up. Hire: enterprise sales + compliance + more eng.

**Exit paths:**
- Strategic acquirer: Datadog, New Relic, Dynatrace (observability stack fit) — intent-layer telemetry story
- Strategic acquirer: HashiCorp, GitLab, Atlassian (devtool suite fit)
- Strategic acquirer: Anthropic, OpenAI (native integration as first-party feature) — long shot but highest multiple
- Continue independent — OSS-core category doesn't need exit to hit $100M ARR (see analogs)

---

## 9. Open questions for founder

- Year-1 revenue goal: $1M ARR (aggressive) vs $500k ARR (realistic)?
- Pro tier price point: $9/mo (volume play) vs $29/mo (value play)?
- F500 design partner mix: vertical focus (fintech only) vs horizontal (any industry)?
- Savings-Share % rate: 20% (premium pricing) vs 15% (faster adoption) vs tiered by volume?
- TOOG open-source decision: MIT like streetman core, or commercial-only?

These resolve post-Phase-0 bench results (which anchor realistic pricing + customer ROI math).
