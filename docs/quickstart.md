# Streetman Quickstart

Install the current source-backed binary:

```bash
cargo install --git https://github.com/efij/streetman streetman-cli --bin streetman --locked
```

The Git install path tracks the latest pushed source release. Current source
release: `4.0.0`.

Build and run locally:

```bash
cargo run --bin streetman -- compress --mode full --domain prose <<'EOF'
The reason your React component is re-rendering is likely because it receives a new object reference every render.
EOF
```

Run the fixture bench:

```bash
cargo run --bin streetman -- bench run --suite absolute-win
cargo run --bin streetman -- bench run --suite token-greedy
cargo run --bin streetman -- bench run --suite final-case
cargo run --bin streetman -- bench run --suite all-lanes
cargo run --bin streetman -- bench run --suite absolute-win-2
cargo run --bin streetman -- bench run --suite absolute-win-3
cargo run --bin streetman -- bench run --suite absolute-win-4
```

Use the code-transport and security surfaces:

```bash
cargo run --bin streetman -- code diff --before old.rs --after new.rs --json
cargo run --bin streetman -- code elide src/lib.rs --keep 3 --json
cargo run --bin streetman -- code behavior-gate --before "cargo test" --after "cargo test" --json
cargo run --bin streetman -- security attest --json
cargo run --bin streetman -- policy protect --config .streetman.toml
cargo run --bin streetman -- policy verify --config .streetman.toml
cargo run --bin streetman -- policy push --config .streetman.toml --registry .streetman-policy-registry
cargo run --bin streetman -- enterprise init-config --out .streetman.toml --protect --push-registry .streetman-policy-registry --force
cargo run --bin streetman -- enterprise report --json
cargo run --bin streetman -- daemon --port 24846
```

Compare against the tracked competitor bars:

```bash
cargo run --bin streetman -- bench compare --against headroom,token-optimizer,caveman
```

Generate the Intel Dashboard:

```bash
printf 'retry failed\n%.0s' {1..40} \
  | cargo run --bin streetman -- audit dashboard \
      --out benchmarks/results/intel-dashboard.html
```

Check install health:

```bash
cargo run --bin streetman -- audit doctor
```

Start the proxy scaffold:

```bash
cargo run --bin streetman -- proxy --port 8787 --provider auto
curl http://127.0.0.1:8787/health
```

The proxy exposes health, local compression, and OpenAI-compatible forwarding
when `STREETMAN_UPSTREAM_URL` is set. Local compression is available at
`POST /v1/compress`:

```bash
curl -s http://127.0.0.1:8787/v1/compress \
  -H 'content-type: application/json' \
  -d '{"text":"The database configuration should be checked before deployment.","mode":"full","domain":"prose"}'
```

Expose MCP-compatible tool metadata:

```bash
cargo run --bin streetman -- mcp tools
```

Package-manager installers and editor plugins are not published yet. Today the
verified path is the Git source install above or `cargo run` from the repository.
