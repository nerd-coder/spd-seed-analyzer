# SPD Seed Analyzer — Handoff

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is authoritative.

## Goal

Public reports and search results are seed-only. Never expose a concrete item
field that player runtime/history can change. Keep exact values internally for
pinned parity, but publicly emit only proven invariant constraints and clearly
labeled conditional player-state effects. Apply this rule equally to item
lists, map heaps/markers, WASM, search evidence, and UI.

## Next phase

Continue the fixture-first seed-only audit:

1. Shop stock, especially inventory-dependent bag selection.
2. Ghost, Wandmaker, Blacksmith, and Imp reward identities.
3. Crypt, Armory, Pool, Statue, Sentry, Traps, crystal, and secret-room loot.
4. Quantity, level, curse, enchantment, heap type, and cell independently;
   redact only fields that are not seed-invariant.
5. Decide whether createItems grass-flattening requires a pre-items public
   terrain projection; its heap/mob/marker cells are already omitted.

Add regression tests proving constrained values cannot leak through serialized
reports, maps, or exact seed-finder matches. Fixed forced drops may remain exact
when pinned evidence proves every reported field is seed-only.

## Checkpoint

- SacrificeRoom keeps its exact generated reward internally, but public output
  reports only `weapon reward`, the derived stable tier, forced curse, source,
  and a conditional Parchment Scrap enchantment-chance note.
- Sacrificial heap items and concrete item marker labels are redacted publicly.
- Regular createItems heap, Mimic, and GoldenMimic facts remain internal only;
  their public item entries and map heap/mob/marker cells are omitted. Stable
  room-hosted Mimic cells remain visible while their rewards are omitted.
- Exact item searches ignore constrained predictions.
- Overall and item accuracy remain `partial`; do not claim full accuracy.
