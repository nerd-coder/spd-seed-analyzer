# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Generate a pinned AAA-AAA-AAA floor-19 final-facts fixture. Compare room classes,
normalized bounds, and the pre-paint RNG boundary first; fix only the first
semantic divergence. Update the accuracy manifest, run the full CI `check`
sequence, save this handoff, commit, and stop.

## Checkpoint

- Floor 18 matches every recorded pinned-oracle fact, including exact 1,564-cell
  terrain, discoverability, and tile-variance arrays.
- Overall accuracy remains `partial`; coverage is fixture-specific and stops at
  floor 18.
