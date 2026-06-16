---
name: streetman-lean-review
description: >
  Diff review for unnecessary code, dependencies, abstractions, wrappers, and
  files. Use when asked to review for bloat, overengineering, Ponytail parity,
  or what can be deleted.
license: MIT
---

# Streetman Lean Review

Review only unnecessary complexity. One line per finding:

`path:Lline: <tag>: <what>. <replacement>. (-N lines)`

Tags:

- `delete`: dead code, dead config, placeholder, future-proofing.
- `stdlib`: dependency or custom code replaced by stdlib.
- `native`: dependency or code replaced by platform/runtime/browser/database.
- `yagni`: abstraction with one implementation or config nobody sets.
- `shrink`: same behavior in fewer lines.

Never flag one minimal runnable check as bloat. Security, data-loss handling,
trust-boundary validation, and accessibility go to normal review, not deletion.

End with `net: -N lines possible.` If nothing should be cut: `Lean already. Ship.`

Local proof path:

```bash
streetman lean review --diff
streetman lean gate --before base --after HEAD
```
