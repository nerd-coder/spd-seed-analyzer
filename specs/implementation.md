# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Capture the pinned Java oracle fixture for preserved-run AAA-AAA-AAA floor 14.
Compare room classes and normalized bounds first; fix only the first semantic
difference, then stop.

## Current checkpoint

- Floor 13 matches Java's room classes, normalized bounds, RNG boundaries,
  final mobs, and normalized final heaps.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
