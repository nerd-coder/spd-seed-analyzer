# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Extend the pinned Java final-heaps oracle through floor 16, preserving the
floor-15 boss lifecycle. Compare AAA-AAA-AAA floor 16 room classes and
normalized bounds first; fix only the first semantic difference, then stop.

## Current checkpoint

- AAA-AAA-AAA floor 14 matches Java through normalized final heaps.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
