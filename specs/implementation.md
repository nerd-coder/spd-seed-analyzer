# SPD Seed Analyzer — Implementation Plan

**Pinned SPD:** v3.3.8 @ `7b8b845a7`

**Accuracy:** `partial`; `specs/accuracy.json` is the authoritative coverage
manifest. Do not claim full seed-finder accuracy while any relevant area remains
partial.

## Product contract

The analyzer reports facts determined by the seed, including useful partial
facts. Its primary output is the item, loot, or reward guaranteed to be
generated and every property that remains fixed across valid player states.

When player state or choices affect generation, do not discard the entire
result. Enumerate the possible outcomes, attach the condition that selects each
outcome, and retain their shared deterministic properties. For example, if a
Sacrifice Room always produces a cursed `+2` weapon but its concrete class can
vary with Generator history, report `weapon`, `cursed`, and `+2`, then describe
the possible class outcomes or class range if they can be proven.

Exclude events whose occurrence or result is selected by runtime RNG rather
than seed-generation state, such as combat drops. A player-state-dependent
generation branch is not runtime RNG: model it as a conditional outcome when
its inputs and alternatives can be bounded from the pinned source.

The forced food created at the start of floor 1 is reported with its exact
identity: it is drawn before mobs are created or gameplay can mutate Generator
state. Later forced-food identities remain category constraints until relevant
gameplay-time Generator calls are modeled.

This contract must be identical in Rust reports, WASM, search matching, map
markers/heaps, and UI copy.

## Required report model

### Deterministic map profiles

The first conservative profile slice is implemented across `spd-core`, WASM,
and the web worker API. Public maps are painter-complete layout snapshots taken
before NPC, mob, and item population, so Guide Page/meta, heap, and mob state do
not gate layout rendering. The baseline profile explicitly assumes no Mossy
Clump or Trap Mechanism, resolving the default-feeling branch without changing
its two `Random.Float` calls. Legacy/unspecified analysis remains map-free.
Held trinket levels and other inputs that can change the painter/layout remain
future profile fields.

Replace omission-oriented projection with a fact/provenance model. Each
reportable source must produce:

- `source`: the generation source, such as Sacrifice Room, Ghost reward, shop
  slot, forced floor item, or Secret Library.
- `guarantee`: `always`, `conditional`, or `possible`. `always` means the fact
  survives every modeled player state and choice reaching that source.
- `facts`: independently reportable properties such as category, concrete
  identity, tier/range, upgrade level/range, curse state, enchantment/glyph,
  quantity, heap kind, and location.
- `conditions`: named state predicates that select an outcome, such as artifact
  availability, Parchment Scrap count, challenge state, inventory/bag state,
  trinket state, quest choice, or prior Generator/deck history.
- `outcomes`: the finite alternatives or a source-backed category/range when
  concrete alternatives cannot be enumerated safely.
- `evidence`: pinned Java/source reference and a Rust test or Java-oracle
  fixture proving each retained fact and branch boundary.
- `confidence`: `verified`, `partial`, or `unknown`; unknown facts are omitted,
  but verified siblings from the same item remain visible.

Do not represent uncertainty with a single generic description when individual
properties have different guarantees. Search uses the same fact model: exact
filters match verified exact facts, while category/range/property filters match
verified constraints and conditional outcomes only when requested explicitly.

## Ordered work

### P0 — Establish the contract in code and fixtures

1. Inventory every current public suppression/taint path in `spd-core`, WASM,
   search, and UI. Record each path in a table with: source, suppressed fields,
   state dependency, seed RNG dependency, and current evidence.
2. Introduce the report model above in `spd-core`; keep raw parity snapshots
   internal and derive public facts from explicit provenance. `spd-wasm` remains
   a serialization façade.
3. Add projection tests proving that one uncertain property does not erase
   independent deterministic properties. Required cases: Sacrifice reward,
   Ghost pair, Wandmaker choices, Imp reward, Blacksmith rewards, one shop
   persistent-deck slot, and one forced-item queue entry.
4. Update `specs/accuracy.json` terminology and coverage entries from the old
   omission/“seed-only” policy to deterministic facts plus conditional outcomes.

Acceptance:

- No public projection decision relies on a room-name denylist or a single
  floor-wide taint boolean when narrower field-level provenance is available.
- Altered-player-state tests compare outcomes and their intersection, and every
  retained shared fact has a failing regression if removed or changed.
- Existing exact Java-parity fixtures remain unchanged unless the pinned Java
  oracle proves the old fixture wrong.

### P1 — Make guaranteed loot and rewards useful

Complete these sources in priority order:

1. Sacrifice Room: prove the reward's existence, category, tier rule, upgrade
   rule, curse state, enchantment possibilities, and concrete weapon outcomes
   across reachable Generator histories.
2. Ghost, Wandmaker, Blacksmith, and Imp: publish common fixed properties plus
   explicit alternatives for Generator history, Parchment Scrap, quest choice,
   and player-triggered regeneration. Clearly separate floor-generation rewards
   from rewards generated later by player interaction.
3. Shops: report fixed stock, persistent-deck slot constraints, artifact/ring
   fallback outcomes as an `artifact or ring` entry, bag choices, and Hourglass
   sand conditions. Preserve shop stock when the rare callback suppresses the
   runtime-sensitive layout, and preserve slot facts when shuffle-dependent
   cells cannot be fixed.
4. Special and secret rooms: replace generic category/count summaries with the
   strongest proven identity set, quantity, property, heap, and placement facts
   for every supported room family. Start with Crystal Vault/Choice/Path,
   Secret Library, Crypt, Statue, Ring, Study, Ritual, Storage, and Armory.
5. Forced queues and regular `createItems`: report whether each item is
   guaranteed to spawn. Model Forbidden Runes, Large feeling, limited-drop schedules,
   Halls torches, Cached Rations, Dried Rose petals, Darkness torches, and
   guide/meta items individually.

Acceptance for each source:

- A pinned-source call graph identifies all RNG streams and mutable state read
  before the result is finalized.
- Tests cover the default state and every state variable that can change a
  reported field; where the state space is large, cover boundaries and prove
  the shared invariant from source structure.
- The report shows deterministic fields even when identity, placement, or
  another sibling field remains conditional or unknown.
- Runtime-RNG results such as combat drops never enter reports or searches.

### P2 — Stop losing unrelated facts after divergent branches

1. Replace persistent floor-tail taint with dependency-scoped provenance.
   Track which RNG stream and mutable subsystem a branch consumes: level RNG,
   Generator deck/category, artifact availability, quest state, inventory,
   trinket state, challenges, and meta progression.
2. After default-feeling trinket overrides, rare-artifact shop fallback,
   Wandmaker duplicate retries, and runtime-sensitive painter callbacks,
   continue reporting facts proven independent of the divergence.
3. For dependent facts, emit conditional branches or bounded outcomes instead
   of suppressing the entire layout/item/quest tail.

Implemented increment: inherited population/Generator taint is now separated
from the freshly seeded current-floor quest stream. A later independently
selected quest remains public, along with each reward's proven tier, upgrade,
curse, and category facts. Armor identity remains exact when its known tier has
one armor class. Current-floor divergences before quest selection still
suppress the dependent quest tail.

Wandmaker's initRooms-selected presence/type now also survives later same-floor
painter uncertainty; its two public wand contracts retain only their invariant
facts. Ambitious Imp rewards use the concise `+X ring` presentation while the
unsafe concrete ring class remains absent from the public projection.

Wandmaker reward choices now use the corresponding concise `+1…+3 wand`
presentation. The structured report retains the proven inclusive upgrade range
and uncursed state, while identity and history-warning UI remain absent.

Acceptance:

- Each of the four divergence classes above has paired state fixtures showing
  which downstream facts change and which remain stable.
- A divergent item branch cannot hide an unrelated pre-branch quest selection,
  forced schedule, or room fact.
- Conditional outcomes identify their state dependency in user-facing terms;
  they are never presented as the single concrete run outcome.

### P3 — Restore maps incrementally with per-cell facts

Do not wait for a whole floor to become exact. Give each terrain, heap, mob, and
marker fact provenance and finality through painter, `createMobs`,
`createItems`, meta/guide placement, and later cell rewrites.

1. Publish cells proven final across all modeled states.
2. Publish conditional cells as alternatives when the possible final states are
   bounded and useful.
3. Omit only the unresolved cell or entity property, not the entire map.
4. Add Toxic Gas vents/blob facts and remaining additive painter facts as their
   lifecycle becomes verified.

Acceptance:

- Map tests mutate relevant player state and assert stable cells, conditional
  cells, and omitted unknown cells separately.
- Every public marker/heap is backed by the same fact object used by the text
  report and search; there is no UI-only reconstruction.
- Real-seed maps are enabled floor-by-floor only after finality fixtures pass.

### P4 — Close generation coverage systematically

Build a coverage matrix for depths 1–24 and every regular builder, standard
room family, special room, secret room, quest branch, boss lifecycle carryover,
and supported level type. Each cell must be `verified`, `partial`, `unknown`, or
`out-of-scope`, with direct evidence.

Prioritize uncovered paths that can alter loot or downstream RNG:

1. Direct-drop lifecycle branches, Spyglass taint, and boss-floor Bones.
2. Remaining legacy room loot and painter callbacks.
3. Later-floor builder histories and room-selection decks.
4. VaultLevel and quest-branch levels, including MiningLevel, if brought into
   supported scope.

Acceptance:

- `specs/accuracy.json` mirrors the matrix without generic “other paths may
  diverge” placeholders; every remaining gap names a concrete branch or family.
- Each newly verified branch has phase-boundary parity evidence and at least one
  final public-report assertion.
- Overall status remains `partial` until every in-scope matrix cell and all
  cross-floor mutable histories are verified.

## Definition of done for every implementation slice

1. Port behavior from pinned SPD source and document version impact if a newer
   tree differs.
2. Add or extend `spd-core` unit tests plus Java-oracle/state-variation fixtures.
3. Update `specs/accuracy.json` in the same change.
4. Rebuild WASM after Rust changes and verify report, search, map, and UI use the
   same projection.
5. Update this plan's completed/next work when priorities or behavior change.
6. Run CI parity before handoff: `bun run check`, `bun run check:rust`,
   `bun run test:rust`, `bun run build`, and `bun run test:visual:only`.

## Immediate next slice

Checkpoint: runtime-sensitive regular-floor layouts now render by default as
clearly labeled per-floor baseline-assumption previews, while exact maps retain
their strict deterministic gate. Initial forced-queue reporting now describes the
invariant food-category existence directly, and even-schedule Scrolls of
Upgrade remain visible with a Forbidden Runes removal condition rendered as a
chip. Boss and LastLevel floor rows are no longer hidden by the UI, and shop
stock plus quest rewards render in provenance-specific sections instead of the
general item list.

Next map coverage slice: port the pinned dedicated boss generators for depths
5, 10, 15, 20, and 25 plus LastLevel depth 26. These cannot reuse the regular
room builder: each has level-specific terrain, transitions, painter RNG, and
boss lifecycle state that must be fixture-checked before its map is public.

Checkpoint: Sacrifice Room now exposes its verified one-higher floor-set tier
range and final `+0..+3` upgrade range through Rust, WASM, and the UI while
keeping concrete identity and enchantment conditional. The remaining work below
is the richer condition/outcome model and altered-history outcome enumeration.

Implement P0 using Sacrifice Room as the vertical reference:

1. Trace the pinned Java call path and enumerate all mutable inputs affecting
   weapon class, tier, upgrade level, curse, and enchantment.
2. Add at least two altered-history fixtures that produce different concrete
   weapon outcomes from the same seed.
3. Encode the intersection (`weapon`, guaranteed curse state, exact/ranged
   upgrade and tier facts proved by the trace) plus conditional concrete
   outcomes in the new report model.
4. Expose that model through WASM, render it in the report UI, and make search
   support its exact and constrained properties without treating a conditional
   identity as guaranteed.
5. Use the completed slice to finalize the inventory table and migrate the
   remaining P1 sources one at a time.
