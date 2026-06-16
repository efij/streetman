# Changelog

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
