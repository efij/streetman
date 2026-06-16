# Changelog

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
