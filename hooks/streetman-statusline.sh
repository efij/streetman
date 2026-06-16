#!/usr/bin/env sh
flag="${PLUGIN_DATA:-$HOME/.streetman}/.streetman-lean-active"
[ -f "$flag" ] || exit 0
mode="$(head -n1 "$flag" | tr -d '[:space:]')"
[ -n "$mode" ] || exit 0
printf '[STREET:LEAN:%s]' "$(printf '%s' "$mode" | tr '[:lower:]' '[:upper:]')"
