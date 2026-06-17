# Changelog

## 4.0.1 - 2026-06-17

- Fixed prose mode regression: `lite`, `full`, and `ultra` now produce
  mode-dependent token-positive prose output again instead of collapsing to
  byte-identical punctuation stripping.
- Added a regression test for prose mode differentiation with accuracy 100 and
  never-worse token counts.
- Added Case-C2 versioned platform-builtin oracle:
  `streetman code builtin-oracle --language typescript --runtime node18 --task
  "make an http request" --json`.
- Added `take5-case-c2-versioned-builtin-oracle` to the `absolute-win-4` gate so
  TAKE 5 covers Case-C1/Case-C2/Case-C3 rather than only decision/behavior proof.
- Hardened the daemon smoke test against connection reset after response close.
- Bumped source install/plugin manifests to `4.0.1`.

## 4.0.0 - 2026-06-17

- Added `streetman bench run --suite absolute-win-4`, the all-Case executable gate
  for the attached engineer plan.
- Optimized the prose hot path by moving protected-token, phrase, numeric, log,
  HTML, and sensitive-data regexes to lazy statics, adding an embedded
  word/skeleton shortcut table, and caching token-greedy word decisions.
- Added deterministic Case-9 stacked prose rewrite for long prose, gated by
  accuracy and never-worse token checks.
- Added JSON columnar delta factoring and tightened log run-length templates for
  the widened logs/JSON lane.
- Added `streetman code behavior-gate` for behavior-equivalence proof of code
  minimization.
- Added zeroized archive encryption buffers for the copied plaintext used during
  encrypted archive writes.
- Added `docs/absolute-win-4.0.md` and bumped source install/plugin manifests to
  `4.0.0`.

## 3.0.0 - 2026-06-17

- Added `streetman bench run --suite absolute-win-3`, extending the v2
  accuracy-gated/offline/reversible lane with enterprise product-surface gates.
- Added `streetman enterprise` commands for config init/protect/push UX, RBAC,
  compliance mapping, SBOM, release attestation, deployment templates, local
  observability, and a combined readiness report.
- Added `streetman daemon` with a local resident HTTP health/compress surface and
  `--once` mode for CI smoke verification.
- Expanded security attestation claims for RBAC, supply-chain artifacts,
  air-gap posture, compliance mapping, local observability, and deployment
  templates.
- Added `docs/absolute-win-3.0.md` and refreshed install docs to make v3 the
  current release gate.
- Bumped source install/plugin manifests to `3.0.0`.

## 2.0.0 - 2026-06-17

- Added `streetman bench run --suite absolute-win-2`, an executable 17-dimension
  gate for the accuracy-100, lossless/reversible, deterministic, offline win
  lane.
- Added published-baseline gates for LLMLingua and LeanCTX. They are tracked as
  top raw-ratio competitors, while Streetman only claims the gated lane unless a
  live snapshot proves raw-ratio superiority.
- Extended `bench accuracy-fixtures` to include the v2 gate so installed builds
  verify token correctness, code transport, Lean code-generation minimalism,
  reversibility, enterprise controls, and published-baseline claim boundaries in
  one command.
- Added `docs/absolute-win-2.0.md` with the exact claim definition and honest
  caps.
- Bumped source install/plugin manifests to `2.0.0`.

## 1.0.1 - 2026-06-16

- Fixed the `ultra` truncation bug: prose/docs compression now uses the same
  strict protected-token accuracy checker as `streetman accuracy-check` before
  emitting output. If strict accuracy would fail, Streetman falls back or
  reverts raw.
- Added a regression test for the camelCase loss case where `ultra` previously
  emitted a certificate score of 100 while standalone `accuracy-check` scored
  the result below 100.
- Added enterprise config controls:
  `streetman policy protect`, `streetman policy verify`, and
  `streetman policy push`.
- Added protected config manifests with content hash + deterministic signature,
  plus local registry push receipts for distributing/verifying the exact
  `.streetman.toml` policy file.
- Bumped source install/plugin manifests to `1.0.1`.

## 1.0.0 - 2026-06-16

- Added `streetman bench run --suite all-lanes`, the major-version gate covering
  token correctness, prose stacking on supplied rewrites, logs/JSON, code
  transport, reversibility/context fit, performance smoke, and enterprise-local
  controls.
- Fixed the compression guard path into an explicit safe-mode ladder: requested
  mode -> safer mode(s) -> raw, with accuracy and real-token checks required
  before any compressed output is emitted.
- Added `streetman compress --fit N` for token-budget packing.
- Added `streetman decode` for archive-free readable expansion of common
  Streetman abbreviations.
- Added tokenizer profile reporting with an honest Claude cap:
  `streetman tokenizer profile --model claude-...`.
- Added `streetman security scan` plus core secret/PII classification. Archive
  records with sensitive markers are tagged before encrypted storage without
  returning plaintext findings.
- Added BYOK-style local archive key override via `STREETMAN_ARCHIVE_KEY`.
- Added tamper-evident archive event hashes (`prev_hash` + `event_hash`).
- Kept heavyweight targets honest-capped: bundled learned rewriting,
  Claude-optimal offline counts, seccomp syscall enforcement, Sigstore/SBOM,
  RBAC, Helm, and SIMD/daemon sub-ms performance need dedicated executable
  gates before being claimed as complete.
- Bumped source install/plugin manifests to `1.0.0`.

## 0.3.0 - 2026-06-16

- Added `streetman bench run --suite final-case`, a real-token verification gate
  for the implemented final-design pieces.
- Added Case-C8 anchored edit-only code transport with token accounting.
- Added Case-C9 unchanged-region elision for long code payloads.
- Added Case-C7 code comment/docstring compression: logic lines remain intact,
  comments are token-greedy compressed, and never-worse still gates output.
- Added Case-3a log-line templatization and Case-3b JSON schema-row factoring,
  both chosen only when real tokenizer counts beat the prior candidate.
- Added `streetman code diff`, `streetman code elide`, and
  `streetman security attest`.
- Added an offline security attestation for Case-S1/Case-S2/Case-S3/Case-S5 with an
  honest Claude tokenizer cap. Learned rewriting, Claude-optimal offline
  counts, seccomp enforcement, SBOM signing, SIMD, and daemon warm mode remain
  roadmap-gated until they have executable acceptance tests.
- Bumped source install/plugin manifests to `0.3.0`.

## 0.2.0 - 2026-06-16

- Added Streetman Lean as a first-class subsystem: host instructions, review,
  audit, gate, proof, Ponytail H2H fixtures, hook/adaptor assets, and
  `streetman:` shortcut comments.
- Added token-greedy compression backed by real `tiktoken` counts. Word,
  abbreviation, skeleton, and phrase candidates are accepted only when they
  reduce actual tokens for the active model.
- Added the never-worse-than-raw guard. If compression inflates token count,
  Streetman reverts the output and records the guard in the proof certificate.
- Added `STREETMAN_MODEL` tokenizer selection plus exported
  `token_estimate_for_model` for model-specific benches and gates.
- Added `streetman bench run --suite token-greedy` and committed
  `benchmarks/results/token-greedy-case1-case2.json` proving the known
  `for -> 4` / out-of-vocabulary skeleton trap is rejected.
- Added ShortLang compile/run receipts, MCP-style tool metadata, memory/learn
  helpers, cache alignment, duel reports, and proxy forwarding support.
- Bumped source install/plugin manifests to `0.2.0` so Git installs and local
  plugin manifests advertise the latest feature set.

## 0.1.0 - 2026-06-07

- Initial Streetman OSS release with deterministic compression, proof
  certificates, archive/retrieve, fixture benches, policy checks, and local
  audit reports.
