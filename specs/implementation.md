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

1. Ghost, Wandmaker, Blacksmith, and Imp reward identities.
2. Crypt, Armory, Pool, Statue, Sentry, Traps, crystal, and secret-room loot.
3. Quantity, level, curse, enchantment, heap type, and cell independently;
   redact only fields that are not seed-invariant.
4. Decide whether createItems grass-flattening requires a pre-items public
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
- Shop deck slots expose only proven category/tier and forced-property
  constraints. Bag and Hourglass stock are conditional; all FOR_SALE cells are
  hidden and public shop entries use canonical order.
- On the rare Artifact shop branch, artifact history can perturb the remaining
  floor RNG, so the public map and post-ShopRoom item/quest tail are omitted.
- Exact item searches ignore constrained predictions.
- Overall and item accuracy remain `partial`; do not claim full accuracy.
