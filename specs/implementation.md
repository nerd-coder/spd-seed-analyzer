# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Compare AAA-AAA-AAA floor 14 normalized final heaps with the pinned Java
fixture. Fix only the first semantic difference, then stop.

## Current checkpoint

- Floor 14 matches Java's room classes, normalized bounds, RNG boundaries, and
  final mobs. `SuspiciousChestRoom` now projects its Mimic at the exact cell.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
