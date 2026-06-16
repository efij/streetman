---
name: streetman-lean
description: >
  Streetman Lean mode: build the smallest correct implementation and prove it
  with diff-aware checks. Use when the user asks for lean, minimal, YAGNI,
  Ponytail-like behavior, smaller diffs, fewer dependencies, or less code.
license: MIT
---

# Streetman Lean

ACTIVE EVERY RESPONSE until "stop streetman lean", "streetman normal", or
"normal mode". Default level: full. Switch with `/streetman-lean lite|full|ultra|off`.

## Ladder

Stop at the first rung that holds:

1. Does this need to exist? If no, skip it.
2. Does the standard library do it? Use it.
3. Does the platform/runtime/database/browser do it natively? Use it.
4. Does an already-installed dependency solve it? Use it.
5. Can it be one line? Make it one line.
6. Only then write the minimum code that works.

## Rules

- No unrequested abstractions, factories, wrappers, future-proof config, or new deps.
- Prefer deletion over addition and fewer touched files.
- Mark intentional simplifications with `streetman:` plus ceiling and upgrade path.
- Non-trivial logic leaves one small runnable check.
- Never simplify away trust-boundary validation, security, data-loss handling, accessibility basics, or explicit requirements.
- End with what was skipped and when to add it.

## Levels

| Level | Behavior |
|---|---|
| lite | Build what was asked, then name the smaller stdlib/native alternative in one short line. |
| full | Enforce the ladder and ship the smallest correct diff without stalling. |
| ultra | Deletion first; challenge bloat and use native/one-line answers when correct. |

Use `streetman lean prove --diff` after implementation to emit a Lean Certificate.
