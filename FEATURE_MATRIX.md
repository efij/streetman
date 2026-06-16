# Streetman Absolute-Win Feature Matrix

This matrix tracks the requested Headroom + Token Optimizer parity work.

| Capability | Headroom Bar | Token Optimizer Bar | Streetman Status |
|---|---|---|---|
| Output-prose compression | Limited direct focus | Detects output waste | Scaffolded in Rust core |
| JSON/tool-output compression | SmartCrusher + CCR | Runtime compression events | Scaffolded with anomaly-preserving JSON summary |
| Logs/search/diffs/code | ContentRouter compressors | Bash/read handlers | Snapshot-winning deterministic handlers for logs/search/pytest |
| Reversible originals | CCR retrieve tool | Tool archive/expand | Scaffolded encrypted archive + retrieve |
| Proxy | OpenAI/Anthropic/Gemini proxy | N/A | Local `/health`, `/stats`, `/v1/compress`, chat/responses transform, and `STREETMAN_UPSTREAM_URL` forwarding |
| MCP tools | `headroom_compress/retrieve/stats` | N/A | `streetman mcp serve` exposes compress/compile/retrieve/stats over stdio JSON |
| Memory | Cross-agent memory | Config/memory audits | Shared ShortLang memory store via `streetman memory add/list` |
| Session quality | Context management | v6 dual-score | Scaffolded audit score |
| Compaction continuity | CCR/context tracker | Progressive checkpoints | Planned; audit detectors scaffolded |
| Dashboard | Stats/history | Full local dashboard | Intel Dashboard implemented as local HTML |
| Zero telemetry | Opt-out | Default no telemetry | Implemented default |
| Bench-as-service | Bench docs/evals | Methodology docs | Fixture bench + competitor capture + absolute-win compare implemented |
| Policy-as-code | N/A | Config audits | OSS local policy checks implemented with `streetman policy check` |
| Proof certificates | Retrieve/citation metadata | Audit trails | OSS deterministic compression certificates + `streetman proof verify` |
| Red-team compression suite | Safety by compressor rules | Safety by config/audit | OSS `streetman bench run --suite redteam` implemented |
| Compression diff viewer | N/A | Dashboard issue views | OSS text/HTML diff via `streetman diff` |
| Gateway conformance | Proxy compatibility | Codex/session support | OSS LiteLLM/OpenRouter/Portkey contract checks via `streetman gateway conformance` |
| Agent wrapping | `headroom wrap` for coding agents | Codex/session support | `streetman run` and `streetman wrap` emit replayable run receipts |
| Cache alignment | CacheAligner | N/A | `streetman cache-align` stabilizes policy/memory/retrieval/payload prefixes |
| H2H proof | Public proof/evals | N/A | `streetman duel --against headroom` creates trace comparison JSON/HTML |
| Implementation minimalism | N/A | Ponytail-style YAGNI / stdlib / native rules | `streetman lean` ships instructions, review, audit, gate, Lean Certificates, adapter assets, and Ponytail H2H fixtures |
| Token-greedy safety | N/A | Prose-shortening can inflate real tokens | `streetman bench run --suite token-greedy` proves actual-token candidate selection and never-worse-than-raw guard |

## OSS vs Enterprise Segregation

| Area | Open Source | Enterprise / DevBooster |
|---|---|---|
| Compression core | Deterministic local compressors, proof certificates, red-team fixtures | Private model/profile tuning, managed certification packs |
| Policy | Local `.streetman.toml` policy checks | Org-wide policy distribution, SSO/RBAC, approval workflows |
| Gateways | Local conformance checks for LiteLLM/OpenRouter/Portkey | Hosted gateway, billing integration, fleet rollout, provider SLAs |
| Dashboard | Local Intel Dashboard HTML and local archive stats | Multi-team dashboards, chargeback, savings-share billing, compliance exports |
| Archive | Local encrypted content-addressed originals | Managed retention policies, legal hold, centralized KMS, audit trails |

## Implementation Rule

Streetman should not claim parity for any row marked scaffolded or planned. Parity
requires end-to-end tests plus a committed benchmark or fixture proving the behavior.

Current committed evidence: `benchmarks/results/competitor-live.json` and
`benchmarks/results/competitor-compare.json` show a local offline absolute-win
snapshot. Broader hosted/provider claims still require additional snapshots.
