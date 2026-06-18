# Streetman One-Liner Install + Per-Prompt Enforce — Design

Date: 2026-06-18
Status: Approved (brainstorming → ready for implementation plan)
Author: design session

## Problem

The product (Rust compression engine) is built, but the **distribution and
enforcement story is broken**:

- No `curl | sh` one-liner. Install requires Rust (`cargo`) or `npm install -g
  github:efij/streetman`.
- README contradicts itself: top says `/plugin install streetman` works,
  bottom says "Claude Code marketplace plugin — not published yet"
  (README.md:151 vs README.md:196).
- The existing hooks (`hooks/streetman-activate.js`,
  `hooks/streetman-mode-tracker.js`) only toggle **lean mode** state and inject
  lean instructions. **Nothing actually compresses anything per-prompt.**
- The headline "enforce by default on each prompt" promised on
  streetmandev.vercel.app does not exist as a working install.

Competitive context (COMPETITIVE.md, verified 2026-06-18):

- **Caveman (40.8k★)** wins on distribution via instruction-injection skills,
  not engine quality. Shipping injection-only = "a caveman fork."
- **RTK (63.6k★, Apache-2.0)** owns a lane streetman ignores per-prompt:
  **tool/command-output compression** via a PreToolUse hook (`ls`, `cat`,
  `git`, test runners) — the biggest token sink in a coding session
  (~118k→24k in their example).
- Nobody owns **deterministic + both-directions + output-prose**. That is the
  gap to take.

## Goal

A single pasteable command that installs streetman and turns on per-prompt
enforcement by default, across Claude Code and Codex (v1):

```
curl -fsSL https://streetman.dev/install.sh | sh
```

## Scope (v1)

- **Hosts:** Claude Code, Codex CLI. (VS Code dropped from v1. Host-wiring
  designed to extend to Cursor / OpenCode / OpenClaw / Hermes / Pi later.)
- **Enforce:** layered, default ON, mode=`full`.
- **Direction:** both (input + output), via the layers below.
- **Marketplaces:** not published by this work. Produce publish-ready manifests
  + a paste-list of publish commands; the maintainer runs them with their
  credentials.

### Out of scope (v1, documented follow-on)

- The full RTK 100+ command catalog. v1 ships the **wedge** only (see Layer C).
- VS Code extension wiring.
- Auto-publish CI to npm / VS Code Marketplace / Claude store.

## Architecture

```
curl | sh  →  fetch prebuilt binary (fallback cargo)  →  ~/.streetman/bin/streetman
                                                      →  streetman init --host auto
                                                              │
   Layer A  injection    → hook injects "compress output" instructions every turn   (prose; caveman-parity)
   Layer B  proxy        → host API base-URL → streetman proxy; byte-rewrite both ways (deterministic; the moat)
   Layer C  tool-output  → PreToolUse hook rewrites bash → streetman compress         (RTK-parity; biggest $ save)
```

The three layers are **independent**. Any layer can fail or degrade without
blocking the session. Config wiring lives in **Rust (`streetman init`)**, not
shell — the binary owns JSON/TOML writing so it is testable and there is a
single source of truth. `install.sh` stays tiny: fetch binary, delegate to
`streetman init`.

## Components

| Component | New/Exists | Purpose |
|---|---|---|
| `install.sh` (extends `hooks/streetman-bootstrap.sh`) | new | detect OS/arch, fetch binary, call `streetman init` |
| `streetman init --host auto\|claude\|codex` | new Rust subcommand | detect hosts; write/remove configs idempotently; `--uninstall`, `--dry-run` |
| `streetman instructions` (compress mode) | exists (main.rs:344) — verify it emits compression guidance, not lean-only | per-turn injected guidance |
| Host hook configs | rewrite existing `hooks/hooks.json` | Claude + Codex: SessionStart + UserPromptSubmit + PreToolUse |
| `streetman proxy` | exists | deterministic both-way rewrite + upstream forward |
| proxy lifecycle | new (small) | start-on-demand, health check, `~/.streetman/proxy.pid` |
| Layer C command handlers | new (clean-room) | top ~10 high-token commands |
| `streetman gain` | extend `audit`/dashboard | savings stats, USD, daily breakdown, JSON |
| `~/.streetman/config.toml` | extend `.streetman.toml` schema | mode, on/off, per-command excludes |
| README + website fix | edit | remove the "works / not published" contradiction |
| publish-prep docs + paste-list | new | manifests + maintainer publish commands |

### Off-switch

- `streetman init --uninstall` — removes all wiring.
- `STREETMAN_DEFAULT=off` or `/streetman off` — disables enforcement, keeps
  wiring.

## Layer C — RTK-parity (wedge for v1)

RTK is a CLI wrapper + PreToolUse hook that compresses **command output before
the agent reads it**, using four strategies: smart filtering, grouping,
truncation, deduplication. Streetman's engine already implements those
strategies across its compress lanes (logs, JSON, diff, code) and already has
`streetman run`/`wrap`. The missing piece is the **PreToolUse hook that routes
tool output through the compressor**, plus per-command handlers.

**Clean-room mandate:** reimplement RTK's *ideas* from scratch in streetman's
engine. No RTK source code copied. CI includes a grep guard against RTK/Apache
source strings. (RTK is Apache-2.0; streetman core is MIT.)

### RTK key-feature coverage (all Case tracked; v1 ships the wedge)

| RTK Case | Streetman today | v1? |
|---|---|---|
| PreToolUse auto-rewrite | not wired | **v1 — core wedge** |
| `ls/read/find/grep/diff` handlers | logs/diff/code lanes + run/wrap | v1 (top set) |
| `git status/log/diff` | diff lane | v1 |
| Test runners failures-only (`jest/pytest/cargo/go`) | — | v1 (top set) |
| Linters grouped (`eslint/ruff/clippy/tsc`) | log templatize | follow-on |
| `docker/kubectl/aws/gh` | — | follow-on |
| 4 strategies (filter/group/truncate/dedup) | all exist | reuse |
| Failure recovery (full output on non-zero exit) | — | **v1** |
| `gain` (savings stats, USD, graphs) | audit/dashboard partial | v1 (basic) |
| `discover` (find bypassed commands) | — | follow-on |
| `session` (adoption) | run receipts | follow-on |
| Per-command excludes in config | `.streetman.toml` | v1 (schema) |
| Telemetry off by default | done | done |

**v1 command set (~10 by token value):** `ls`/tree, `cat`/read, `grep`,
`find`, `git status`, `git log`, `git diff`, `cargo test`, `pytest`,
`jest`/`vitest`.

## Data flow

**Install (once):** detect OS/arch → fetch binary (fallback cargo) →
`streetman init --host auto` → locate Claude (`~/.claude` or project `.claude`)
and Codex (`.codex-plugin` / `PLUGIN_DATA`) → write `hooks.json` (SessionStart,
UserPromptSubmit, PreToolUse) idempotently → write proxy base-URL config +
`~/.streetman/config.toml` (mode=full, on) → print what was wired + off-switch.

**Per turn:**

- **A** — SessionStart/UserPromptSubmit hook → `streetman instructions --mode
  full` → injected as `additionalContext`. Model emits ShortLang.
- **B** — proxy (if running) → host API call routed to `127.0.0.1:PORT` →
  `streetman` rewrites request + response bytes → forwards upstream via
  `STREETMAN_UPSTREAM_URL`.
- **C** — PreToolUse hook → sees a Bash command (`git status`, `cat`, test) →
  runs through `streetman compress` with the matching lane → returns compressed
  output. On non-zero exit → returns **full unfiltered output** (failure
  recovery).

State: `~/.streetman/config.toml` (mode, on/off, excludes),
`~/.streetman/proxy.pid`, savings ledger for `streetman gain`.

## Error handling / graceful degrade (never block a session)

| Failure | Behavior |
|---|---|
| binary missing at runtime | hook prints guidance, exits 0, session proceeds uncompressed |
| proxy can't bind/start | Layer B skipped; A+C still run; init logs it |
| `streetman compress` errors on tool output | pass through **raw** output (never lose data) |
| compressed command exits non-zero | return full unfiltered output (failure recovery) |
| compression would inflate tokens | existing token-greedy guard → emit raw (never-worse) |
| host config already wired | `init` is idempotent — detect + skip/update, never duplicate |
| user wants out | `streetman init --uninstall` or `/streetman off` |

Every hook exits 0 on any error. Compression is best-effort; the correctness of
the underlying data is never sacrificed.

## Testing

- **Rust unit:** `streetman init` config-writing (idempotent, uninstall,
  dry-run) against a temp HOME; each Layer-C command handler (input fixture →
  expected compressed + protected-token accuracy = 100).
- **Failure-recovery test:** failing command returns full output.
- **Never-worse test:** reuse existing `token-greedy` suite for Layer C
  handlers.
- **Hook integration:** simulate Claude `PreToolUse`/`UserPromptSubmit` JSON on
  stdin → assert stdout contract; exits 0 even on bad input (extend
  `crates/streetman-cli/tests/cli_smoke.rs`).
- **install.sh:** shellcheck + a dry-run wiring a temp HOME, asserting files,
  on macOS + Linux.
- **Clean-room guard:** CI grep that no RTK/Apache source strings landed.

## Success criteria

1. `curl -fsSL https://streetman.dev/install.sh | sh` installs the binary on
   macOS + Linux with no Rust toolchain present.
2. After install, a fresh Claude Code and Codex session shows per-turn
   compression enforcement on by default (mode=full).
3. Layer C compresses at least the v1 command set, with failure-recovery and
   never-worse guarantees.
4. `streetman init --uninstall` cleanly removes all wiring.
5. README and website no longer contradict themselves about what is published.
6. Publish-ready manifests + a maintainer paste-list exist for npm + Claude
   marketplace.
