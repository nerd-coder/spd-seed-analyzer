# SPD Seed Analyzer — Handoff

**Updated:** 2026-07-25

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Status:** `partial` — `specs/accuracy.json` is the coverage source of truth.

## Next phase

Extend the preserved AAA-AAA-AAA replay to floor 12. Capture a pinned Java
oracle fixture, then compare room selection, pre-paint, pre-mobs, and pre-items
RNG boundaries in order. Fix only the first source-proven divergence before
asserting mobs and deterministic heaps.

## Current checkpoint

- Floor 11 now has exact deterministic heap parity, including ordinary Mimic
  reward lifecycle, forced drops, CrystalKeys, and GuidePage placement.
- The floor-11 shop bag class remains non-portable because pinned `ChooseBag`
  uses JVM identity-hash `HashMap` iteration; its other heap facts are pinned.
- VaultLevel branches, player inventory, Hourglass sandbags, and the unseeded
  early Guidebook page remain outside general exact prediction.

Run the complete CI `check` sequence from `AGENTS.md` before committing.
