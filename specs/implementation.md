# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Reproduce the user-observed HKT-JZN-XQQ floor-13 SacrificeRoom reward recorded
in `specs/observed-outcomes.json`. Model the exact Parchment Scrap +3 player
state in a pinned Java probe and determine whether the cursed Corrupting Whip
+2 is created during level generation or by the later sacrifice interaction.
Add state-aware parity only from matched oracle evidence; do not replace the
deterministic fresh-run fixture with a playthrough-dependent result.

## Current checkpoint

- AAA-AAA-AAA floor 21 now matches all three FigureEight attempt boundaries,
  exact room classes and bounds, six TunnelRooms, and pre-paint RNG.
- The source-backed fix restores ChasmExitRoom's pinned `[2, 1, 0]` size
  probabilities; the Java attempt trace is retained as regression evidence.
- Human playthrough reports live in `specs/observed-outcomes.json` and remain
  unverified until reproduced against the pinned game.
- Overall accuracy remains `partial`; continue fixture-first through later
  floor phases after resolving the observed state-dependent drift.
