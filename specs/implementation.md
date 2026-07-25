# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Extend the AAA-AAA-AAA floor-18 preserved-state comparison from pre-mobs to the
pre-items RNG boundary. Fix only the first semantic difference, update the
accuracy manifest, then stop.

## Current checkpoint

- Floor 18 matches Java through the pre-mobs RNG boundary.
- SecretArtilleryRoom now matches its fixed bomb-plus-two-missiles painter.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
