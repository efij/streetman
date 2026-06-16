---
name: streetman-help
description: Quick reference for Streetman compression, Lean mode, proof, and benches.
license: MIT
---

# Streetman Help

Core commands:

```bash
streetman compress --mode full --domain prose
streetman compile --mode full --domain context
streetman proof verify original.txt compressed.txt certificate.json
streetman diff original.txt compressed.txt --html
```

Lean commands:

```bash
streetman lean instructions --mode full --host codex
streetman lean review --diff
streetman lean audit .
streetman lean gate --before base --after HEAD
streetman lean prove --diff --normal-twin full-version.patch --command "cargo test"
streetman lean bench run --against ponytail
streetman lean kill --against ponytail --json
```

Lean levels:

- `lite`: build requested work, mention smaller alternative.
- `full`: smallest correct diff by default.
- `ultra`: deletion-first, challenge bloat.
- `off`: no Lean injection.
