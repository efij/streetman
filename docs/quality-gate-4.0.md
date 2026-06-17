# Quality Gate 4.0

Streetman 4.0 is the all-capability gate from the attached engineer plan:

```bash
streetman bench run --suite quality-gate-4
```

It includes v3 and adds executable cases for:

- FIX 1: warm prose latency after regex/token decision caching
- FIX 2: deterministic capability-9 stacked prose under the caveman token target at
  accuracy 100
- MOVE 3: lossy competitors tracked behind the same accuracy/lossless gate
- WIDEN 4: JSON columnar delta and log run-length template cases
- TAKE 5: `streetman code behavior-gate` for behavior-equivalence proof
- LOCK 6: signed enterprise/privacy surfaces plus zeroized archive encryption
  buffer

The local gate is intentionally explicit: it proves the offline,
accuracy-gated, reversible/proof-carrying lane. Live raw-ratio claims over
lossy systems still require committed external snapshots.
