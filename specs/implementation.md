# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Items, loot, and rewards are the exclusive accuracy priority. Do not resume
level-layout parity work until the `items-and-loot` area in
`specs/accuracy.json` is backed by exhaustive pinned evidence, has no known
in-scope gaps, and can be honestly marked `complete`.

First, reproduce the HKT-JZN-XQQ floor-13 SacrificeRoom reward in
`specs/observed-outcomes.json`. Model Parchment Scrap +3 in a pinned Java probe
and determine whether the cursed Corrupting Whip +2 comes from generation or
the later sacrifice interaction. Preserve deterministic fresh-run fixtures
while adding explicit player-state contracts.

Then close the remaining item gaps fixture-first:

1. Sacrifice rewards and all player-state modifiers.
2. Every special/secret-room loot family and placement lifecycle.
3. Shops, crystal rooms, quest rewards, forced drops, and generator decks.
4. Cross-floor persistence, inventory-dependent choices, and supported item
   upgrade/curse/enchantment state.
5. An exhaustive seed/floor/state parity matrix with exact identities,
   quantities, levels, curses, enchantments, heap types, cells, and RNG
   boundaries.

Only after that matrix passes and `items-and-loot` is `complete` may the next
handoff schedule further layout work.

## Current checkpoint

- AAA-AAA-AAA floor 21 now matches all three FigureEight attempt boundaries,
  exact room classes and bounds, six TunnelRooms, and pre-paint RNG.
- The source-backed fix restores ChasmExitRoom's pinned `[2, 1, 0]` size
  probabilities; the Java attempt trace is retained as regression evidence.
- Human playthrough reports live in `specs/observed-outcomes.json` and remain
  unverified until reproduced against the pinned game.
- Overall accuracy remains `partial`; item accuracy is not yet 100%, so layout
  work is paused.
