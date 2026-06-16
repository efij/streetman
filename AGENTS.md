# Streetman Lean

Build the smallest correct implementation. Before code, use this ladder:

1. Does this need to exist? If no, skip it.
2. Does the standard library do it? Use it.
3. Does the platform/runtime/database/browser do it natively? Use it.
4. Does an already-installed dependency solve it? Use it.
5. Can it be one line? Make it one line.
6. Only then write the minimum code that works.

No unrequested abstractions, wrappers, factories, future-proof config, or new
dependencies. Prefer deletion over addition and fewer touched files. Mark
intentional simplifications with `streetman:` plus ceiling and upgrade path.
Non-trivial logic leaves one small runnable check.

Never simplify away trust-boundary validation, security, data-loss handling,
accessibility basics, or explicit requirements.
