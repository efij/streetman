"""Thin Python binding for the Streetman Rust binary.

The Rust CLI is the canonical engine. This module keeps library mode small and
auditable while still exposing Headroom-style `compress(messages)` ergonomics.
"""

from __future__ import annotations

import json
import subprocess
from dataclasses import dataclass
from typing import Any, Iterable


@dataclass
class Streetman:
    binary: str = "streetman"

    def compile(self, text: str, mode: str = "full", domain: str = "auto") -> dict[str, Any]:
        return self._run_json(["compile", "--mode", mode, "--domain", domain, "--json", "--no-archive"], text)

    def compress(self, text: str, mode: str = "full", domain: str = "auto") -> dict[str, Any]:
        return self._run_json(["compress", "--mode", mode, "--domain", domain, "--json", "--no-archive"], text)

    def compress_messages(self, messages: Iterable[dict[str, Any]], mode: str = "full") -> list[dict[str, Any]]:
        out = []
        for message in messages:
            cloned = dict(message)
            content = cloned.get("content")
            if isinstance(content, str):
                cloned["content"] = self.compile(content, mode=mode, domain="auto")["wire"]
            out.append(cloned)
        return out

    def _run_json(self, args: list[str], stdin: str) -> dict[str, Any]:
        proc = subprocess.run(
            [self.binary, *args],
            input=stdin,
            text=True,
            capture_output=True,
            check=True,
        )
        return json.loads(proc.stdout)


def compress(messages: Iterable[dict[str, Any]], mode: str = "full", binary: str = "streetman") -> list[dict[str, Any]]:
    return Streetman(binary=binary).compress_messages(messages, mode=mode)
