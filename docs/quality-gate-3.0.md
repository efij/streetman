# Quality Gate 3.0

Streetman 3.0 promotes the main verification command to:

```bash
streetman bench run --suite quality-gate-3
```

It includes the complete v2 accuracy-gated lane and adds executable enterprise
product surfaces:

- `streetman enterprise init-config --protect --push-registry`
- `streetman enterprise rbac`
- `streetman enterprise compliance`
- `streetman enterprise sbom`
- `streetman enterprise release-attest`
- `streetman enterprise deploy`
- `streetman enterprise observability`
- `streetman enterprise report`
- `streetman daemon --once`

The release still keeps raw-ratio claims honest: LLMLingua and LeanCTX remain
tracked as top lossy/raw-ratio baselines, not defeated by live measurement unless
a committed snapshot proves it. The v3 win is the local lane: accuracy-100,
never-worse, reversible/proof-carrying, deterministic, offline, and backed by
enterprise configuration and release evidence.
