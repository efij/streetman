# Streetman Benchmarks

The current implementation ships an **quality-gate fixture bench**:

```bash
cargo run --bin streetman -- bench run --suite quality-gate --out benchmarks/results/fixture-latest.json
cargo run --bin streetman -- bench gate benchmarks/results/fixture-latest.json
cargo run --bin streetman -- bench run --suite redteam
```

Fixture benches verify the local implementation shape. Competitor captures provide
the auditable comparison snapshot.

Generate comparison output:

```bash
cargo run --bin streetman -- bench capture-competitors --out benchmarks/results/competitor-live.json
cargo run --bin streetman -- bench compare --against headroom,token-optimizer,caveman,llmlingua,leanctx
```

Generate the local Intel Dashboard:

```bash
cargo run --bin streetman -- audit dashboard --out benchmarks/results/intel-dashboard.html
```

Run adoption-gate checks:

```bash
cargo run --bin streetman -- policy check --mode ultra --domain prose README.md
cargo run --bin streetman -- gateway conformance --provider all
cargo run --bin streetman -- diff README.md README.md --html --out benchmarks/results/compression-diff.html
```

The current committed snapshot is `competitor-live-2026-06-07`:

- Streetman context on matched Headroom workloads: `96.6%`
- Headroom measured context average: `90.5%`
- Streetman output on Caveman's own eval snapshot: `65.2%`
- Caveman output on its own eval snapshot: `50.0%`
- Streetman uses `30.3%` fewer output tokens than Caveman on that snapshot
- Streetman session fixture: `70.0%`
- Token Optimizer session detect-only effective savings: `0.0%`

Snapshot caveat: Headroom's JSON message-API lane is recorded as blocked by a local
certificate failure during tokenizer download. The measured Headroom direct
compressor lanes still run and are included. LLMLingua and LeanCTX are included as
published top baselines, not as local measured gates.

Future live competitor benches must include:

- exact competitor commit/version
- exact Streetman commit
- input/output/cache/effective-cost metrics
- fidelity and lost-decision checks
- latency percentiles
