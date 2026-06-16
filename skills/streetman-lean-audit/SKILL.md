---
name: streetman-lean-audit
description: >
  Repo-wide Streetman Lean audit for code bloat, dependency bloat,
  single-implementation abstractions, wrappers, dead config, and avoidable
  custom code.
license: MIT
---

# Streetman Lean Audit

Scan the whole repo for what can be removed or replaced with stdlib/native
features. Rank largest cuts first.

Output:

`<tag> <what to cut>. <replacement>. [path]`

Use the same tags as `streetman-lean-review`: `delete`, `stdlib`, `native`,
`yagni`, `shrink`.

End with `net: -N lines, -M deps possible.` If nothing should be cut:
`Lean already. Ship.`

Local proof path:

```bash
streetman lean audit .
```
