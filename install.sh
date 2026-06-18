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

# --- 1. already installed? ------------------------------------------------
if command -v streetman >/dev/null 2>&1; then
  RESOLVED="$(command -v streetman)"
elif [ -x "$BIN" ]; then
  RESOLVED="$BIN"
else
  RESOLVED=""
fi

# --- 2. fetch a binary if needed ------------------------------------------
if [ -z "$RESOLVED" ]; then
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

  if [ -n "$asset" ] && command -v curl >/dev/null 2>&1; then
    url="https://github.com/$REPO/releases/latest/download/$asset"
    say "downloading prebuilt binary ($asset)..."
    if curl -fsSL "$url" -o "$BIN.tmp" 2>/dev/null; then
      chmod +x "$BIN.tmp" 2>/dev/null && mv "$BIN.tmp" "$BIN" 2>/dev/null
    fi
  fi

  if [ ! -x "$BIN" ] && command -v cargo >/dev/null 2>&1; then
    say "no prebuilt binary; building from source with cargo (one-time)..."
    cargo install --git "https://github.com/$REPO" streetman-cli \
      --bin streetman --locked --root "${STREETMAN_HOME:-$HOME/.streetman}" >/dev/null 2>&1 || true
  fi

  if [ -x "$BIN" ]; then
    RESOLVED="$BIN"
  elif command -v streetman >/dev/null 2>&1; then
    RESOLVED="$(command -v streetman)"
  fi
fi

if [ -z "$RESOLVED" ]; then
  say "could not install automatically. Install once with:"
  say "  cargo install --git https://github.com/$REPO streetman-cli --bin streetman --locked"
  exit 0
fi
say "binary ready at $RESOLVED"

# --- 3. wire per-prompt enforcement into detected hosts -------------------
"$RESOLVED" init --host auto --mode "${STREETMAN_MODE:-full}" || \
  say "wiring step reported an error; binary is installed, re-run: streetman init"

exit 0
