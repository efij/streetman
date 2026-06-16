# Absolute Win 2.0

Streetman 2.0 adds a named, executable gate for the "absolute win" frame:

```bash
streetman bench run --suite absolute-win-2
```

This is not a raw-ratio benchmark. The lane definition is explicit:

- lossless enough for the protected-token accuracy checker to score `100`
- never worse than raw on actual tokenizer tokens
- reversible or proof-carrying
- deterministic and offline
- locally attestable without provider calls

The suite covers 17 local dimensions: token correctness, prose stacking on a
supplied rewrite, JSON factoring, log templatization, anchored code transport,
code-comment compression, Lean code-generation gates, context `--fit`, readable
archive-free decode, proof-carrying output, secret/PII detection, zero-egress,
encrypted archive, zero telemetry, offline proof attestation, tamper-evident
audit claims, and the Claude tokenizer honesty cap.

Published baselines:

- LLMLingua remains a top raw-ratio research baseline, but it is model-based,
  lossy, and not exactly reversible in this local gate.
- LeanCTX remains a production proxy baseline, but it is network-oriented and
  lossy under this lane definition.

That means Streetman 2.0 can claim the accuracy-gated/offline/reversible lane
when this suite passes. It should not claim to beat lossy systems on raw ratio
unless live committed snapshots prove that separately.
