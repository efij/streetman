# Security Policy

## Supported versions

| Version | Supported |
|---|---|
| Latest `main` | ✅ |
| Latest tagged release | ✅ |
| Older releases | ❌ (upgrade recommended) |

## Reporting a vulnerability

**Do not open a public GitHub issue for security vulnerabilities.**

Email: `security@streetman.dev` (GPG key below)

Include:
- Description of the vulnerability
- Steps to reproduce
- Affected versions
- Impact assessment
- Suggested fix (if any)

We aim to:
- Acknowledge within 24 hours
- Triage within 72 hours
- Ship fix within 7 days for critical, 30 days for high, 90 days for medium

## Scope

In scope:
- Streetman binary, core library, CLI
- Official plugin wrappers (Claude Code, Cursor, Codex, VS Code)
- Official gateway adapters (LiteLLM, Portkey, OpenRouter)
- `streetman serve` MCP server
- CI workflows

Out of scope:
- Third-party forks or plugins
- Issues in dependencies (report upstream — we'll track)
- Denial-of-service via expected high-compute operations (bench runs)
- Issues requiring physical access or compromised user machine

## Accuracy regressions as security issues

Streetman promises 100% technical accuracy. An accuracy bypass — where the skeleton engine mangles a security-critical detail (CVE number, auth token, SQL keyword, `rm -rf` safeguard) — is treated as a **HIGH severity security issue**, not a functional bug.

Examples of accuracy-regression-as-security:
- CVE identifier dropped or altered: `CVE-2026-1234 → CVE-2026-234`
- SQL keyword mangled: `DROP TABLE → DRP TABLE`
- Destructive flag lost: `rm -rf --no-preserve-root → rm -rf`
- Auth header corrupted: `Bearer xyz... → Berer xyz...`

Report via the security channel above. These get priority-fix status.

## GPG key

```
-----BEGIN PGP PUBLIC KEY BLOCK-----

[Placeholder — real key published on first release]

-----END PGP PUBLIC KEY BLOCK-----
```

## Hall of fame

Security researchers credited here after disclosure (with permission).
