#!/usr/bin/env sh
# streetman bootstrap: make the plugin zero-prerequisite.
# Runs on SessionStart. If the `streetman` binary is already reachable it exits
# immediately (fast no-op). Otherwise it installs one, preferring a prebuilt
# release binary and falling back to `cargo install`. It never blocks or fails
# the session: on any error it prints guidance and exits 0.

set -u

BIN_DIR="${STREETMAN_HOME:-$HOME/.streetman}/bin"
BIN="$BIN_DIR/streetman"
REPO="efij/streetman"

# Already installed somewhere on PATH or in our managed dir? Done.
if command -v streetman >/dev/null 2>&1; then
  exit 0
fi
if [ -x "$BIN" ]; then
  # Expose for sibling hooks that look in the managed dir.
  printf '%s\n' "$BIN_DIR" > "${STREETMAN_HOME:-$HOME/.streetman}/bin-path" 2>/dev/null || true
  exit 0
fi

mkdir -p "$BIN_DIR" 2>/dev/null || true

# Resolve platform -> release asset name (publish these in GitHub Releases).
os="$(uname -s 2>/dev/null || echo unknown)"
arch="$(uname -m 2>/dev/null || echo unknown)"
case "$os/$arch" in
  Darwin/arm64)  asset="streetman-darwin-arm64" ;;
  Darwin/x86_64) asset="streetman-darwin-x64" ;;
  Linux/aarch64) asset="streetman-linux-arm64" ;;
  Linux/x86_64)  asset="streetman-linux-x64" ;;
  *)             asset="" ;;
esac

# 1) Prefer a prebuilt release binary (fast, no toolchain needed).
if [ -n "$asset" ] && command -v curl >/dev/null 2>&1; then
  url="https://github.com/$REPO/releases/latest/download/$asset"
  if curl -fsSL "$url" -o "$BIN.tmp" 2>/dev/null; then
    chmod +x "$BIN.tmp" 2>/dev/null && mv "$BIN.tmp" "$BIN" 2>/dev/null
  fi
fi

# 2) Fall back to building from source if a toolchain is present.
if [ ! -x "$BIN" ] && command -v cargo >/dev/null 2>&1; then
  echo "streetman: building from source (one-time)..." 1>&2
  cargo install --git "https://github.com/$REPO" streetman-cli \
    --bin streetman --locked --root "${STREETMAN_HOME:-$HOME/.streetman}" >/dev/null 2>&1 || true
fi

if [ -x "$BIN" ] || command -v streetman >/dev/null 2>&1; then
  printf '%s\n' "$BIN_DIR" > "${STREETMAN_HOME:-$HOME/.streetman}/bin-path" 2>/dev/null || true
  echo "streetman: ready." 1>&2
else
  echo "streetman: could not auto-install. Install once with:" 1>&2
  echo "  cargo install --git https://github.com/$REPO streetman-cli --bin streetman --locked" 1>&2
fi

exit 0
