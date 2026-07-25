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

1. Recover the seed-safe pre-callback portion of maps instead of omitting the
   whole map after a runtime-sensitive room callback.
2. Decide whether createItems grass-flattening requires a pre-items public
   terrain projection.
3. Audit initial forced-item identities, remaining uncovered room/lifecycle
   loot, and the secret-room queue's main-path assumption.

Add regression tests proving constrained values cannot leak through serialized
reports, maps, or exact seed-finder matches. Fixed forced drops may remain exact
when pinned evidence proves every reported field is seed-only.

## Checkpoint

- SacrificeRoom keeps its exact generated reward internally, but public output
  reports only `weapon reward`, the derived stable tier, forced curse, source,
  and a conditional Parchment Scrap enchantment-chance note.
- Audited standard, special, crystal, and secret rooms keep sampled rewards
  internal. Public output uses static room contracts containing only invariant
  counts, categories, fixed identities, and explicitly conditional effects.
- Runtime-sensitive room callbacks set an exact item-tail boundary. Their heap
  cells are sanitized, and the whole public map is currently omitted where
  later terrain, mobs, or markers may diverge.
- The public forced-item queue is snapshotted before room callbacks, so moving a
  queued prize into a room does not change its public existence.
- Sacrificial heap items and concrete item marker labels are redacted publicly.
- Regular createItems heap, Mimic, and GoldenMimic facts remain internal only;
  their public item entries and map heap/mob/marker cells are omitted. Stable
  room-hosted Mimic cells remain visible while their rewards are omitted.
- Shop deck slots expose only proven category/tier and forced-property
  constraints. Bag and Hourglass stock are conditional; all FOR_SALE cells are
  hidden and public shop entries use canonical order.
- On the rare Artifact shop branch, artifact history can perturb the remaining
  floor RNG, so the public map and post-ShopRoom item/quest tail are omitted.
- Quest reward provenance keeps only proven fields public: Ghost weapon,
  Wandmaker wand, BlacksmithRoom weapon/missile, and Imp ring identities are
  constrained; Parchment-dependent properties are conditional.
- Safe quest type/target summaries remain visible without concrete reward
  titles. Persisted Wandmaker choices are internal-only on later floors.
- Wandmaker duplicate retries suppress its later item/map tail. Imp and
  BlacksmithRoom retain proven-stable structure and cells; constrained room
  heaps use generic labels and empty contents.
- Exact item searches ignore constrained predictions.
- Overall and item accuracy remain `partial`; do not claim full accuracy.
