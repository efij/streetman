# Streetman Absolute Win Plan

Streetman 3.0 defines its strongest public lane as:

```text
highest compression that is also lossless, accuracy-100, reversible or
proof-carrying, deterministic, offline, and locally attestable
```

That definition is intentionally narrower than raw compression ratio. Lossy
systems can still win raw ratio; they do not win this lane unless they satisfy
the same local gates.

## Top Published Baselines

- LLMLingua: model-based prompt compression with high raw ratio, but lossy,
  neural-model dependent, not exactly reversible, and outside the zero-egress
  local proof lane by default.
- LeanCTX: production proxy-style context compression with useful bill cuts, but
  network-oriented and lossy under this lane definition.

## Executable Gate

Run:

```bash
streetman bench run --suite absolute-win-3
```

The gate covers 17 dimensions:

1. token correctness and `ultra` accuracy-100 fallback
2. prose stacking on a supplied rewrite with never-worse token guard
3. JSON schema factoring
4. log-line templatization
5. anchored code diff transport
6. code-comment compression with logic intact
7. Lean code-generation minimalism gate
8. context `--fit` token-budget packing
9. archive-free readable decode
10. proof-carrying output
11. secret/PII-aware scan
12. zero-egress attestation
13. encrypted-archive attestation
14. zero-telemetry attestation
15. offline proof attestation
16. tamper-evident audit attestation
17. Claude tokenizer honesty cap

It also records enterprise product-surface cases for config UX, RBAC,
compliance mapping, SBOM, release attestation, deployment templates, local
observability, and daemon smoke coverage.

It records two published-baseline claim-boundary cases:

- `published-baseline-llmlingua-lossy-gate`
- `published-baseline-leanctx-network-lossy-gate`

## Honest Residuals

- Do not claim raw-ratio superiority over LLMLingua or LeanCTX without live,
  committed snapshots.
- Claude has no public offline tokenizer, so Streetman only makes best-effort or
  online-optional Claude token claims.
- External Sigstore transparency-log inclusion, hosted SSO, and live raw-ratio
  wins over lossy baselines need environment-specific runs before becoming
  shipped claims.
