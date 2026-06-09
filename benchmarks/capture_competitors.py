#!/usr/bin/env python3
"""Capture pinned competitor benchmark snapshots for Streetman claim gates."""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
HEADROOM_REPO = "https://github.com/chopratejas/headroom.git"
HEADROOM_REF = "9fe4886cf6b612452f7271d3204872f804074c1f"
HEADROOM_WHEEL = "headroom-ai==0.23.0"
TOKEN_REPO = "https://github.com/alexgreensh/token-optimizer.git"
TOKEN_REF = "7051112b224ccd48bcb50da992ffa93fe4bed867"
CAVEMAN_REPO = "https://github.com/JuliusBrussee/caveman.git"
CAVEMAN_REF = "655b7d9c5431f822264b7732e9901c5578ac84cf"


def token_estimate(text: str) -> int:
    return max(1, (len(text) + 3) // 4) if text else 0


def run(cmd: list[str], cwd: Path | None = None) -> subprocess.CompletedProcess[str]:
    return subprocess.run(cmd, cwd=cwd, text=True, capture_output=True, check=False)


def ensure_repo(cache: Path, name: str, repo: str, ref: str) -> Path:
    dest = cache / name
    if not dest.exists():
        result = run(["git", "clone", repo, str(dest)])
        if result.returncode != 0:
            raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    result = run(["git", "fetch", "--all", "--tags"], cwd=dest)
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    result = run(["git", "checkout", ref], cwd=dest)
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    return dest


def ensure_headroom_venv(cache: Path) -> Path:
    venv = cache / "headroom-venv"
    python = venv / "bin" / "python"
    if not python.exists():
        result = run([sys.executable, "-m", "venv", str(venv)])
        if result.returncode != 0:
            raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    result = run([str(python), "-m", "pip", "install", "-U", "pip"])
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    result = run([str(python), "-m", "pip", "install", HEADROOM_WHEEL])
    if result.returncode != 0:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    return python


def case(
    competitor: str,
    workload: str,
    lane: str,
    status: str,
    before: int,
    after: int,
    source: str,
    accuracy: int = 100,
    error: str | None = None,
) -> dict[str, object]:
    savings = ((before - after) / before * 100.0) if before > 0 else 0.0
    return {
        "competitor": competitor,
        "workload": workload,
        "lane": lane,
        "status": status,
        "before_tokens": before,
        "after_tokens": after,
        "savings_percent": savings,
        "accuracy_score": accuracy,
        "source": source,
        "error": error,
    }


def capture_headroom(python: Path) -> list[dict[str, object]]:
    script = r'''
import json
from headroom.transforms import SearchCompressor, LogCompressor

def tok(s):
    return max(1, (len(s) + 3) // 4) if s else 0

search = "\n".join([f"src/file{i}.py:{i * 10}:def function_{i}():" for i in range(100)])
logs = "\n".join([
    "2026-06-07T12:00:00Z ERROR auth failed request_id=req_123" if i == 333
    else f"2026-06-07T12:00:00Z INFO worker heartbeat {i} ok"
    for i in range(500)
])
items = []
for name, text, compressed in [
    ("search", search, SearchCompressor().compress(search, context="find function_50").compressed),
    ("logs", logs, LogCompressor().compress(logs).compressed),
]:
    items.append({"workload": name, "before": tok(text), "after": tok(compressed)})
print(json.dumps(items))
'''
    result = run([str(python), "-c", script])
    cases: list[dict[str, object]] = []
    if result.returncode == 0:
        for item in json.loads(result.stdout):
            cases.append(
                case(
                    "headroom",
                    item["workload"],
                    "context",
                    "measured",
                    item["before"],
                    item["after"],
                    f"{HEADROOM_WHEEL}; {HEADROOM_REF}",
                )
            )
    else:
        cases.append(
            case(
                "headroom",
                "search-logs",
                "context",
                "blocked",
                0,
                0,
                f"{HEADROOM_WHEEL}; {HEADROOM_REF}",
                0,
                (result.stderr or result.stdout).strip(),
            )
        )

    api_script = r'''
import json
from headroom import compress

items = [{"id": i, "status": "FATAL" if i == 42 else "ok", "message": "background worker heartbeat finished successfully"} for i in range(80)]
messages = [{"role": "user", "content": json.dumps(items)}]
result = compress(messages, model="gpt-4o", compress_user_messages=True, protect_recent=0, kompress_model="disabled")
print(json.dumps({"before": len(json.dumps(messages)) // 4, "after": len(json.dumps(result.messages)) // 4, "saved": result.tokens_saved}))
'''
    result = run([str(python), "-c", api_script])
    if result.returncode == 0 and "Compression failed" not in result.stderr:
        item = json.loads(result.stdout)
        cases.append(
            case(
                "headroom",
                "json",
                "context",
                "measured",
                item["before"],
                item["after"],
                f"{HEADROOM_WHEEL}; {HEADROOM_REF}",
            )
        )
    else:
        cases.append(
            case(
                "headroom",
                "json",
                "context",
                "blocked",
                0,
                0,
                f"{HEADROOM_WHEEL}; {HEADROOM_REF}",
                0,
                (result.stderr or result.stdout).strip()[:1000],
            )
        )
    return cases


def capture_token_optimizer(repo: Path) -> list[dict[str, object]]:
    scripts = repo / "skills" / "token-optimizer" / "scripts"
    sys.path.insert(0, str(scripts))
    import bash_compress  # type: ignore
    from detectors.retry_churn import detect_retry_churn  # type: ignore

    logs = "\n".join([
        "2026-06-07T12:00:00Z ERROR auth failed request_id=req_123" if i == 333
        else f"2026-06-07T12:00:00Z INFO worker heartbeat {i} ok"
        for i in range(500)
    ])
    pytest = "\n".join(
        [
            "======================================== test session starts ========================================",
            "collected 100 items",
            *[f"tests/test_{i}.py::test_case_{i} PASSED" for i in range(95)],
            "tests/test_fail.py::test_case_fail FAILED",
            "",
            "======================================== FAILURES ========================================",
            "AssertionError: expected True, got False",
            "FAILED tests/test_fail.py::test_case_fail",
            "1 failed, 95 passed",
        ]
    )
    cases = []
    for workload, command, text in [
        ("logs", "tail -f app.log", logs),
        ("pytest", "pytest", pytest),
    ]:
        compressed = bash_compress.compress(command, text, 0, "")
        cases.append(
            case(
                "token-optimizer",
                workload,
                "context",
                "measured",
                token_estimate(text),
                token_estimate(compressed),
                f"token-optimizer@{TOKEN_REF} bash_compress.py",
            )
        )

    records = []
    for _ in range(5):
        records.append({"type": "assistant", "message": {"content": [{"type": "tool_use", "name": "Bash", "input": {"cmd": "npm test"}}]}})
        records.append({"type": "tool_result", "content": "failed with error"})
    with tempfile.NamedTemporaryFile("w", suffix=".jsonl", delete=False) as handle:
        for record in records:
            handle.write(json.dumps(record) + "\n")
        jsonl_path = handle.name
    findings = detect_retry_churn({"jsonl_path": jsonl_path})
    os.unlink(jsonl_path)
    detected = bool(findings)
    cases.append(
        case(
            "token-optimizer",
            "retry-churn",
            "session",
            "measured",
            token_estimate(json.dumps(records)),
            token_estimate(json.dumps(records)),
            f"token-optimizer@{TOKEN_REF} retry_churn.py detect-only",
            100 if detected else 0,
        )
    )
    return cases


def streetman_compress(text: str, mode: str, domain: str) -> str:
    proc = subprocess.run(
        [
            "cargo",
            "run",
            "--quiet",
            "--bin",
            "streetman",
            "--",
            "compress",
            "--mode",
            mode,
            "--domain",
            domain,
            "--json",
            "--no-archive",
        ],
        cwd=ROOT,
        input=text,
        text=True,
        capture_output=True,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.strip() or proc.stdout.strip())
    return json.loads(proc.stdout)["compressed"]


def capture_caveman(repo: Path) -> list[dict[str, object]]:
    snapshot_path = repo / "evals" / "snapshots" / "results.json"
    snapshot = json.loads(snapshot_path.read_text())
    baseline = snapshot["arms"]["__baseline__"]
    caveman = snapshot["arms"]["caveman"]
    streetman = [streetman_compress(text, "ultra", "prose") for text in baseline]

    before = sum(token_estimate(text) for text in baseline)
    caveman_after = sum(token_estimate(text) for text in caveman)
    streetman_after = sum(token_estimate(text) for text in streetman)
    source = f"caveman@{CAVEMAN_REF} evals/snapshots/results.json"
    return [
        case(
            "caveman",
            "caveman-eval-output",
            "output",
            "measured",
            before,
            caveman_after,
            source,
        ),
        case(
            "streetman",
            "caveman-eval-output",
            "output",
            "measured",
            before,
            streetman_after,
            f"streetman ultra over {source}",
        ),
        case(
            "caveman",
            "caveman-compress-memory",
            "context",
            "measured",
            898,
            481,
            f"caveman@{CAVEMAN_REF} README caveman-compress receipts",
        ),
    ]


def published_top_baselines() -> list[dict[str, object]]:
    return [
        case(
            "llmlingua",
            "published-prompt-compression",
            "context",
            "published-claim",
            1000,
            50,
            "microsoft/LLMLingua README/arXiv: up to 20x prompt compression",
        ),
        case(
            "leanctx",
            "published-coding-context",
            "context",
            "published-claim",
            1000,
            100,
            "leanctx.com: 60-90% fewer tokens per read, up to 99% on large ops",
        ),
    ]


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--out", default=str(ROOT / "benchmarks" / "results" / "competitor-live.json"))
    parser.add_argument("--cache-dir", default=str(Path.home() / ".cache" / "streetman" / "competitors"))
    args = parser.parse_args()

    cache = Path(args.cache_dir)
    cache.mkdir(parents=True, exist_ok=True)
    ensure_repo(cache, "headroom", HEADROOM_REPO, HEADROOM_REF)
    token_repo = ensure_repo(cache, "token-optimizer", TOKEN_REPO, TOKEN_REF)
    caveman_repo = ensure_repo(cache, "caveman", CAVEMAN_REPO, CAVEMAN_REF)
    headroom_python = ensure_headroom_venv(cache)

    cases = capture_headroom(headroom_python)
    cases.extend(capture_token_optimizer(token_repo))
    cases.extend(capture_caveman(caveman_repo))
    cases.extend(published_top_baselines())
    snapshot = {
        "snapshot_id": "competitor-live-2026-06-07",
        "captured_at": datetime.now(timezone.utc).isoformat(),
        "status": "measured-with-blocked-cases",
        "cases": cases,
    }
    out = Path(args.out)
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(snapshot, indent=2) + "\n")
    print(out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
