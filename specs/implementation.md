# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is authoritative.

## Next phase

Continue items-and-loot parity. Reproduce the reported HKT-JZN-XQQ floor-13
cursed Corrupting Whip +2 by modeling the missing persistent player/generator
history; Parchment Scrap +3 added at floor entry is now pinned and produces a
cursed Corrupting Sword +2. Keep explicit state contracts and exact RNG/deck
boundaries in Java-oracle fixtures. Do not special-case the reported identity.

After the Whip history is explained, close remaining item gaps fixture-first:

1. Other SacrificeRoom player-state histories and reward lifecycle.
2. Every special/secret-room loot family and placement lifecycle.
3. Shops, crystal rooms, quest rewards, forced drops, and generator decks.
4. Cross-floor persistence and inventory-dependent item choices.
5. Exhaustive seed/floor/state parity for identity, quantity, level, curse,
   enchantment, heap type, cell, and RNG boundaries.

Keep layout work paused until `items-and-loot` has exhaustive pinned evidence,
no known in-scope gaps, and can be marked `complete`.

## Checkpoint

- The floor-13 reward is created during `SacrificeRoom.paint`, stored in
  `SacrificialFire`, and only dropped by the later sacrifice interaction.
- The explicit floor-entry Parchment Scrap +3 contract matches pinned Java:
  cursed Corrupting Sword +2. The reported Whip needs earlier run history.
- Overall and item accuracy remain `partial`; do not claim full accuracy.
