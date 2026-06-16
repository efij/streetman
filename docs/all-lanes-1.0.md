# All Lanes 1.0

Streetman 1.0 adds an executable all-lanes gate:

```bash
streetman bench run --suite all-lanes
```

The suite verifies:

- Token correctness: unsafe `ultra` output is guarded by accuracy and real-token
  checks before emission.
- Prose: Streetman can stack token-greedy compression on top of a supplied
  semantic rewrite and never exceed that rewrite's token count.
- Logs/JSON: deterministic structural compression remains active.
- Code: anchored edit transport avoids full-file reprints for small changes.
- Reversibility/context: archive-free decode and `--fit N` token-budget packing.
- Performance: local deterministic smoke gate. SIMD/sub-ms daemon performance is
  not claimed yet.
- Enterprise-local controls: secret classification, security attestation,
  tamper-evident event hashes, encrypted archive, protected/pushed config
  manifests, and Claude tokenizer honesty.

Honest caps:

- No public Claude tokenizer exists for offline optimal counts.
- Learned on-device rewriting, seccomp no-network enforcement, RBAC,
  Sigstore/SBOM, Docker/Helm packaging, and SIMD/daemon targets remain gated
  roadmap items until local tests prove them.
