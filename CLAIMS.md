# Streetman Claims Ledger

Streetman claims two bounded wins:

- an **absolute win for the committed local offline snapshot**
  `competitor-live-2026-06-07`
- an **accuracy-gated/offline/reversible enterprise lane win** when
  `streetman bench run --suite absolute-win-3` passes

This repository contains deterministic compression, local encrypted archive, retrieval,
audit reports, fixture benchmarks, live competitor capture, and claim-gate commands.
The project may only claim broader market victory after additional committed live
benchmarks pass the same gates.

## Current Status

| Claim | Status | Evidence |
|---|---|---|
| Output-prose compression engine exists | Implemented scaffold | `cargo test --workspace`; `streetman compress` |
| Reversible archive/retrieve exists | Implemented scaffold | `Archive` unit test; `streetman retrieve <hash>` |
| Zero telemetry by default | Implemented default | `.streetman.toml`, `StreetmanConfig::default()` |
| Absolute win over Caveman | Snapshot pass | `benchmarks/results/competitor-live.json`; Streetman `65.2%` vs Caveman `50.0%` on Caveman's own output eval snapshot; Streetman uses `30.3%` fewer output tokens |
| Absolute win over Headroom | Snapshot pass | `benchmarks/results/competitor-live.json`; Streetman `96.6%` vs Headroom `90.5%` on measured matching context workloads |
| Absolute win over Token Optimizer | Snapshot pass | `benchmarks/results/competitor-live.json`; Streetman `70.0%` session fixture vs Token Optimizer `0.0%` detect-only effective savings |
| `>=85%` output-prose fixture | Fixture-only pass | `benchmarks/results/fixture-latest.json` |
| `100%` technical fidelity | Fixture-only scaffold | `streetman bench accuracy-fixtures` |
| Policy-as-code adoption hook | Implemented local OSS | `streetman policy check --mode ultra --domain prose README.md` |
| Proof-carrying compression | Implemented local OSS | `streetman compress --json`; `streetman proof verify <original> <compressed> <certificate>` |
| Red-team compression safety | Implemented local OSS | `streetman bench run --suite redteam` |
| Compression diff viewer | Implemented local OSS | `streetman diff <original> <compressed> --html --out <file>` |
| Gateway conformance checks | Implemented local OSS | `streetman gateway conformance --provider all` |
| Accuracy-gated published-baseline lane | Implemented local OSS | `streetman bench run --suite absolute-win-3`; LLMLingua/LeanCTX tracked as lossy/network raw-ratio baselines |
| Enterprise config/protect/push UX | Implemented local OSS | `streetman enterprise init-config --protect --push-registry .streetman-policy-registry` |
| SBOM/release attestation | Implemented local OSS | `streetman enterprise sbom --json`; `streetman enterprise release-attest --json` |
| RBAC/compliance/deploy/observability artifacts | Implemented local OSS | `streetman enterprise report --json` |
| Resident daemon smoke path | Implemented local OSS | `streetman daemon --once --port 24846` |

## Win Gates

Streetman may publish a lane win only if a committed snapshot in
`benchmarks/results/` passes the relevant gate.

| Lane | Gate |
|---|---|
| Output prose | `>=85%` median full-mode savings, `>=90%` ultra, `100%` protected-fact fidelity |
| Output competitor | `>=30%` fewer visible output tokens than Caveman on a committed shared snapshot |
| Context compression | Match Headroom public workloads and beat by `>=5pp` or tie while winning latency/privacy/fidelity |
| Session optimization | `>=25%` lower effective cost than Token Optimizer on real-agent tasks |
| Trust | Zero telemetry default, local encrypted originals, public claim audit |
| Latency | Rust hot path `p50 <10ms` for 100KB deterministic compression |
| Enterprise adoption hooks | Policy, proof, red-team, diff, and gateway conformance commands must pass locally |
| Published research/proxy baselines | LLMLingua/LeanCTX are not counted as defeated on raw ratio unless live snapshots prove it; they are disqualified only from the lossless/offline/reversible lane when they cannot satisfy local proof gates |

## Snapshot Policy

- Fixture benches are useful for development, but they are **not** market claims.
- Live competitor benches must include exact competitor commit/version identifiers.
- Any failed lane must be documented as `not-yet-proven`.
- README headline numbers must be generated from or cite a snapshot ID.

## Snapshot Notes

- Headroom measured lanes: search `83.0%`, logs `98.1%`.
- Headroom JSON message-API lane is recorded as blocked by a local certificate failure
  while downloading `o200k_base.tiktoken`.
- Token Optimizer measured lanes: logs `0.0%`, pytest `61.0%`, retry-churn
  detector `0.0%` effective savings because it detects but does not compress/prevent
  in the offline fixture.
- Caveman measured lane: committed eval snapshot output `50.0%`; Streetman ultra over
  the same baseline outputs `65.2%`, which is `30.3%` fewer output tokens than Caveman.
- LLMLingua and LeanCTX are tracked as published top baselines in the snapshot, not
  local measured gates.
- Streetman compare status is generated by:
  `streetman bench compare --against headroom,token-optimizer,caveman,llmlingua,leanctx`.
