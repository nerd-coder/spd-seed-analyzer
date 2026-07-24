# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Compare AAA-AAA-AAA floor 12's pre-items RNG probe and normalized heaps with the
pinned oracle. Fix only the first post-mob or item-lifecycle semantic difference,
then stop. Do not claim full floor-12 lifecycle parity until both match.

## Current checkpoint

- Floor 12 matches Java's room classes, normalized bounds, pre-paint boundary,
  and pre-mobs boundary.
- SecretRunestoneRoom now paints its terrain and performs Java's exact two
  generated-stone drops plus fixed StoneOfEnchantment drop.
- Pre-items and final heap parity are not yet claimed for floor 12.
- Floor 11 remains exact at its documented deterministic boundaries.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
