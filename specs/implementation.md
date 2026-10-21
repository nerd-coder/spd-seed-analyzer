# Implementation status

The analyzer renders the Troll Blacksmith's Crystal or Gnoll mine as the
once-through painter-complete map for the seed. Branch maps appear inside the
origin floor's quest panel, with reciprocal transitions, access conditions,
objective-specific tilesets, and painter-complete layout. Rust generation
matches pinned Java painter fixtures exactly; browser snapshots cover both
objectives. Entry still requires accepting the quest, carrying the Pickaxe,
and confirming travel.

The overall analyzer remains partial.

Main-path floor maps publish heaps, mobs, and loot markers with an inspect
panel (hover/tap). `uncertain_cells` marks "May differ" entities: everything
from depth 2 on, because run-history taint propagates from depth 1; depth 1 of
a fresh run is exact. `hidden_cells` (secret-door-only areas) pulse on the
canvas. Boss, vault, and mining maps remain layout-only.

The Ambitious Imp quest panel reports each of the six take-out slots with its
seed-independent guarantees and a candidate distribution over a +-3 deck-index
window, seed draw first. The window is exact rather than sampled: a
category's draw order is fixed by the seed, so run history only shifts which
index the Imp lands on. Percentages are the share of that enumerated window, a
stability measure, not an observed player frequency.

Upgrades and enchants are stronger than the classes: they are pinned by the
seed, because every class in a category costs the same ambient RNG. The panel
reports each slot's concrete `+N`, its enchant by name, and the pool's total
Scroll of Upgrade worth. `upgrades_pinned` goes false only when an
`UnstableSpellbook` draw is reachable in slot 0, which shifts every later roll.

The Old Wandmaker quest panel reports two ordered options, wand1 then wand2,
each over a +-3 deck-index window around the seed-drawn wand1 index, seed
draw first. The class at each index is a pure function of the seed, so run history
only shifts which index the reward lands on. Percentages are the share of that
enumerated window, a stability measure, not an observed player frequency.
wand1's +N is pinned across the window. wand2's +N is the seed roll and stays
pinned only when every offset in the window rejects the same number of
duplicate draws.

The finder catalogue includes the weapon, missile, and armor classes those
quests can roll. Its upgrade filter reaches +5 for that equipment. Rings and
wands still stop at +4. The Deep search switch fills the seed-only Imp and Wandmaker entries with
the same class window so those quest rewards can be matched at all; it is off
by default.

The Trinket Catalyst picker sits on the seed's selection floor. Picking an
offer calls `analyze_seed_with_trinket` (WASM export of the same name): the
trinket is held at +3 from `first_effective_depth`, variable samples are
published, and only Forbidden Runes stays a spawn condition. No pick is the
merged `analyze_seed` report. Per-option `catalyst_option_effects` marks which
trinkets the replay models (Mossy Clump, Trap Mechanism, Mimic Tooth, Rat
Skull, Exotic Crystals); Parchment Scrap's equipment enchant rolls and Cracked
Spyglass's extra loot are not modeled. The pick persists per seed in
localStorage. Mimic carried items are on the floor list and the map inspect
panel: the converted heap item, the generated prize, and the Mimic Tooth
extra when one is held. Ebony mimics (Mimic Tooth only) carry the prize,
the tooth extra, and one more default item, each uncursed and at least +1.
Crystal mimics keep the uncursed vault prize and lose it if they escape.
On floor 2 an ebony mimic's cell can depend on the unseeded entrance guide
page, and is then left off the map. Java oracle fixtures cover Mimic Tooth
+3 floors (`*-mimic-tooth-3-before-*.json`).

Every public object is a numeric id from `crates/spd-objects` (items,
enchantments, mobs, rooms, traps, plants, blobs). `bun run gen:ids` writes
`web/src/generated/game-ids.ts`; a Rust test fails when it drifts. Id lists are
append-only. The UI composes titles from ids, levels, curses, and enchantments.
Items whose concrete result depends on gameplay carry `prediction: "variable"`
and keep their seed-drawn value.

The finder runs an item-only pass (`search_seed_floors`): no maps, branches,
probes, or report projection; it stops at the deepest constraint and, in
Match all mode, as soon as a closed depth window can no longer match. A parity
test pins it to the full seed-only analysis.

`spd_core::analyze_seed_exact` (CLI: `bun run analyze SEED [FLOORS] [--maps]
[--pretty] [--full]`, crate `spd-cli`) is a concise exact projection under one
declared default profile: no challenges, no trinkets, no items acquired outside
level generation, default inventory. It reads the engine's internal per-floor
items, so it has no `constrained`/`variable` entries; quest choose-one pools sit
under `quests[].rewards`, and maps are RLE terrain plus traps, plants, and
transitions. Tests pin that every public exact item appears in it. The public
`SeedReport`, WASM, and web UI are unchanged.

Default-profile floor lists pair blank room contracts with concrete Variable
samples (RingRoom, Sacrifice, CrystalVault, labs, shops, …) so the UI can show
the item instead of "?". Remaining blank stubs are branch-miss contracts
(possible pool equipment, gold fallback, unused catalyst slot, SecretGarden)
with no sample on this profile; the UI hides those.

Next:

1. Decide whether enchanted item titles should use the game's `%s` templates
   ("Friendly Sickle") instead of a separate enchantment tag.
2. Public `SecretLaboratoryRoom` contract claims exactly 10 Energy Crystals;
   Java drops two `IntRange(3, 5)` stacks (the exact report has the real sum).
3. Consider exposing `analyze_seed_exact` through WASM if the web app should
   offer the default-profile view.
4. Narrow map `uncertain_cells` per entity (most later-floor mobs and heaps do
   not actually depend on run history); publish entities on boss, vault, and
   mining maps.
5. Performance (`bun run profile`): a 26-floor report is ~24 ms native,
   ~38 ms WASM. Remaining cost is spread thin; `patch::generate` (~25%) is
   now mostly the parity-bound `next_float` fill draws. Measured but not
   adopted: talc allocator (no WASM gain), `+simd128` (~7% WASM, drops
   pre-16.4 Safari), and sharing the pre-trinket floor prefix across
   trinket routes (~1–2 ms; routes split on ghost-reward profiling and need
   trinket/RNG thread-local snapshots).
