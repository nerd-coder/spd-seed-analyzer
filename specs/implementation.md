# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Compare AAA-AAA-AAA floor-18 blob facts with the preserved Java fixture.
Fix only the first semantic difference, update the accuracy manifest, then stop.

## Current checkpoint

- Floor 18 matches Java through transitions, traps, and its GardenRoom Sungrass.
- Blob facts and any later additive facts remain unverified.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
