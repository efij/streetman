#!/usr/bin/env sh
# Streetman one-liner installer.
#
#   curl -fsSL https://raw.githubusercontent.com/efij/streetman/main/install.sh | sh
#
# Fetches the prebuilt binary for this platform (falling back to `cargo install`
# if a Rust toolchain is present), then wires per-prompt compression enforcement
# into every AI host it can find (Claude Code, Codex). Never fails the shell on
# error: on any problem it prints guidance and exits 0.

set -u

REPO="efij/streetman"
BIN_DIR="${STREETMAN_HOME:-$HOME/.streetman}/bin"
BIN="$BIN_DIR/streetman"

say() { printf 'streetman: %s\n' "$1" 1>&2; }

# A binary is only "good enough" if it understands the `init` subcommand. An old
# cargo/npm install predating the installer will be on PATH but lack it, so we
# probe capability instead of mere presence.
supports_init() { [ -x "$1" ] && "$1" init --help >/dev/null 2>&1; }

fetch_prebuilt() {
  mkdir -p "$BIN_DIR" 2>/dev/null || true
  os="$(uname -s 2>/dev/null || echo unknown)"
  arch="$(uname -m 2>/dev/null || echo unknown)"
  case "$os/$arch" in
    Darwin/arm64)  asset="streetman-darwin-arm64" ;;
    Darwin/x86_64) asset="streetman-darwin-x64" ;;
    Linux/aarch64) asset="streetman-linux-arm64" ;;
    Linux/x86_64)  asset="streetman-linux-x64" ;;
    *)             asset="" ;;
  esac
  [ -n "$asset" ] || return 1
  command -v curl >/dev/null 2>&1 || return 1
  url="https://github.com/$REPO/releases/latest/download/$asset"
  say "downloading prebuilt binary ($asset)..."
  curl -fsSL "$url" -o "$BIN.tmp" 2>/dev/null || return 1
  chmod +x "$BIN.tmp" 2>/dev/null || true
  mv "$BIN.tmp" "$BIN" 2>/dev/null || return 1
  [ -x "$BIN" ]
}

# --- 1. find a binary that actually supports `init` -----------------------
ONPATH=""
command -v streetman >/dev/null 2>&1 && ONPATH="$(command -v streetman)"

RESOLVED=""
if supports_init "$ONPATH"; then
  RESOLVED="$ONPATH"
elif supports_init "$BIN"; then
  RESOLVED="$BIN"
fi

# --- 2. install / update if no capable binary was found -------------------
if [ -z "$RESOLVED" ]; then
  if [ -n "$ONPATH" ]; then
    say "found an older streetman at $ONPATH (no 'init'); fetching the current release..."
  fi
  if fetch_prebuilt && supports_init "$BIN"; then
    RESOLVED="$BIN"
  elif command -v cargo >/dev/null 2>&1; then
    say "no prebuilt binary for this platform; building from source with cargo (one-time)..."
    cargo install --git "https://github.com/$REPO" streetman-cli \
      --bin streetman --locked --force --root "${STREETMAN_HOME:-$HOME/.streetman}" >/dev/null 2>&1 || true
    supports_init "$BIN" && RESOLVED="$BIN"
  fi
fi

if [ -z "$RESOLVED" ]; then
  say "could not install a working binary automatically. Install once with:"
  say "  cargo install --git https://github.com/$REPO streetman-cli --bin streetman --locked --force"
  exit 0
fi
say "binary ready at $RESOLVED"

# --- 3. if a stale streetman shadows PATH, upgrade it in place ------------
# Keeps the bare `streetman` command working (off-switch, manual runs) even when
# an old cargo/npm copy sits earlier on PATH than our fresh binary.
if [ -n "$ONPATH" ] && [ "$ONPATH" != "$RESOLVED" ] && ! supports_init "$ONPATH"; then
  if [ -w "$ONPATH" ] && cp "$RESOLVED" "$ONPATH" 2>/dev/null; then
    chmod +x "$ONPATH" 2>/dev/null || true
    say "upgraded the stale copy on your PATH: $ONPATH"
    RESOLVED="$ONPATH"
  else
    say "note: an old 'streetman' shadows PATH at $ONPATH."
    say "      put the new one first:  export PATH=\"$BIN_DIR:\$PATH\""
    say "      (add that line to your shell rc to make it permanent)"
  fi
fi

# --- 4. wire per-prompt enforcement into detected hosts -------------------
"$RESOLVED" init --host auto --mode "${STREETMAN_MODE:-full}" || \
  say "wiring step reported an error; binary is installed, re-run: streetman init"

exit 0
