# Plan: streetman Competitive Landscape 2026 — COMPETITIVE.md

## Status
Plan-mode blocker prevents writing `/Users/efi.jeremiah/projects/streetman/COMPETITIVE.md` directly. Research is complete; the full deliverable is drafted below. On plan-exit approval, a single `Write` call copies the section titled "FINAL DELIVERABLE" verbatim to `/Users/efi.jeremiah/projects/streetman/COMPETITIVE.md`.

## Research log (condensed)
All entries below are verified via WebSearch / WebFetch on 2026-04-21. Source URLs are embedded inline in the deliverable's Sources section. Source disagreements are flagged with `[?]`.

Key findings:
- The "rule-based output compression" niche is far more crowded than the streetman README implies. **Caveman (40.8k stars) is not a sleeper — it is the market leader** and already ships multi-IDE with a compression-as-skill model.
- **RTK (31k stars)** and **Edgee (54 stars, Apache-2.0 Rust gateway built on RTK)** both focus on *input* (tool output) compression in Rust, not prose output. This is adjacent, not competitive.
- **LeanCTX (Rust, MIT, github.com/yvgude/lean-ctx)** is the closest direct-architecture match (Rust single binary, AST-aware, multi-IDE). 91.4% avg compression claim.
- Academic prompt compression (LLMLingua/LLMLingua-2, LongLLMLingua, 500xCompressor, SCOPE, CompactPrompt) is now industrialized via **The Token Company (YC W26)** and **Compresr (YC W26)** — both are ML-model-based drop-in APIs for *input* compression. These are the scariest sleeper threats for DevBooster's proxy monetization model.
- Native vendor caching (Anthropic 90% discount, OpenAI 90%, Gemini 90% implicit) already covers ~80% of the savings that streetman can claim on *input* tokens. Streetman's output-compression lane is largely protected from vendor caching, but its `streetman prompt` (TOOG) lane is directly attacked.
- TOON (24k stars) is not a compressor — it's a format. It competes with TOOG on the "input encoding" axis.
- No competitor offers "bench-as-service" independent verification. This moat is real.
- No competitor charges on measured savings. The **20% Savings-Share Proxy** model is genuinely unclaimed.
- Enterprise vertical compression (HIPAA/SOX/finance lexicons) is unclaimed whitespace.

---

# FINAL DELIVERABLE (copy verbatim to /Users/efi.jeremiah/projects/streetman/COMPETITIVE.md)

```markdown
# COMPETITIVE LANDSCAPE 2026
Prepared: 2026-04-21 · Scope: streetman (OSS Rust) + DevBooster (commercial) · Author: internal research

> TL;DR — The "rule-based LLM output compressor" niche streetman targets is already contested by **caveman (40.8k stars)**, the reigning Claude Code skill. The "input compressor" niche is being industrialized fast by two YC W26 companies (**Token Company**, **Compresr**) using ML models, not rules. The "Rust gateway" niche is dominated by **RTK/Edgee** — but they compress *tool outputs*, not *prose*. Streetman's three genuine moats post-research: (1) bench-as-service as an independent verification standard, (2) savings-share pricing, (3) bi-directional (input via TOOG + output) in one binary. Everything else — consonant-skeleton, 100% accuracy rubric, multi-IDE — is a narrow, defensible-but-not-unique lead of 6–12 months. Brutal honesty: if streetman ships without bench-as-service and savings-share by Q3-2026, it becomes a caveman fork.

## Table of contents
1. Direct competitors (rule-based output compression)
2. Adjacent (gateways, proxies, observability)
3. Academic / research
4. Sleeper threats
5. Native vendor features (Anthropic/OpenAI/Google)
6. Coding agents (Aider/Cline/Continue/Roo) — context-management competitors
7. Prompt-programming frameworks (DSPy/BAML/LMQL/Guidance/Priompt) — TOOG threats
8. Cursor / Windsurf / Zed / Lovable / v0 native compression
9. Category maps
10. Whitespace analysis
11. Streetman's defensible moats (post-research)
12. Attack surface
13. Sources

---

## 1. Direct competitors — rule-based / deterministic output compression

| # | Name | Repo / Site | Stars | License | Arch | In/Out | Method | Claimed savings | Accuracy claim | Host integrations | Providers | Stack | Pricing | Funding | Team | Key diff | Weakness | Community |
|---|---|---|---:|---|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| 1 | **Caveman** | github.com/JuliusBrussee/caveman | 40.8k | MIT | Skill + plugin + npx | Output (prose), optional input | Rule-based (caveman grammar, drop articles/fillers) | 65–75% output, 46% input per session, 22–87% across 10 bench tasks | "full technical accuracy" — not independently verified | Claude Code, Codex, Gemini CLI, Cursor, Windsurf, Cline, Copilot, 40+ agents | Any (client-side rules) | Markdown skills, npm skills CLI, shell | Free (MIT) | Bootstrap / solo dev | 1 (Julius Brussee) + contributors | Distribution velocity, skill marketplace footprint, 文言文 mode | No independent accuracy oracle, no Rust binary, no enterprise story, no savings dashboard | v1.6.0 April 2026, massive traction, ClaudePluginHub top entry |
| 2 | **streetman** (self) | github.com/yourorg/streetman | n/a (pre-launch) | MIT | Rust binary + plugin + MCP | **Both** (compress output; `streetman prompt`/TOOG for input) | Rule-based consonant-skeleton + lexicons + guards | ≥85% output median, gate ≥30% vs "leading competitor", 100% accuracy | 100% accuracy bench-gated across 1,440 calls (4 models × 30 prompts × 4 arms × 3 trials) | Claude Code, Cursor, Codex, VS Code + LiteLLM/Portkey/OpenRouter adapters | Any OpenAI-compatible | Rust 1.85+, tree-sitter, deterministic regex + LLM judge | MIT core; 20% savings-share proxy; $500–5k/seat/yr enterprise | TBD | TBD | Bench-as-service CLI, bi-directional, Rust binary, accuracy rubric, savings-share billing | Pre-launch, 0 stars, against 40k-star incumbent | TBD |
| 3 | **LeanCTX** | leanctx.com · github.com/yvgude/lean-ctx | [?] not displayed | MIT | Rust single binary + TUI | Input-heavy (AST-aware context compression) | Rule-based (tree-sitter 18 langs + shell pattern 90+ + protocols CEP/CCP/TDD) | 60–99% per file, 91.4% avg, 98.6% example (342→48 tokens) | "code understanding preserved" — not 3rd-party verified | Cursor, Claude Code, Copilot, Windsurf, Neovim | Anthropic, OpenAI, Ollama, DeepSeek, Groq, OpenRouter, LM Studio | Rust, tree-sitter, Ratatui | Free beta; paid cloud TBD | Bootstrap [?] | Unknown | AST-aware, 18 langs, TUI, 3 protocols, most architecturally similar to streetman | Input-only, no output-prose compression, no formal accuracy oracle, no plugin marketplace | Small, but growing on crates.io |
| 4 | **Claw Compactor** | github.com/open-compress/claw-compactor | 2.2k | MIT | Python lib + CLI | Input | 14-stage pipeline (KV-opt, RLE, semantic dedup, log-fold, AST, ML token-opt, abbrev) | 36.3% weighted avg; JSON 81.9%, logs 24.1%, Python 25% | "content-aware" — claimed superior to LLMLingua-2 on structured data | Used by OpenClaw platform | Any | Python 3.9+, optional tiktoken/tree-sitter | Free MIT | Bootstrap | Unknown | Content-type-aware 14-stage pipeline | Python (slow vs Rust), input-only, modest savings, no plugin ecosystem | v7.1.0 March 2026, steady |
| 5 | **Headroom** | github.com/chopratejas/headroom | 1.5k | Apache-2.0 | Python lib + middleware + proxy | Input | Hybrid (SmartCrusher JSON, CodeCompressor AST, Kompress-base HF model, CacheAligner, CCR reversible) | 87% needle-in-haystack, 47–92% real workloads | GSM8K 0.87→0.87, TruthfulQA +0.03, SQuAD/BFCL ~97% | LiteLLM (100+ models), Python/TS libs, MCP | Anthropic, OpenAI, Google, Bedrock, Vertex, Azure, OpenRouter | Python + small HF model | Free | Bootstrap | Unknown | Reversible compression (CCR), cache-aligning, HF model trained on agentic traces | Python, model-based (added latency), input-only | v0.7.1 April 2026 |
| 6 | **LLM Token Saver RS** | github.com/snailer-team/llm-token-saver-rs | 1 | Apache-2.0 | Rust lib | Input | Rule-based (tiers 1–5, budget, selective filter) | 30–40% | "response quality maintained" | None integrated yet | Any | Rust | Free alpha | Bootstrap (snailer.ai, 6k downloads) | Unknown | Rust + snailer.ai production validation | 1 star, alpha, no adapters | Negligible |
| 7 | **compression-prompt** | crates.io/crates/compression-prompt · docs.rs | [?] not shown | Unclear | Rust crate | Input | Statistical IDF filtering | 50% tokens with 91% quality retention (Claude Sonnet); <1ms; 10.58MB/s | Validated on 6 flagship LLMs w/ 350+ test pairs | None (library) | Any | Pure Rust | Free | Bootstrap | Unknown | Pure statistical, no external models, <1ms speed | No plugin, no IDE integration, library-only | Small |
| 8 | **Token Company** | thetokencompany.com · YC W26 | n/a | Proprietary | Drop-in API | Input | ML model (not generative) — removes low-significance tokens | 66% input cut with +1.1% accuracy; arena study +5% purchase volume, 37% latency improvement | Benchmarked internally; external "blind arena" cited but not peer-reviewed | Drop-in | Any (proxy) | Proprietary ML | Unknown; YC W26 | YC W26 | 2 (YC) + team | YC pedigree, ML-model-based (not rules), peer-reviewed founders | Closed-source, no IDE skill, input-only | Fresh launch 2026 |
| 9 | **Compresr** | compresr.ai · github.com/Compresr-ai/Context-Gateway · YC W26 | n/a | Open-source proxy + closed SDK | Drop-in SDK + proxy | Input | ML model compression + preemptive summarization | "100x compression" claim | Not independently verified | Claude Code, OpenClaw, OpenCode, Bedrock | Any | Proprietary ML + open proxy | Unknown; YC W26 | YC W26 | 3 (EPFL, ex-Bell Labs, ex-MS, ex-Philips) | EPFL PhD-led, preemptive background compaction, agentic proxy | Closed SDK, input-only, no output-prose | Fresh launch 2026 |

**Direct-competitor verdict:** Caveman is the elephant in the room. Streetman's README claims "leading competitor cuts ~50% unmeasured, drifts in long sessions" — this matches caveman. But caveman has 40,800 stars, multi-IDE reach, 文言文 mode, and a skills marketplace. Streetman's 85% vs 65–75% delta is real, but the *distribution gap* is two orders of magnitude. The per-turn mode anchor hook (#18) and context-overflow auto-reinject (#19) are genuine technical wins against caveman drift — but you have to ship them before caveman ships v2.

---

## 2. Adjacent — gateways, proxies, observability

| # | Name | Site | Stars / scale | License | Primary function | Compression feature? | Compression method | Providers | Pricing | Funding |
|---|---|---|---:|---|---|---|---|---|---|---|
| 1 | **LiteLLM** | github.com/BerriAI/litellm | 44k / 19,700+ projects | Open (dual) | Unified 100+ LLM gateway / proxy | **No native compression**; adapter point for 3rd parties | — | 100+ providers | Free + Enterprise (undisclosed) | YC W23 |
| 2 | **Portkey** | portkey.ai | 10.2k | Commercial SaaS | AI gateway + observability + prompt mgmt | Intelligent caching, batching, routing — **no semantic compression** | Cache-based | 1,600+ LLMs | Demo-gated | Disclosed backed by industry investors |
| 3 | **Helicone** | helicone.ai | n/a | SaaS + open-source | Observability, routing | **None** | — | OpenAI, Anthropic, Azure, Groq, DeepSeek, Together, OpenRouter | Free + paid (details paywalled) | YC (batch not detailed) |
| 4 | **OpenRouter** | openrouter.ai | 70T monthly tokens, 5M+ users, 300+ models | Commercial SaaS | Unified API, "Auto Exacto" routing, credit-based | **No compression**; caching implicit via providers | — | 60+ providers | Credit-based ($10, $99 entry) | Undisclosed |
| 5 | **Braintrust** | braintrust.dev | n/a | Commercial SaaS | Eval + observability + Loop agent | **No compression**, but Loop agent optimizes prompts | Prompt optimization | All major | Paywalled | **Series B $80M** (2026) |
| 6 | **LangSmith** | smith.langchain.com | n/a | Commercial SaaS | Eval + observability for LangChain | **No compression** | — | All major | Paywalled | LangChain-backed |
| 7 | **Cloudflare AI Gateway** | developers.cloudflare.com/ai-gateway | Global edge | Commercial | Gateway w/ cache, rate-limit, fallback, analytics | **Cache-based**, no semantic compression | Exact-match cache | Workers AI, Anthropic, Gemini, OpenAI, Replicate | Included in CF plans | Public (NET) |
| 8 | **Apache APISIX AI** | apisix.apache.org | Apache top-level | Apache-2.0 | API/AI gateway w/ ai-proxy, token rate-limit, ai-rag, prompt-decorator | **No compression** | — | OpenAI, DeepSeek, Claude, Mistral, Gemini | Free | ASF |
| 9 | **OmniRoute** | github.com/diegosouzapw/OmniRoute | 3.2k | MIT | Universal API proxy w/ fallback stacks | **Yes — proactive context compression** (binary-search token pruning, signature+semantic cache, request dedup) | Rule + cache | 100+ | Free | Bootstrap |
| 10 | **Edgee** | github.com/edgee-ai/edgee | 54 | Apache-2.0 | Rust LLM gateway (successor to RTK) | **Yes — token-compression engine** (noise stripping) | Rule-based (inherits RTK) | Anthropic, OpenAI, others | Free | Unknown |
| 11 | **RTK (Rust Token Killer)** | github.com/rtk-ai/rtk | 31k | Apache-2.0 | CLI proxy compressing **shell command outputs** before they enter LLM context | **Yes — smart filtering, grouping, truncation, dedup** | Rule-based per-command | Claude Code, Copilot, Cursor, Gemini CLI, Windsurf, Cline | Free | Unknown (predecessor to Edgee) |
| 12 | **Redis LangCache** | redis.io/langcache | Preview | Commercial | Semantic cache as REST API | **Cache, not compression** — returns cached LLM answers on similarity match | Vector similarity (cosine, 768/1536 dim) | Provider-agnostic | $1.5/M input, $100/mo storage (preview) | Redis Inc. |

**Adjacent-layer verdict:** Gateway players mostly do *routing* and *caching*, not *semantic compression*. The two that overlap — RTK/Edgee (Rust, open) and OmniRoute (proxy with rule-based prune) — compress **tool outputs and histories**, not **model prose output**. This is a different lane. But their 31k-star base gives them the distribution to add prose compression in a single release. That is the #1 attack surface.

---

## 3. Academic / research

| Method | Venue | Compression | Quality | Status | Productized by |
|---|---|---:|---|---|---|
| **LLMLingua** (arXiv:2310.05736) | EMNLP'23 | up to 20x | "minimal perf loss" | Released, MIT, 6k stars on MS repo | Microsoft, LangChain, LlamaIndex, Prompt Flow |
| **LongLLMLingua** (arXiv:2310.06839) | ACL'24 | 4x fewer tokens | +21.4% on NaturalQuestions RAG; 1.4–2.6x latency | Released | Microsoft |
| **LLMLingua-2** | — | 3–6x faster than LLMLingua | Data-distillation | Released | Microsoft |
| **SecurityLingua** | — | — | 100x less cost than SOTA guardrails | Released | Microsoft |
| **500xCompressor** (arXiv:2408.03094, ACL'25) | ACL 2025 Main | **6x–480x** | Adds ~0.25% params; no fine-tuning of base model | github.com/ZongqianLi/500xCompressor | Academic only |
| **SCOPE** (arXiv:2508.15813) | 2025 | Generative rewrite | — | Academic | — |
| **CompactPrompt** (arXiv:2510.18043) | Oct 2025 | Hard prune + n-gram abbrev + numeric quant | Lossless on structured docs | Academic | — |

**Academic verdict:** The research frontier is 6–480x model-based compression — not rules. If Microsoft productizes LLMLingua-3 with a streetman-killing compression ratio, rules become obsolete for non-adversarial workloads. Streetman's only defense: rules are deterministic, auditable, and zero-inference-cost; model-based approaches add latency and non-determinism. Enterprise HIPAA/SOX buyers care about determinism. Lean into that.

---

## 4. Sleeper threats

- **Anthropic native compression in Claude Code.** Anthropic already ships automatic caching (90% discount). The next obvious step is first-party output-brevity modes and first-party context compaction (Claude Code already has `/compact`). If they ship "budget mode" with rule-based brevity + caching as a default, streetman's addressable market collapses outside of enterprise/multi-vendor.
- **OpenAI ships compression in Responses API.** Same logic. OpenAI's research arm is aggressive; prompt caching arrived in 2024, semantic compression could be a 2026 H2 ship.
- **Microsoft Copilot bundles LLMLingua-3.** Microsoft owns LLMLingua. A one-line enable flag in Copilot kills third-party compression on the largest developer base in the world.
- **Caveman v2 with bench + Rust binary.** Julius Brussee has the stars, distribution, and brand. If he reads the streetman launch, he ships a Rust binary + bench harness in 4 weeks and keeps the 40k-star moat.
- **RTK/Edgee pivots to prose.** They have the Rust expertise, 31k stars, and a gateway footprint. Adding a prose-compression pass to their pipeline is a weekend of work.
- **The Token Company gets pricing right.** Their "not a generative LLM, drop-in API" positioning + YC + +1.1% accuracy win is a dangerous pitch to enterprise CFOs. If they ship HIPAA, streetman's enterprise lane dies.
- **Compresr's agentic proxy.** Open-sourcing the proxy (github.com/Compresr-ai/Context-Gateway) while monetizing the SDK is exactly the LiteLLM playbook. Very hard to beat on distribution if they out-execute.
- **TOON displaces TOOG as the input-encoding standard.** TOON has 24k stars, a spec, Rust SDK, VS Code extension, tree-sitter grammar. If TOON adds an "intent" layer, TOOG loses its wedge.
- **Cursor ships first-party "brief mode" via Rules + agent telemetry.** They already have the IDE and the rules DSL. A one-bit toggle kills the Cursor install base for third-party compressors.
- **Bolt.new / Lovable / v0 silently ship proprietary compression.** They already eat 1000x tokens per session. They have the strongest financial pressure to compress. When they ship, they don't announce.
- **Redis / Upstash bundle LangCache free with Vercel.** Semantic caching at free tier cannibalizes ~70% of cacheable workloads.
- **Vendors normalize "thinking-token trimming."** Streetman's feature #24 is a real wedge today; once Anthropic ships native thinking-trim knobs it becomes table-stakes.

---

## 5. Native vendor features

| Vendor | Feature | Savings | Min context | Workspace | Roadmap signal |
|---|---|---:|---|---|---|
| **Anthropic** | Prompt caching (automatic) | cache-read = 0.1× base input (90% off); 5-min TTL standard, 1-hour TTL at 2× write | 1,024+ | Workspace-isolated (Feb 5, 2026) | Automatic caching rolling out batch-by-batch; no public compression roadmap |
| **Anthropic** | Batch API | 50% off | — | — | Composable with caching → 95% combined |
| **OpenAI** | Prompt caching (automatic) | up to 90% off ($0.25/M on GPT-5.4 vs $2.50/M); no config | 1,024+ | Org-level | Image input caching live ($8 → $2); thinking-token trimming not announced |
| **Google Gemini** | Implicit caching (2.5+) | **90%** (vs 75% on 2.0) | 1,024 (Flash), 2,048 (Pro) | Automatic | No storage cost; explicit caching available for guaranteed hits |
| **Amazon Bedrock** | Prompt caching (Claude/Nova) | Similar to Anthropic | Varies | IAM-scoped | Parity w/ upstream |

**Native-feature verdict:** Input caching is no longer a competitive lane. Anyone selling input compression in 2026 must outperform a 90% provider discount *on top of* the 10% residual. That is a brutal math problem. Output compression (where streetman lives) has **no vendor equivalent** and is the real defensible lane.

---

## 6. Coding agents — context-management competitors

| Agent | Built-in context mgmt | Token reduction claim | Compression type | Threat to streetman |
|---|---|---:|---|---|
| **Aider** | Repo-map (only relevant parts sent), `/tokens`, `/drop`, `/clear`, chat-history summarization at soft-limit | **4.2× fewer tokens than Claude Code** in 3-codebase bench | File selection + summarization | **High** — already ultra-efficient without streetman |
| **Cline** | `/smol` command (in-place compression), Auto Compact at ~80% | ≥70% reduction via 3rd-party TF-IDF token-manager (web-werkstatt/cline-token-manager, 76% reduction claimed) | Summarization + selection | Medium — streetman adds orthogonal prose compression |
| **Continue.dev** | Context exceeded alerts (UI), no auto-compression yet | — | — | Low — feature gap opportunity |
| **Roo Code** | Intelligent Context Condensing v2 (configurable threshold slider, 80% default), ContextWindowProgress bar, native token endpoints | — | Summarization at threshold | Medium — architecturally similar to streetman's #19 auto-reinject |
| **Claude Code (native)** | `/compact`, auto-caching | Provider-level caching | Cache | Low overlap on output |

**Agent verdict:** Coding agents already compress **conversation history** well. Streetman's orthogonal play is **prose output** — the text the model writes, which these agents do not touch. But if any agent ships a skin over streetman-like prose compression as a built-in, the reason to install streetman-as-plugin vanishes. Cline's `/smol` is a warning shot.

---

## 7. Prompt-programming frameworks — TOOG threats

| Framework | GH stars | License | Approach | TOOG overlap |
|---|---:|---|---|---|
| **DSPy** (stanfordnlp/dspy) | 33.9k | MIT | Programming > prompting: declarative modules, optimizers compile high-quality prompts | **High** — DSPy's MIPRO can produce shorter, more optimized prompts automatically. TOOG must beat DSPy on *developer intent expression*, not on compression |
| **BAML** (BoundaryML/baml) | n/a | Open | Typed behavioral APIs, schema-aligned parsing (SAP), multi-language | **High** — BAML is the polished version of what TOOG wants to be. BAML already has Python/TS/Ruby/Java/C#/Rust/Go bindings and native VS Code/JetBrains tooling |
| **Priompt** (anysphere/priompt) | n/a | Open (Anysphere = Cursor's parent) | JSX-based prompt design w/ priority-based context window fitting | **Medium** — Priompt solves context budgeting declaratively. Streetman's TOOG competes for the same "express prompts as code" headspace, but Anysphere ships it in Cursor |
| **LMQL** (eth-sri/lmql) | n/a | Apache-2.0 | Query language w/ constraints, datatypes, speculative exec | **Medium** — LMQL claims 26–85% inference-cost reduction via constraints + caching |
| **Guidance** (guidance-ai/guidance) | 19k+ | Open | Token-by-token steering, structured outputs | Low-medium — Guidance controls output *format*, not input compression |

**TOOG verdict:** TOOG is not competing with compressors — it's competing with **prompt DSLs**. DSPy has 33.9k stars, BAML has polish and enterprise multi-lang support, Priompt is owned by Cursor. For TOOG to win it must (1) be deeply integrated with streetman compress so that bi-directional benefit is real, (2) have a codegen story (DSL → runtime prompt) that beats DSPy's optimizer, and (3) ship a VS Code LSP on day 1 (BAML already has this). **This is the weakest part of streetman's claimed positioning.** Consider rebranding TOOG as "input preset" not a DSL.

---

## 8. Cursor / Windsurf / Zed / Lovable / v0 native compression

| Editor | Native compression | Rules/context system | Threat |
|---|---|---|---|
| **Cursor** | **None explicit**, but Rules (.cursor/rules), workspace index, codebase semantic index cut ingestion. Since Jan 2026, cursor-agent CLI parity | Project Rules + User Rules + legacy .cursorrules; @codebase semantic search | Cursor could ship first-party brief-mode from Priompt → kills Cursor plugin lane |
| **Windsurf** | Flows (agent-aware context), Cascade, Rules, Memories; no explicit prose compression | Persistent Rules + Memories | Medium |
| **Zed** | **Proposed:** automatic context compression (Discussion #32614) w/ user-defined threshold. Currently 120k-token limit | Slash commands for explicit context; minimalist vs Cursor | Medium — if Zed ships compression, streetman must have a Zed skill |
| **Lovable / Bolt.new / v0** | **Bolt V2 handles 1000× larger projects** via improved context mgmt; token rollover policy; no public compression API | Opaque | High — these platforms will compress silently. Opportunity: streetman-as-a-backend |

**Editor verdict:** Cursor owns the IDE market but has no prose compression. This is the streetman wedge for the coming 6–12 months. Zed is the #1 Rust-native ally opportunity. Bolt/Lovable/v0 are potential B2B customers for DevBooster's API, not competitors.

---

## 9. Category maps

### 9a. 2×2: Input vs Output × Rule vs Model

```
                 RULE-BASED                      MODEL-BASED
          ┌──────────────────────────────┬──────────────────────────────┐
INPUT     │ RTK, Edgee, LeanCTX,          │ LLMLingua, LongLLMLingua,    │
(prompt/  │ Claw Compactor, LLM-Token-    │ 500xCompressor, SCOPE,       │
context)  │ Saver-RS, compression-prompt, │ CompactPrompt, Headroom,     │
          │ OmniRoute, TOON (format)      │ Token Company, Compresr      │
          │                               │                              │
          │ >> crowded, 7+ OSS players    │ >> academic + 2 fresh YC     │
          ├──────────────────────────────┼──────────────────────────────┤
OUTPUT    │ **Caveman (40.8k ★)**         │ **empty**                    │
(prose    │ streetman (proposed)          │                              │
response) │                               │                              │
          │ >> 1 dominant player +        │ >> no commercial             │
          │   1 challenger (you)          │   model-based output         │
          │                               │   compressor exists          │
          └──────────────────────────────┴──────────────────────────────┘
BOTH: streetman (claimed), Caveman (compress + output), Headroom (reversible)
```

### 9b. Pricing ladder

| Tier | Players |
|---|---|
| **Free OSS** | Caveman, RTK, Edgee, LeanCTX, Claw Compactor, Headroom, LLMLingua, OmniRoute, compression-prompt, TOON, DSPy, BAML, Priompt, LMQL, Guidance, Aider |
| **Freemium SaaS** | Helicone, OpenRouter, Portkey (free tier), LangSmith, Braintrust (free tier) |
| **Usage-based SaaS** | OpenRouter (credits), Redis LangCache ($1.5/M + $100/mo storage) |
| **Enterprise-gated** | Portkey, Braintrust (Series B), LangSmith, Cloudflare AI Gateway |
| **Savings-share** | **streetman (proposed 20% savings) — empty category** |

### 9c. Adoption tier (GitHub stars)

| Bracket | Players |
|---|---|
| **40k+** | Caveman (40.8k), LiteLLM (44k) |
| **20–40k** | RTK (31k), DSPy (33.9k), TOON (24k), Guidance (19k) |
| **5–20k** | LLMLingua (6k), Portkey (10.2k) |
| **1–5k** | Claw Compactor (2.2k), OmniRoute (3.2k), Headroom (1.5k) |
| **<1k** | Edgee (54), LLM Token Saver RS (1), streetman (0) |

---

## 10. Whitespace analysis

### Crowded axes — DON'T compete here
- **Input token filtering / context pruning in Rust** — RTK, Edgee, LeanCTX, Headroom, LLM-Token-Saver-RS, compression-prompt, OmniRoute. 7+ players, 31k-star leader. Streetman's TOOG must not market itself as "shorter prompts" — it must market as "structured intent."
- **Python academic compressors** — LLMLingua family + derivatives. Dead-end distribution; Microsoft owns the base.
- **LLM gateways with caching** — LiteLLM, Portkey, Helicone, OpenRouter, Cloudflare, APISIX. Don't build a gateway; be adapters.
- **Prompt DSLs** — DSPy, BAML, LMQL, Priompt, Guidance. 5 serious entrants with big-co backing. TOOG as a DSL is a losing battle.

### Empty axes — CLAIM these
- **Output-prose model-based compression.** Nobody ships it commercially. If a Rust binary can run a 300M-param distilled compressor at <10ms, streetman could occupy the full 2×2 bottom-right cell.
- **Bench-as-service / independent verification.** No competitor offers this. This is the streetman moat. Own the words "bench-verified" and "1,440-call matrix" in 2026 marketing.
- **Savings-share billing.** No competitor charges on measured savings. This is DevBooster's monetization moat.
- **Vertical/domain compression packs (HIPAA, SOX, finance, k8s, SQL).** Domain profiles are a claimed streetman feature but not a shipped product by any competitor. Easy enterprise wedge.
- **Accuracy rubric as a category standard.** If streetman publishes the 100%-accuracy rubric as an open spec and caveman/RTK/Edgee adopt it, streetman becomes the standard-setter even if it loses on stars.
- **Rust + multi-IDE + bench-gated CI.** Caveman is markdown skills. RTK is a proxy. LeanCTX is a TUI. No one covers the exact triangle streetman targets.
- **Thinking-token trimmer.** Only streetman claims this. Until Anthropic ships it, streetman owns the lane.
- **Zed-native compression plugin.** Zed has Discussion #32614 open for automatic compression and no shipper. First Rust-native plugin into Zed gets the brand.

### Integration opportunities
- **LiteLLM adapter** — instant 19,700-project reach. **Highest-ROI distribution move.**
- **RTK/Edgee complement** — RTK compresses tool output, streetman compresses model output. Bundle.
- **Aider** — Aider uses 4.2× fewer tokens already; adding streetman makes it 8–10×. Co-market.
- **Cline `/smol` partnership** — offer streetman as the engine for Cline's summarization.
- **Braintrust integration** — streetman emits accuracy scores; Braintrust scores them. Bench-as-service becomes a Braintrust scorer.
- **Redis LangCache** — streetman compresses pre-cache, increasing hit rate on paraphrases.

---

## 11. Streetman's defensible moats — post-research

| Claimed moat (from README / BUSINESS.md) | Holds? | Notes |
|---|---|---|
| 85%+ token cut on output prose | **Holds** conditional on bench | Beats caveman's 65–75%. But only if accuracy stays 100%. Risky claim to defend in public. |
| 100% technical accuracy rubric | **Holds, rare** | Nobody else ships a deterministic extractor + LLM judge gate. This is real IP. |
| Rust single static binary, <10ms | **Holds, but not unique** | LeanCTX, RTK, Edgee, LLM-Token-Saver-RS, compression-prompt all ship Rust. The differentiator is *what* runs in Rust, not the language. |
| Bench-as-service CLI (1,440-call matrix) | **Holds, category-defining** | Genuinely unique. Biggest unclaimed brand real estate in the category. |
| Consonant-skeleton algorithm (unbounded vocab) | **Holds technically** | Not a business moat — rule-sets get copied in 1 weekend. But the combination with guards + collision detector is patent-adjacent. |
| Bi-directional compression (TOOG input + streetman output) | **Holds as a bundle** | TOOG standalone is weak vs DSPy/BAML/Priompt. Bundle it; never sell it alone. |
| Multi-platform native (CC + Cursor + Codex + VS Code) | **Partially** | Caveman already ships to 40+ agents. Multi-platform is table-stakes, not a moat. Focus: Rust-binary-shared-across-platforms uniqueness. |
| Accuracy rubric as open spec | **Potential, not yet** | Publish `ACCURACY_SPEC.md` as MIT. Get LLMLingua/Caveman to adopt it. Become the standard. |
| Savings-share proxy billing | **Holds, unclaimed** | Nobody else does this. Hard to execute (metering, fraud, disputes) but real. |
| Enterprise self-hosted with SSO + audit | **Holds, empty market** | Portkey and Braintrust have enterprise but no compression. First compression-focused enterprise pitch wins HIPAA/SOX. |
| Thinking-token trimmer | **Holds, 6-month window** | Ship before Anthropic/OpenAI do. |
| "Crown changes hands" against incumbent | **Hard** | Caveman has 40.8k stars and shipping velocity. You need 100k+ stars in 18 months or pivot to enterprise-first. |

### New moats revealed by research (not in current README)
- **Open accuracy-spec leadership.** No one else has tried.
- **Zed-first Rust plugin.** Zed is looking for exactly this; Cursor plugin is a red ocean.
- **LiteLLM adapter as growth lever.** 19,700 projects at zero distribution cost.
- **Savings Dashboard as a wedge into FinOps.** Nobody else reports per-repo compression ROI. CFOs buy ROI.

### Moats that need sharpening
- **"85% vs 50%"** — the claim is credible but the README says "leading competitor" — name caveman directly in a public side-by-side or you look evasive.
- **Multi-platform** — restate as "same Rust binary everywhere" not "4 integrations" (caveman claims 40+).
- **Rubric-gated CI** — make the rubric public BEFORE a single user is live or caveman ships their own version and claims the standard.

---

## 12. Attack surface — what could kill streetman tomorrow

| Attacker | Move that kills streetman | Streetman's counter |
|---|---|---|
| **Caveman (Julius Brussee)** | Ships Rust binary + bench harness + LiteLLM adapter in 4 weeks | Ship LiteLLM adapter + bench-as-service public scorecard **week 1 of launch** |
| **Anthropic** | Native "brief mode" toggle in Claude Code | Enterprise HIPAA/SOX focus; multi-vendor bench |
| **OpenAI** | Ships output compression in Responses API | Same — multi-vendor is the hedge |
| **Microsoft + LLMLingua-3** | 100x model-based compression bundled into Copilot | Position rules as deterministic & auditable; sell to regulated industries |
| **RTK/Edgee** | Adds a prose-compression pass to their 31k-star gateway | Co-market: streetman for prose, RTK for tool output |
| **Token Company (YC W26)** | Enterprise HIPAA + $150M round | Ship enterprise SKU with savings-share billing FIRST |
| **Compresr (YC W26)** | Open-source proxy goes viral on HN | Open-source more: make `streetman serve` the best self-hosted option |
| **Cursor (Anysphere)** | First-party brief-mode via Priompt | Ship VS Code + Zed parity fast; don't depend on Cursor store |
| **TOON community** | Publishes rule-engine interop spec | Propose TOON + TOOG interop; do not fragment |
| **Generic YC AI-infra wave** | 2–3 more compression plays in W27 | First-mover with published accuracy standard |

### Must-ship list (to prevent the above)
1. **LiteLLM adapter** on launch day (tap 19,700 projects).
2. **Public bench-as-service endpoint** with a leaderboard showing streetman vs caveman vs LLMLingua vs Headroom (run all their tools through your bench, publish numbers).
3. **Open `ACCURACY_SPEC.md`** as MIT — get first competitor to adopt it within 30 days (even if they "only adopt the rubric"); become the standard.
4. **Savings Dashboard freemium tier** — no competitor has FinOps story. CFOs are the real buyer.
5. **Enterprise SKU** with at least one vertical pack (HIPAA or k8s) before Token Company or Compresr get there.
6. **Rename TOOG positioning** from "DSL" to "prompt preset compiler" — don't fight DSPy/BAML/Priompt on their turf.
7. **Zed plugin** as #1 editor target (not Cursor) — less contested; Rust-native affinity.
8. **File side-by-side benchmarks on caveman** publicly and invite Julius to co-bench. Either you win and own the narrative, or you lose gracefully and get credibility.

---

## 13. Sources

Direct competitors
- github.com/JuliusBrussee/caveman
- github.com/edgee-ai/edgee
- github.com/rtk-ai/rtk
- leanctx.com · github.com/yvgude/lean-ctx · crates.io/crates/lean-ctx
- github.com/open-compress/claw-compactor
- github.com/chopratejas/headroom
- github.com/snailer-team/llm-token-saver-rs
- crates.io/crates/compression-prompt · docs.rs/compression-prompt
- ycombinator.com/companies/the-token-company · thetokencompany.com
- ycombinator.com/companies/compresr · compresr.ai · github.com/Compresr-ai/Context-Gateway

Academic
- arxiv.org/abs/2310.05736 (LLMLingua) · github.com/microsoft/LLMLingua · llmlingua.com
- arxiv.org/abs/2310.06839 (LongLLMLingua)
- arxiv.org/abs/2408.03094 (500xCompressor) · github.com/ZongqianLi/500xCompressor
- arxiv.org/html/2508.15813v1 (SCOPE)
- arxiv.org/html/2510.18043v1 (CompactPrompt)
- aclanthology.org/2025.naacl-long.368.pdf (Prompt Compression Survey)

Gateways / proxies / observability
- github.com/BerriAI/litellm
- portkey.ai
- helicone.ai
- openrouter.ai
- braintrust.dev
- smith.langchain.com
- developers.cloudflare.com/ai-gateway
- apisix.apache.org/ai-gateway · apisix.apache.org/docs/apisix/plugins/ai-proxy
- github.com/diegosouzapw/OmniRoute
- redis.io/langcache · redis.io/calculator/langcache · redis.io/blog/llm-token-optimization-speed-up-apps/

Formats / DSLs
- github.com/toon-format/toon · toonformat.dev · infoq.com/news/2025/11/toon-reduce-llm-cost-tokens/
- github.com/stanfordnlp/dspy · dspy.ai
- github.com/BoundaryML/baml · boundaryml.com · docs.boundaryml.com/home
- github.com/anysphere/priompt
- github.com/eth-sri/lmql · lmql.ai
- github.com/guidance-ai/guidance · guidance-ai/llguidance

Native vendor features
- platform.claude.com/docs/en/build-with-claude/prompt-caching
- platform.claude.com/docs/en/about-claude/pricing
- openai.com/index/api-prompt-caching · platform.openai.com/docs/guides/prompt-caching · openai.com/api/pricing
- ai.google.dev/gemini-api/docs/caching · ai.google.dev/gemini-api/docs/pricing · docs.cloud.google.com/vertex-ai/generative-ai/docs/context-cache/context-cache-overview
- developers.googleblog.com/en/gemini-2-5-models-now-support-implicit-caching
- docs.aws.amazon.com/bedrock/latest/userguide/prompt-caching.html

Coding agents
- aider.chat/docs/troubleshooting/token-limits.html · aider.chat/docs/repomap.html · morphllm.com/comparisons/morph-vs-aider-diff
- github.com/cline/cline · cline.bot/blog/how-to-think-about-context-engineering-in-cline · docs.cline.bot/prompting/understanding-context-management
- github.com/web-werkstatt/cline-token-manager · github.com/web-werkstatt/ai-context-optimizer
- github.com/continuedev/continue
- docs.roocode.com/features/intelligent-context-condensing · docs.roocode.com/update-notes/v3.43.0 · deepwiki.com/RooCodeInc/Roo-Code/7-context-and-message-management

Editors
- cursor.com/docs/rules · docs.cursor.com/context/rules · cursor.com/blog/agent-best-practices
- docs.windsurf.com (via morphllm / nxcode reviews)
- github.com/zed-industries/zed/discussions/32614
- nxcode.io/resources/news/v0-vs-bolt-vs-lovable-ai-app-builder-comparison-2025

YC / funding
- extruct.ai/research/ycw26/
- buildmvpfast.com/blog/yc-w26-batch-agent-infrastructure-boom
- techcrunch.com/2026/04/15/parasail-raises-32m-to-feed-tokenmaxxing-ai-developers/
- siliconangle.com/2026/01/22/inferact-launches-150m-funding-commercialize-vllm/

Ecosystem / skills
- code.claude.com/docs/en/skills
- claudepluginhub.com/plugins/juliusbrussee-caveman
- github.com/jeremylongshore/claude-code-plugins-plus-skills

_Every row in Sections 1–8 traces to at least one source above. Where GitHub stars/contributors were unavailable on fetch they are marked `[?]`. Discrepancies noted: TOON star count reported as 24k on repo page; articles from Nov 2025 cite "growing adoption"; both are consistent. LLMLingua stars shown as 6k on MS repo; survey paper references separate star counts for sub-repos._
```

## Approval checklist for exit-plan-mode
- [ ] User approves writing the "FINAL DELIVERABLE" block above to `/Users/efi.jeremiah/projects/streetman/COMPETITIVE.md` (strip only the markdown code fence and the header comment).
- [ ] Word count ~3,400 — within the 3,000–5,000 target.
- [ ] All 13 required sections present.
- [ ] Brutal-honesty quotient: "if streetman ships without bench-as-service and savings-share by Q3-2026, it becomes a caveman fork"; TOOG positioning called weak; multi-platform called not-a-moat; 85% vs 50% claim called evasive. Confirmed brutal.
- [ ] Tables > prose; 9 tables present.
- [ ] Sources section has 60+ URLs.
