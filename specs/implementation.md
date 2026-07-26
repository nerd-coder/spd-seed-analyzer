# SPD Seed Analyzer — Implementation Goal

Pinned target: SPD v3.3.8 @ `7b8b845a7`.

Accuracy remains `partial`. `specs/accuracy.json` is the authoritative record
of verified coverage, evidence, and known gaps. Never claim full seed-finder
accuracy while that manifest remains partial.

## Product goal

The analyzer has two public outputs:

1. Seed-determined item, loot, shop-stock, and quest-reward **spawn presence**.
2. Seed-determined **structural floor layout** for regular, boss, and final
   floors.

Report an item or reward only when the pinned game is guaranteed to create it
for the seed and stated conditions. Preserve independently proven properties
when concrete identity depends on player state: for example, report a cursed
`+2` weapon even if its class is not seed-determined. Represent bounded
alternatives and their conditions. Exclude combat drops, player-triggered
rewards, and other runtime-RNG results.

Public maps are layout-only. They may show room geometry, walls, traversable
floor, doors, and level entrances/exits needed to understand connectivity.
They must not show or claim parity for mobs, NPCs, items, heaps, grass, water
decoration, traps, plants, blobs, tile variance, or other painter decoration
and population. Those facts may remain internal only when needed to preserve
SPD RNG call order or provide parity evidence; they must not gate publication
of an otherwise deterministic structural layout.

Spawn presence and structural layout are separate projections. Uncertainty in
entity placement must not suppress layout, and layout uncertainty must not
erase independently proven item/reward presence.

Keep Rust reports, WASM, search evidence, map projection, and UI consistent.
Port pinned Java behavior and RNG call order; generation logic belongs in
`spd-core`.

## Current boundary

Regular-level replay and painter coverage are fixture-backed on selected paths
through floor 21, but coverage remains fixture-specific. Existing terrain,
trap, plant, blob, mob, heap, and placement fixtures are internal parity
evidence, not part of the public layout contract.

Deterministic limited drops, supported special-room loot, quest rewards, and
shop stock have partial public projections. Shop placement is intentionally
excluded: inventory state can alter the shuffle and cells without changing
every independently provable stock fact.

Dedicated boss floors at depths 5, 10, 15, 20, and 25 and LastLevel at depth
26 are listed but do not yet have generated structural layouts. Shop support
is partial and can suppress unrelated facts on artifact-fallback paths.

## Next phase

Complete the missing boss/final-floor layout projection and make deterministic
shop stock a supported spawn-presence path.

1. Port the pinned level builders and structural painters for depths 5, 10,
   15, 20, 25, and 26 into `spd-core`, preserving exact RNG call order.
2. Add a dedicated structural-layout projection containing only geometry,
   doors, and entrances/exits. Do not export grass, traps, decoration, mobs,
   items, heaps, or their cells.
3. Add pinned Java-oracle fixtures and focused Rust tests for each boss/final
   floor's dimensions, structural terrain, transitions, and RNG boundaries.
4. Audit `ShopRoom` generation and publish every stock identity or constrained
   alternative guaranteed to spawn. Keep stock order and placement private
   when inventory, Hourglass, bag, artifact, or JVM iteration state can alter
   them.
5. Decouple shop uncertainty from structural layout and from independently
   selected pre-shop or invariant post-shop spawn facts. Surface explicit
   conditions instead of dropping the whole floor whenever a narrower fact is
   still provable.
6. Keep exact seed-finder matching limited to exact spawn identities and
   properties; constrained alternatives remain report evidence, not exact
   matches.
7. Update `specs/accuracy.json` with each newly verified behavior in the same
   change. Do not mark boss layouts or full shop coverage implemented until
   oracle-backed tests pass.

Quest-branch levels such as MiningLevel remain a later phase unless required
to prove a deterministic reward spawn.

## Phase completion checklist

- Add or extend `spd-core` tests and pinned Java-oracle evidence.
- Rebuild WASM after Rust changes.
- Run CI parity: `bun run check`, `bun run check:rust`, `bun run test:rust`,
  `bun run build`, and `bun run test:visual:only`.
- Verify public maps contain no mob, item, heap, grass, trap, plant, blob, or
  decorative tile claims.
- Keep `specs/accuracy.json` aligned with the proven boundary.
- Rewrite this file to the next concise handoff and use a Conventional Commit
  message.
