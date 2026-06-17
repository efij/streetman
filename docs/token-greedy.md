# Token-Greedy Compression

Streetman now optimizes against real tokenizer tokens, not characters.

Implemented fixes:

- capability-1 Token-Greedy Encoder: word and phrase candidates are accepted only when
  they reduce actual `tiktoken` tokens for the active model profile.
- capability-2 Never Worse Than Raw: after accuracy checks, the final candidate is
  reverted if `tokens(compressed) > tokens(original)`.

Default model profile is `gpt-4o` / `o200k_base`. Override with:

```bash
STREETMAN_MODEL=gpt-4o streetman compress --mode full --domain prose --json
```

Regression proof:

```bash
streetman bench run --suite token-greedy
```

The committed trap shows the old char-greedy form:

```text
creating dependencies configuration -> crtng dpndncs cnfgrtn
```

as `3 -> 10` real tokens, so Streetman now keeps the raw span instead.
