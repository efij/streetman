# PR

## What

<!-- 1-2 lines -->

## Why

<!-- Link issue, describe motivation -->

Closes #

## Bench impact

<!-- MUST fill for compression-rule changes -->

- Savings delta: __ % (before → after)
- Accuracy: __ / 100
- Snapshot file: `benchmarks/results/bench-YYYYMMDD.json`

## Checklist

- [ ] Tests added / updated
- [ ] Bidirectional test for any new lexicon entry
- [ ] Accuracy fixture for any new rule
- [ ] Local bench run clean: `streetman bench run --trials 1 --models claude-sonnet-4-6`
- [ ] No hand-edited numbers in README / CLAIMS (auto-rendered only)
- [ ] No predecessor references (clean-room rule)
- [ ] Docs updated if user-visible behavior changed

## Breaking changes

<!-- If yes, describe migration path -->

None / Yes (describe below):
