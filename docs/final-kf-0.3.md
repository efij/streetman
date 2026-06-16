# Final Case 0.3 Status

Streetman 0.3 implements the verifiable parts of the final killer-feature
design that can be shipped without fake claims.

Implemented and gated:

- Case-1/Case-2: real-token greedy compression and never-worse-than-raw guard.
- Case-3a: repeated log-line templatization.
- Case-3b: JSON schema-row factoring for repeated object shapes.
- Case-C7: code comment/docstring compression with code logic preserved.
- Case-C8: anchored edit-only transport for small code changes.
- Case-C9: unchanged-region elision for long code payloads.
- Case-S1/Case-S2/Case-S3/Case-S5: offline security attestation, encrypted archive
  evidence, zero telemetry evidence, and proof-carrying output evidence.

Verification:

```bash
streetman bench run --suite final-case
streetman security attest --json
```

Honest caps:

- Claude has no public offline tokenizer. Streetman does not claim
  Claude-optimal counts; optional online verification remains off by default.
- Learned on-device rewriting, seccomp/no-network syscall enforcement, SBOM
  signing, SIMD streaming, daemon warm mode, and behavior-equivalence CI are
  roadmap-gated until executable fixtures prove them.
- Prose readability is not claimed as a win over caveman-style English.
