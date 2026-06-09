# Streetman Quickstart

Build and run locally:

```bash
cargo run --bin streetman -- compress --mode full --domain prose <<'EOF'
The reason your React component is re-rendering is likely because it receives a new object reference every render.
EOF
```

Run the fixture bench:

```bash
cargo run --bin streetman -- bench run --suite absolute-win
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

The proxy currently exposes health and placeholder compression routes. Full
OpenAI/Anthropic forwarding is not implemented yet, but local compression is
available at `POST /v1/compress`:

```bash
curl -s http://127.0.0.1:8787/v1/compress \
  -H 'content-type: application/json' \
  -d '{"text":"The database configuration should be checked before deployment.","mode":"full","domain":"prose"}'
```

Expose MCP-compatible tool metadata:

```bash
cargo run --bin streetman -- mcp tools
```
