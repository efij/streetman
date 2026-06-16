# Streetman Lean

Streetman Lean competes with Ponytail at the implementation-behavior layer while
keeping Streetman's proof, archive, policy, and benchmark model.

Commands:

```bash
streetman lean instructions --mode full --host codex
streetman lean review --diff
streetman lean audit .
streetman lean gate --before base --after HEAD
streetman lean prove --diff --normal-twin full-version.patch --command "cargo test"
streetman lean bench run --against ponytail --out benchmarks/results/ponytail-h2h.json
streetman lean kill --against ponytail --json
```

Feature-wise kill status is machine-readable: `streetman lean kill --against
ponytail --json` returns `feature_kill: true` when every Ponytail feature row is
covered and Streetman's extra proof/compression/gateway features are present.

Public performance claims still require a live provider replay. The built-in
`streetman lean bench run` output distinguishes this with
`public_performance_claim_ready: false`.

Lean gates block:

- avoidable new dependencies such as `flatpickr`, `lodash`, `axios`, and `moment`
- large non-trivial diffs without a runnable check
- too many touched files for a minimal change
- high extension-cost scores

Lean never removes trust-boundary validation, security, data-loss handling,
accessibility basics, or explicit requirements.
