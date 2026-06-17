# Final capability 0.3 Status

Streetman 0.3 implements the verifiable parts of the final killer-feature
design that can be shipped without fake claims.

Implemented and gated:

- capability-1/capability-2: real-token greedy compression and never-worse-than-raw guard.
- capability-3a: repeated log-line templatization.
- capability-3b: JSON schema-row factoring for repeated object shapes.
- capability-C7: code comment/docstring compression with code logic preserved.
- capability-C8: anchored edit-only transport for small code changes.
- capability-C9: unchanged-region elision for long code payloads.
- capability-S1/capability-S2/capability-S3/capability-S5: offline security attestation, encrypted archive
  evidence, zero telemetry evidence, and proof-carrying output evidence.

Verification:

```bash
streetman bench run --suite capabilities
streetman security attest --json
```

Honest caps:

- Claude has no public offline tokenizer. Streetman does not claim
  Claude-optimal counts; optional online verification remains off by default.
- Learned on-device rewriting, seccomp/no-network syscall enforcement, SIMD
  streaming, and behavior-equivalence CI remain roadmap-gated. SBOM/release
  attestation and daemon smoke coverage were added later in v3.
- Prose readability is not claimed as a win over caveman-style English.
