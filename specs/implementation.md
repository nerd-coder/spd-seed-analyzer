# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Extend the AAA-AAA-AAA floor-18 preserved-state comparison from pre-paint to the
pre-mobs RNG boundary. Fix only the first semantic difference, update the
accuracy manifest, then stop.

## Current checkpoint

- Floor 18 matches Java for room classes, normalized bounds, and pre-paint RNG.
- Later floor-18 lifecycle boundaries remain unverified.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
